use byteorder;
use de::read::BincodeRead;
use error::{ErrorKind, Result};
use serde;
use std::io::{Read, Write};
use std::marker::PhantomData;

pub(crate) use self::internal::*;

use self::EndianOption::*;
use self::LimitOption::*;

/// The default options for bincode serialization/deserialization.
/// Implements OptionsExt to allow building configuration object for non-default settings.
///
/// ### Defaults
/// By default bincode will use little-endian encoding for mult-byte integers, and will not
/// limit the number of serialized/deserialized bytes.
#[derive(Copy, Clone)]
pub struct DefaultOptions(Infinite);

impl DefaultOptions {
    /// Get a default configuration object.
    ///
    /// ### Default Configuration:
    ///
    /// | Byte limit | Endianness |
    /// |------------|------------|
    /// | Unlimited  | Little     |
    pub fn new() -> DefaultOptions {
        DefaultOptions(Infinite)
    }
}

impl Default for DefaultOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Options for DefaultOptions {
    type Limit = Infinite;
    type Endian = LittleEndian;

    #[inline(always)]
    fn limit(&mut self) -> &mut Infinite {
        &mut self.0
    }
}

/// A configuration builder trait whose options Bincode will use
/// while serializing and deserializing.
///
/// ### Options
/// Endianness: The endianness with which multi-byte integers will be read/written.  *default: little endian*
///
/// Limit: The maximum number of bytes that will be read/written in a bincode serialize/deserialize. *default: unlimited*
///
/// ### Byte Limit Details
/// The purpose of byte-limiting is to prevent Denial-Of-Service attacks whereby malicious attackers get bincode
/// deserialization to crash your process by allocating too much memory or keeping a connection open for too long.
///
/// When a byte limit is set, bincode will return `Err` on any deserialization that goes over the limit, or any
/// serialization that goes over the limit.
/// Sets the byte limit to be unlimited.
/// This is the default.
pub trait OptionsExt: Options + Sized {
    /// Sets the byte limit to be unlimited.
    /// This is the default.
    fn with_no_limit(self) -> WithOtherLimit<Self, Infinite> {
        WithOtherLimit::new(self, Infinite)
    }

    /// Sets the byte limit to `limit`.
    fn with_limit(self, limit: u64) -> WithOtherLimit<Self, Bounded> {
        WithOtherLimit::new(self, Bounded(limit))
    }

    /// Sets the endianness to little-endian
    /// This is the default.
    fn with_little_endian(self) -> WithOtherEndian<Self, LittleEndian> {
        WithOtherEndian::new(self)
    }

    /// Sets the endianness to big-endian
    fn with_big_endian(self) -> WithOtherEndian<Self, BigEndian> {
        WithOtherEndian::new(self)
    }

    /// Sets the endianness to the the machine-native endianness
    fn with_native_endian(self) -> WithOtherEndian<Self, NativeEndian> {
        WithOtherEndian::new(self)
    }

    /// Serializes a serializable object into a `Vec` of bytes using this configuration
    #[inline(always)]
    fn serialize<S: ?Sized + serde::Serialize>(self, t: &S) -> Result<Vec<u8>> {
        ::internal::serialize(t, self)
    }

    /// Returns the size that an object would be if serialized using Bincode with this configuration
    #[inline(always)]
    fn serialized_size<T: ?Sized + serde::Serialize>(self, t: &T) -> Result<u64> {
        ::internal::serialized_size(t, self)
    }

    /// Serializes an object directly into a `Writer` using this configuration
    ///
    /// If the serialization would take more bytes than allowed by the size limit, an error
    /// is returned and *no bytes* will be written into the `Writer`
    #[inline(always)]
    fn serialize_into<W: Write, T: ?Sized + serde::Serialize>(self, w: W, t: &T) -> Result<()> {
        ::internal::serialize_into(w, t, self)
    }

    /// Deserializes a slice of bytes into an instance of `T` using this configuration
    #[inline(always)]
    fn deserialize<'a, T: serde::Deserialize<'a>>(self, bytes: &'a [u8]) -> Result<T> {
        ::internal::deserialize(bytes, self)
    }

    /// TODO: document
    #[doc(hidden)]
    #[inline(always)]
    fn deserialize_in_place<'a, R, T>(self, reader: R, place: &mut T) -> Result<()>
    where
        R: BincodeRead<'a>,
        T: serde::de::Deserialize<'a>,
    {
        ::internal::deserialize_in_place(reader, self, place)
    }

    /// Deserializes a slice of bytes with state `seed` using this configuration.
    #[inline(always)]
    fn deserialize_seed<'a, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        bytes: &'a [u8],
    ) -> Result<T::Value> {
        ::internal::deserialize_seed(seed, bytes, self)
    }

    /// Deserializes an object directly from a `Read`er using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from<R: Read, T: serde::de::DeserializeOwned>(self, reader: R) -> Result<T> {
        ::internal::deserialize_from(reader, self)
    }

    /// Deserializes an object directly from a `Read`er with state `seed` using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_seed<'a, R: Read, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        ::internal::deserialize_from_seed(seed, reader, self)
    }

    /// Deserializes an object from a custom `BincodeRead`er using the default configuration.
    /// It is highly recommended to use `deserialize_from` unless you need to implement
    /// `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_custom<'a, R: BincodeRead<'a>, T: serde::de::DeserializeOwned>(
        self,
        reader: R,
    ) -> Result<T> {
        ::internal::deserialize_from_custom(reader, self)
    }

    /// Deserializes an object from a custom `BincodeRead`er with state `seed` using the default
    /// configuration. It is highly recommended to use `deserialize_from` unless you need to
    /// implement `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_custom_seed<'a, R: BincodeRead<'a>, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        ::internal::deserialize_from_custom_seed(seed, reader, self)
    }
}

impl<T: Options> OptionsExt for T {}

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

/// Little-endian byte ordering.
#[derive(Copy, Clone)]
pub struct LittleEndian;

/// Big-endian byte ordering.
#[derive(Copy, Clone)]
pub struct BigEndian;

/// The native byte ordering of the current system.
#[derive(Copy, Clone)]
pub struct NativeEndian;

impl BincodeByteOrder for LittleEndian {
    type Endian = byteorder::LittleEndian;
}

impl BincodeByteOrder for BigEndian {
    type Endian = byteorder::BigEndian;
}

impl BincodeByteOrder for NativeEndian {
    type Endian = byteorder::NativeEndian;
}

#[derive(Clone, Copy, Debug)]
enum LimitOption {
    Unlimited,
    Limited(u64),
}

#[derive(Clone, Copy, Debug)]
enum EndianOption {
    Big,
    Little,
    Native,
}

/// A configuration builder whose options Bincode will use
/// while serializing and deserializing.
///
/// ### Options
/// Endianness: The endianness with which multi-byte integers will be read/written.  *default: little endian*
/// Limit: The maximum number of bytes that will be read/written in a bincode serialize/deserialize. *default: unlimited*
///
/// ### Byte Limit Details
/// The purpose of byte-limiting is to prevent Denial-Of-Service attacks whereby malicious attackers get bincode
/// deserialization to crash your process by allocating too much memory or keeping a connection open for too long.
///
/// When a byte limit is set, bincode will return `Err` on any deserialization that goes over the limit, or any
/// serialization that goes over the limit.
#[derive(Clone, Debug)]
#[deprecated(
    since = "1.3.0",
    note = "please use the `DefaultOptions`/`OptionsExt` system instead"
)]
pub struct Config {
    limit: LimitOption,
    endian: EndianOption,
}

/// A configuration struct with a user-specified byte limit
#[derive(Clone, Copy)]
pub struct WithOtherLimit<O: Options, L: SizeLimit> {
    _options: O,
    pub(crate) new_limit: L,
}

/// A configuration struct with a user-specified endian order
#[derive(Clone, Copy)]
pub struct WithOtherEndian<O: Options, E: BincodeByteOrder> {
    options: O,
    _endian: PhantomData<E>,
}

impl<O: Options, L: SizeLimit> WithOtherLimit<O, L> {
    #[inline(always)]
    pub(crate) fn new(options: O, limit: L) -> WithOtherLimit<O, L> {
        WithOtherLimit {
            _options: options,
            new_limit: limit,
        }
    }
}

impl<O: Options, E: BincodeByteOrder> WithOtherEndian<O, E> {
    #[inline(always)]
    pub(crate) fn new(options: O) -> WithOtherEndian<O, E> {
        WithOtherEndian {
            options,
            _endian: PhantomData,
        }
    }
}

impl<O: Options, E: BincodeByteOrder + 'static> Options for WithOtherEndian<O, E> {
    type Limit = O::Limit;
    type Endian = E;

    #[inline(always)]
    fn limit(&mut self) -> &mut O::Limit {
        self.options.limit()
    }
}

impl<O: Options, L: SizeLimit + 'static> Options for WithOtherLimit<O, L> {
    type Limit = L;
    type Endian = O::Endian;

    fn limit(&mut self) -> &mut L {
        &mut self.new_limit
    }
}

macro_rules! config_map {
    ($self:expr, $opts:ident => $call:expr) => {
        match ($self.limit, $self.endian) {
            (Unlimited, Little) => {
                let $opts = DefaultOptions::new().with_no_limit().with_little_endian();
                $call
            }
            (Unlimited, Big) => {
                let $opts = DefaultOptions::new().with_no_limit().with_big_endian();
                $call
            }
            (Unlimited, Native) => {
                let $opts = DefaultOptions::new().with_no_limit().with_native_endian();
                $call
            }

            (Limited(l), Little) => {
                let $opts = DefaultOptions::new().with_limit(l).with_little_endian();
                $call
            }
            (Limited(l), Big) => {
                let $opts = DefaultOptions::new().with_limit(l).with_big_endian();
                $call
            }
            (Limited(l), Native) => {
                let $opts = DefaultOptions::new().with_limit(l).with_native_endian();
                $call
            }
        }
    };
}

impl Config {
    #[inline(always)]
    pub(crate) fn new() -> Config {
        Config {
            limit: LimitOption::Unlimited,
            endian: EndianOption::Little,
        }
    }

    /// Sets the byte limit to be unlimited.
    /// This is the default.
    #[inline(always)]
    pub fn no_limit(&mut self) -> &mut Self {
        self.limit = LimitOption::Unlimited;
        self
    }

    /// Sets the byte limit to `limit`.
    #[inline(always)]
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = LimitOption::Limited(limit);
        self
    }

    /// Sets the endianness to little-endian
    /// This is the default.
    #[inline(always)]
    pub fn little_endian(&mut self) -> &mut Self {
        self.endian = EndianOption::Little;
        self
    }

    /// Sets the endianness to big-endian
    #[inline(always)]
    pub fn big_endian(&mut self) -> &mut Self {
        self.endian = EndianOption::Big;
        self
    }

    /// Sets the endianness to the the machine-native endianness
    #[inline(always)]
    pub fn native_endian(&mut self) -> &mut Self {
        self.endian = EndianOption::Native;
        self
    }

    /// Serializes a serializable object into a `Vec` of bytes using this configuration
    #[inline(always)]
    pub fn serialize<T: ?Sized + serde::Serialize>(&self, t: &T) -> Result<Vec<u8>> {
        config_map!(self, opts => ::internal::serialize(t, opts))
    }

    /// Returns the size that an object would be if serialized using Bincode with this configuration
    #[inline(always)]
    pub fn serialized_size<T: ?Sized + serde::Serialize>(&self, t: &T) -> Result<u64> {
        config_map!(self, opts => ::internal::serialized_size(t, opts))
    }

    /// Serializes an object directly into a `Writer` using this configuration
    ///
    /// If the serialization would take more bytes than allowed by the size limit, an error
    /// is returned and *no bytes* will be written into the `Writer`
    #[inline(always)]
    pub fn serialize_into<W: Write, T: ?Sized + serde::Serialize>(
        &self,
        w: W,
        t: &T,
    ) -> Result<()> {
        config_map!(self, opts => ::internal::serialize_into(w, t, opts))
    }

    /// Deserializes a slice of bytes into an instance of `T` using this configuration
    #[inline(always)]
    pub fn deserialize<'a, T: serde::Deserialize<'a>>(&self, bytes: &'a [u8]) -> Result<T> {
        config_map!(self, opts => ::internal::deserialize(bytes, opts))
    }

    /// TODO: document
    #[doc(hidden)]
    #[inline(always)]
    pub fn deserialize_in_place<'a, R, T>(&self, reader: R, place: &mut T) -> Result<()>
    where
        R: BincodeRead<'a>,
        T: serde::de::Deserialize<'a>,
    {
        config_map!(self, opts => ::internal::deserialize_in_place(reader, opts, place))
    }

    /// Deserializes a slice of bytes with state `seed` using this configuration.
    #[inline(always)]
    pub fn deserialize_seed<'a, T: serde::de::DeserializeSeed<'a>>(
        &self,
        seed: T,
        bytes: &'a [u8],
    ) -> Result<T::Value> {
        config_map!(self, opts => ::internal::deserialize_seed(seed, bytes, opts))
    }

    /// Deserializes an object directly from a `Read`er using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    pub fn deserialize_from<R: Read, T: serde::de::DeserializeOwned>(
        &self,
        reader: R,
    ) -> Result<T> {
        config_map!(self, opts => ::internal::deserialize_from(reader, opts))
    }

    /// Deserializes an object directly from a `Read`er with state `seed` using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    pub fn deserialize_from_seed<'a, R: Read, T: serde::de::DeserializeSeed<'a>>(
        &self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        config_map!(self, opts => ::internal::deserialize_from_seed(seed, reader, opts))
    }

    /// Deserializes an object from a custom `BincodeRead`er using the default configuration.
    /// It is highly recommended to use `deserialize_from` unless you need to implement
    /// `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    pub fn deserialize_from_custom<'a, R: BincodeRead<'a>, T: serde::de::DeserializeOwned>(
        &self,
        reader: R,
    ) -> Result<T> {
        config_map!(self, opts => ::internal::deserialize_from_custom(reader, opts))
    }

    /// Deserializes an object from a custom `BincodeRead`er with state `seed` using the default
    /// configuration. It is highly recommended to use `deserialize_from` unless you need to
    /// implement `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    pub fn deserialize_from_custom_seed<
        'a,
        R: BincodeRead<'a>,
        T: serde::de::DeserializeSeed<'a>,
    >(
        &self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        config_map!(self, opts => ::internal::deserialize_from_custom_seed(seed, reader, opts))
    }
}

mod internal {
    use super::*;
    use byteorder::ByteOrder;

    pub trait Options {
        type Limit: SizeLimit + 'static;
        type Endian: BincodeByteOrder + 'static;

        fn limit(&mut self) -> &mut Self::Limit;
    }

    impl<'a, O: Options> Options for &'a mut O {
        type Limit = O::Limit;
        type Endian = O::Endian;

        #[inline(always)]
        fn limit(&mut self) -> &mut Self::Limit {
            (*self).limit()
        }
    }

    /// A trait for stopping serialization and deserialization when a certain limit has been reached.
    pub trait SizeLimit {
        /// Tells the SizeLimit that a certain number of bytes has been
        /// read or written.  Returns Err if the limit has been exceeded.
        fn add(&mut self, n: u64) -> Result<()>;
        /// Returns the hard limit (if one exists)
        fn limit(&self) -> Option<u64>;
    }

    pub trait BincodeByteOrder {
        type Endian: ByteOrder + 'static;
    }
}
