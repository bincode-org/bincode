use std::io::{Read, Write};
use serde;

use config::{Options, OptionsExt};
use {ErrorKind, Result};

#[derive(Clone)]
struct CountSize<L: SizeLimit> {
    total: u64,
    other_limit: L,
}

/// Serializes an object directly into a `Writer`.
///
/// If the serialization would take more bytes than allowed by `size_limit`, an error
/// is returned and *no bytes* will be written into the `Writer`.
///
/// If this returns an `Error` (other than SizeLimit), assume that the
/// writer is in an invalid state, as writing could bail out in the middle of
/// serializing.
pub(crate) fn serialize_into<W, T: ?Sized, O>(writer: W, value: &T, mut options: O) -> Result<()>
where
    W: Write,
    T: serde::Serialize,
    O: Options,
{
    if options.limit().limit().is_some() {
        // "compute" the size for the side-effect
        // of returning Err if the bound was reached.
        serialized_size(value, &mut options)?;
    }

    let mut serializer = ::ser::Serializer::<_, O>::new(writer, options);
    serde::Serialize::serialize(value, &mut serializer)
}

/// Serializes a serializable object into a `Vec` of bytes.
///
/// If the serialization would take more bytes than allowed by `size_limit`,
/// an error is returned.
pub(crate) fn serialize<T: ?Sized, O>(value: &T, mut options: O) -> Result<Vec<u8>>
where
    T: serde::Serialize,
    O: Options,
{
    let mut writer = {
        let actual_size = serialized_size(value, &mut options)?;
        Vec::with_capacity(actual_size as usize)
    };

    serialize_into(&mut writer, value, options.with_no_limit())?;
    Ok(writer)
}

impl<L: SizeLimit> SizeLimit for CountSize<L> {
    fn add(&mut self, c: u64) -> Result<()> {
        self.other_limit.add(c)?;
        self.total += c;
        Ok(())
    }

    fn limit(&self) -> Option<u64> {
        unreachable!();
    }
}

/// Returns the size that an object would be if serialized using Bincode.
///
/// This is used internally as part of the check for encode_into, but it can
/// be useful for preallocating buffers if thats your style.
pub(crate) fn serialized_size<T: ?Sized, O: Options>(value: &T, mut options: O) -> Result<u64>
where
    T: serde::Serialize,
{
    let old_limiter = options.limit().clone();
    let mut size_counter = ::ser::SizeChecker {
        options: ::config::WithOtherLimit::new(
            options,
            CountSize {
                total: 0,
                other_limit: old_limiter,
            },
        ),
    };

    let result = value.serialize(&mut size_counter);
    result.map(|_| size_counter.options.new_limit.total)
}

/// Deserializes an object directly from a `Read`er.
///
/// If the provided `SizeLimit` is reached, the deserialization will bail immediately.
/// A SizeLimit can help prevent an attacker from flooding your server with
/// a neverending stream of values that runs your server out of memory.
///
/// If this returns an `Error`, assume that the buffer that you passed
/// in is in an invalid state, as the error could be returned during any point
/// in the reading.
pub(crate) fn deserialize_from<R, T, O>(reader: R, options: O) -> Result<T>
where
    R: Read,
    T: serde::de::DeserializeOwned,
    O: Options,
{
    let reader = ::de::read::IoReader::new(reader);
    let mut deserializer = ::de::Deserializer::<_, O>::new(reader, options);
    serde::Deserialize::deserialize(&mut deserializer)
}

/// Deserializes a slice of bytes into an object.
///
/// This method does not have a size-limit because if you already have the bytes
/// in memory, then you don't gain anything by having a limiter.
pub(crate) fn deserialize<'a, T, O>(bytes: &'a [u8], options: O) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
    O: Options,
{
    let reader = ::de::read::SliceReader::new(bytes);
    let options = ::config::WithOtherLimit::new(options, Infinite);
    let mut deserializer = ::de::Deserializer::new(reader, options);
    serde::Deserialize::deserialize(&mut deserializer)
}


/// A limit on the amount of bytes that can be read or written.
///
/// Size limits are an incredibly important part of both encoding and decoding.
///
/// In order to prevent DOS attacks on a decoder, it is important to limit the
/// amount of bytes that a single encoded message can be; otherwise, if you
/// are decoding bytes right off of a TCP stream for example, it would be
/// possible for an attacker to flood your server with a 3TB vec, causing the
/// decoder to run out of memory and crash your application!
/// Because of this, you can provide a maximum-number-of-bytes that can be read
/// during decoding, and the decoder will explicitly fail if it has to read
/// any more than that.
///
/// On the other side, you want to make sure that you aren't encoding a message
/// that is larger than your decoder expects.  By supplying a size limit to an
/// encoding function, the encoder will verify that the structure can be encoded
/// within that limit.  This verification occurs before any bytes are written to
/// the Writer, so recovering from an error is easy.
pub(crate) trait SizeLimit: Clone {
    /// Tells the SizeLimit that a certain number of bytes has been
    /// read or written.  Returns Err if the limit has been exceeded.
    fn add(&mut self, n: u64) -> Result<()>;
    /// Returns the hard limit (if one exists)
    fn limit(&self) -> Option<u64>;
}


/// A SizeLimit that restricts serialized or deserialized messages from
/// exceeding a certain byte length.
#[derive(Copy, Clone)]
pub struct Bounded(pub u64);

/// A SizeLimit without a limit!
/// Use this if you don't care about the size of encoded or decoded messages.
#[derive(Copy, Clone)]
pub struct Infinite;

impl SizeLimit for Bounded {
    #[inline(always)]
    fn add(&mut self, n: u64) -> Result<()> {
        if self.0 >= n {
            self.0 -= n;
            Ok(())
        } else {
            Err(Box::new(ErrorKind::SizeLimit))
        }
    }

    #[inline(always)]
    fn limit(&self) -> Option<u64> {
        Some(self.0)
    }
}

impl SizeLimit for Infinite {
    #[inline(always)]
    fn add(&mut self, _: u64) -> Result<()> {
        Ok(())
    }

    #[inline(always)]
    fn limit(&self) -> Option<u64> {
        None
    }
}
