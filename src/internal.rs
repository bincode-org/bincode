use std::io::{Write, Read};
use serde;

use ::config::Options;
use ::{ErrorKind, Result};
use ::SizeLimit;

struct CountSize {
    total: u64,
    limit: Option<u64>,
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
    if let Some(limit) = options.limit().limit() {
        try!(serialized_size_bounded(value, limit).ok_or(
            ErrorKind::SizeLimit,
        ));
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
    let mut writer = match options.limit().limit() {
        Some(size_limit) => {
            let actual_size = try!(serialized_size_bounded(value, size_limit).ok_or(
                ErrorKind::SizeLimit,
            ));
            Vec::with_capacity(actual_size as usize)
        }
        None => {
            let size = serialized_size(value) as usize;
            Vec::with_capacity(size)
        }
    };

    try!(serialize_into::<_, _, O>(
        &mut writer,
        value,
        options
    ));
    Ok(writer)
}

impl ::SizeLimit for CountSize {
    fn add(&mut self, c: u64) -> Result<()> {
        self.total += c;
        if let Some(limit) = self.limit {
            if self.total > limit {
                return Err(Box::new(ErrorKind::SizeLimit));
            }
        }
        Ok(())
    }

    fn limit(&self) -> Option<u64> {
        unreachable!();
    }
}

pub(crate) fn serialized_size<T: ?Sized>(value: &T) -> u64 {
    unimplemented!();
}

pub(crate) fn serialized_size_bounded<T: ?Sized>(value: &T, b: u64) -> Option<u64> {
    unimplemented!();
}
/*
/// Returns the size that an object would be if serialized using Bincode.
///
/// This is used internally as part of the check for encode_into, but it can
/// be useful for preallocating buffers if thats your style.
pub(crate) fn serialized_size<T: ?Sized>(value: &T) -> u64
where
    T: serde::Serialize,
{
    let mut size_counter = ::ser::SizeChecker {
        options: CountSize {
            total: 0,
            limit: None,
        },
    };

    value.serialize(&mut size_counter).ok();
    size_counter.size_limit.total
}

/// Given a maximum size limit, check how large an object would be if it
/// were to be serialized.
///
/// If it can be serialized in `max` or fewer bytes, that number will be returned
/// inside `Some`.  If it goes over bounds, then None is returned.
pub(crate) fn serialized_size_bounded<T: ?Sized>(value: &T, max: u64) -> Option<u64>
where
    T: serde::Serialize,
{
    let mut size_counter = ::ser::SizeChecker {
        size_limit: CountSize {
            total: 0,
            limit: Some(max),
        },
    };

    match value.serialize(&mut size_counter) {
        Ok(_) => Some(size_counter.size_limit.total),
        Err(_) => None,
    }
}
*/

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
    O: Options
{
    let reader = ::de::read::SliceReader::new(bytes);
    let mut deserializer = ::de::Deserializer::<_, O>::new(reader, options);
    serde::Deserialize::deserialize(&mut deserializer)
}
