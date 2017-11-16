use super::internal::{Bounded, Infinite, SizeLimit};
use ::error::Result;
use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian};
use serde;
use std::io::{Write, Read};
use std::marker::PhantomData;

use self::LimitOption::*;
use self::EndianOption::*;

struct DefaultOptions(Infinite);

pub(crate) trait Options {
    type Limit: SizeLimit + 'static;
    type Endian: ByteOrder + 'static;

    #[inline(always)]
    fn limit(&mut self) -> &mut Self::Limit;
}

pub(crate) trait OptionsExt: Options + Sized {
    fn with_no_limit(self) -> WithOtherLimit<Self, Infinite> {
        WithOtherLimit::new(self, Infinite)
    }

    fn with_limit(self, limit: u64) -> WithOtherLimit<Self, Bounded> {
        WithOtherLimit::new(self, Bounded(limit))
    }

    fn with_little_endian(self) -> WithOtherEndian<Self, LittleEndian> {
        WithOtherEndian::new(self)
    }

    fn with_big_endian(self) -> WithOtherEndian<Self, BigEndian> {
        WithOtherEndian::new(self)
    }

    fn with_native_endian(self) -> WithOtherEndian<Self, NativeEndian> {
        WithOtherEndian::new(self)
    }
}

impl<'a, O: Options> Options for &'a mut O {
    type Limit = O::Limit;
    type Endian = O::Endian;

    #[inline(always)]
    fn limit(&mut self) -> &mut Self::Limit {
        (*self).limit()
    }
}

impl<T: Options> OptionsExt for T {}

impl DefaultOptions {
    fn new() -> DefaultOptions {
        DefaultOptions(Infinite)
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

#[derive(Clone, Copy)]
enum LimitOption {
    Unlimited,
    Limited(u64),
}

#[derive(Clone, Copy)]
enum EndianOption {
    Big,
    Little,
    Native,
}

/// TODO: document
pub struct Config {
    limit: LimitOption,
    endian: EndianOption,
}

pub(crate) struct WithOtherLimit<O: Options, L: SizeLimit> {
    _options: O,
    pub(crate) new_limit: L,
}

pub(crate) struct WithOtherEndian<O: Options, E: ByteOrder> {
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

impl<O: Options, E: ByteOrder> WithOtherEndian<O, E> {
    #[inline(always)]
    pub(crate) fn new(options: O) -> WithOtherEndian<O, E> {
        WithOtherEndian {
            options: options,
            _endian: PhantomData,
        }
    }
}

impl<O: Options, E: ByteOrder + 'static> Options for WithOtherEndian<O, E> {
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
    ($limit:expr, $endian:expr, $opts:ident => $call:expr) => {
        match ($limit, $endian) {
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
    }
}

impl Config {
    #[inline(always)]
    pub(crate) fn new() -> Config {
        Config {
            limit: LimitOption::Unlimited,
            endian: EndianOption::Little,
        }
    }

    /// TODO: Document
    #[inline(always)]
    pub fn no_limit(&mut self) -> &mut Self {
        self.limit = LimitOption::Unlimited;
        self
    }

    /// TODO: Document
    #[inline(always)]
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = LimitOption::Limited(limit);
        self
    }

    /// TODO: Document
    #[inline(always)]
    pub fn little_endian(&mut self) -> &mut Self {
        self.endian= EndianOption::Little;
        self
    }

    /// TODO: Document
    #[inline(always)]
    pub fn big_endian(&mut self) -> &mut Self {
        self.endian= EndianOption::Big;
        self
    }

    /// TODO: Document
    #[inline(always)]
    pub fn native_endian(&mut self) -> &mut Self {
        self.endian = EndianOption::Native;
        self
    }

    /// TODO: Document
    #[inline(always)]
    pub fn serialize<T: ?Sized + serde::Serialize>(&self, t: &T) -> Result<Vec<u8>> {
        config_map!(self.limit, self.endian, opts => ::internal::serialize(t, opts))
    }

    /// TODO: Document
    #[inline(always)]
    pub fn serialized_size<T: ?Sized + serde::Serialize>(&self, t: &T) -> Result<u64> {
        config_map!(self.limit, self.endian, opts => ::internal::serialized_size(t, opts))
    }

    /// TODO: Document
    #[inline(always)]
    pub fn serialize_into<W: Write, T: ?Sized + serde::Serialize>(&self, w: W, t: &T) -> Result<()> {
        config_map!(self.limit, self.endian, opts => ::internal::serialize_into(w, t, opts))
    }

    /// TODO: Document
    #[inline(always)]
    pub fn deserialize<'a, T: serde::Deserialize<'a>>(&self, bytes: &'a [u8]) -> Result<T> {
        config_map!(self.limit, self.endian, opts => ::internal::deserialize(bytes, opts))
    }

    /// TODO: Document
    #[inline(always)]
    pub fn deserialize_from<'a, R: Read, T: serde::de::DeserializeOwned>(&self, r: R) -> Result<T> {
        config_map!(self.limit, self.endian, opts => ::internal::deserialize_from(r, opts))
    }
}
