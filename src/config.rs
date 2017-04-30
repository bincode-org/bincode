//! Configuration options for serialization and deserialization.

use byteorder::ByteOrder;
use std::marker::PhantomData;
use super::internal::Result;
use super::internal::ErrorKind;

/// The default configuration options for bincode.
///
/// Endianness: BigEndian - Big endian is the fastest option on most modern CPU architectures
/// SizeLimit: Infinite - This default is the least surprising
pub static DEFAULT_CONFIG: BasicConfig<::byteorder::LittleEndian, Infinite, private::H> = BasicConfig {
    limit: Infinite,
    _phantom_byte_order: PhantomData,
    _phantom_o: PhantomData,
};

mod private {
    #[derive(Copy, Clone)]
    pub struct H;
    pub trait Hidden {}
}

/// A trait that contains compile-time information about which options to
/// use while serializing and deserializing
pub trait Config: private::Hidden {
    /// Decides how numbers should be serialized
    type Endianness: ByteOrder;
    /// A type that implements SizeLimit
    type Limit: SizeLimit;

    /// Returns the SizeLimit object
    fn limit(&mut self) -> &mut Self::Limit;
}

/// The only implementation of Config.
#[derive(Copy, Clone)]
pub struct BasicConfig<E: ByteOrder, L: SizeLimit, O> {
    limit: L,
    _phantom_byte_order: PhantomData<E>,
    _phantom_o: PhantomData<O>,
}

impl <E: ByteOrder, L: SizeLimit, O> private::Hidden for BasicConfig<E, L, O> {}

impl <E: ByteOrder, L: SizeLimit, O> BasicConfig<E, L, O> {
    /// Produces a new configuration but with a big endian byte order
    pub fn with_big_endian(self) -> BasicConfig<::byteorder::BigEndian, L, O> {
        BasicConfig {
            limit: self.limit,
            _phantom_byte_order: PhantomData,
            _phantom_o: PhantomData,
        }
    }

    /// Produces a new configuration but with a little endian byte order
    pub fn with_little_endian(self) -> BasicConfig<::byteorder::LittleEndian, L, O> {
        BasicConfig {
            limit: self.limit,
            _phantom_byte_order: PhantomData,
            _phantom_o: PhantomData,
        }
    }

    /// Produces a new configuration but with a the byte order specified by the machine
    /// that the code is compiled for.
    pub fn with_native_endian(self) -> BasicConfig<::byteorder::NativeEndian, L, O> {
        BasicConfig {
            limit: self.limit,
            _phantom_byte_order: PhantomData,
            _phantom_o: PhantomData,
        }
    }

    /// Produces a new configuration but with a bounded size limit
    pub fn with_size_limit(self, limit: u64) -> BasicConfig<E, Bounded, O> {
        BasicConfig {
            limit: Bounded(limit),
            _phantom_byte_order: PhantomData,
            _phantom_o: PhantomData,
        }
    }

    /// Produces a new configuration but without a bounded size limit
    pub fn without_size_limit(self) -> BasicConfig<E, Infinite, O> {
        BasicConfig {
            limit: Infinite,
            _phantom_byte_order: PhantomData,
            _phantom_o: PhantomData,
        }
    }
}

impl <E: ByteOrder, L: SizeLimit, O> Config for BasicConfig<E, L, O> {
    type Endianness = E;
    type Limit = L;
    fn limit(&mut self) -> &mut Self::Limit {
        &mut self.limit
    }
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
pub trait SizeLimit {
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
    fn limit(&self) -> Option<u64> { Some(self.0) }
}

impl SizeLimit for Infinite {
    #[inline(always)]
    fn add(&mut self, _: u64) -> Result<()> { Ok (()) }

    #[inline(always)]
    fn limit(&self) -> Option<u64> { None }
}
