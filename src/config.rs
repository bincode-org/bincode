use super::{SizeLimit, Infinite, Bounded};
use ::Result;
use byteorder::{ByteOrder, BigEndian, LittleEndian, NativeEndian};
use serde;
use std::marker::PhantomData;

struct DefaultOptions(Infinite);

pub(crate) trait Options {
    type Limit: SizeLimit + 'static;
    type Endian: ByteOrder + 'static;

    fn limit(&mut self) -> &mut Self::Limit;
}

trait OptionsExt: Options + Sized {
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

impl <'a, O: Options> Options for &'a mut O {
    type Limit = O::Limit;
    type Endian = O::Endian;

    fn limit(&mut self) -> &mut Self::Limit {
        (*self).limit()
    }
}

impl <T: Options> OptionsExt for T {}

impl DefaultOptions {
    fn new() -> DefaultOptions {
        DefaultOptions(Infinite)
    }
}

impl Options for DefaultOptions {
    type Limit = Infinite;
    type Endian = LittleEndian;

    fn limit(&mut self) -> &mut Infinite {
        &mut self.0
    }
}

#[derive(Clone, Copy)]
enum LimitOption {
    Unlimited,
    Limited(u64)
}

#[derive(Clone, Copy)]
enum EndianOption {
    Big,
    Little,
    Native
}

/// TODO: document
pub struct Config {
    limit: LimitOption,
    endian: EndianOption,
}

pub(crate) struct WithOtherLimit<O: Options, L: SizeLimit> {
    _options: O,
    pub (crate) new_limit: L,
}

pub(crate) struct WithOtherEndian<O: Options, E: ByteOrder> {
    options: O,
    _endian: PhantomData<E>,
}


impl <O: Options, L: SizeLimit> WithOtherLimit<O, L> {
    pub(crate) fn new(options: O, limit: L) -> WithOtherLimit<O, L> {
        WithOtherLimit {
            _options: options,
            new_limit: limit
        }
    }
}

impl <O: Options, E: ByteOrder> WithOtherEndian<O, E> {
    pub(crate) fn new(options: O) -> WithOtherEndian<O, E> {
        WithOtherEndian {
            options: options,
            _endian: PhantomData
        }
    }
}

impl <O: Options, E: ByteOrder + 'static> Options for WithOtherEndian<O, E> {
    type Limit = O::Limit;
    type Endian = E;

    fn limit(&mut self) -> &mut O::Limit {
        self.options.limit()
    }
}

impl <O: Options, L: SizeLimit + 'static> Options for WithOtherLimit<O, L> {
    type Limit = L;
    type Endian = O::Endian;

    fn limit(&mut self) -> &mut L {
        &mut self.new_limit
    }
}

macro_rules! config_map {
    ($limit:expr, $endian:expr, $opts:ident => $call:tt) => {
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
    fn new() -> Config {
        Config {
            limit: LimitOption::Unlimited,
            endian: EndianOption::Little,
        }
    }

    /// TODO: Document
    pub fn with_no_limit(self) -> Config {
        Config {
            limit: LimitOption::Unlimited,
            .. self
        }
    }

    /// TODO: Document
    pub fn with_limit(self, limit: u64) -> Config {
        Config {
            limit: LimitOption::Limited(limit),
            .. self
        }
    }

    /// TODO: Document
    pub fn with_little_endian(self) -> Config {
        Config {
            endian: EndianOption::Little,
            .. self
        }
    }

    /// TODO: Document
    pub fn with_big_endian(self) -> Config {
        Config {
            endian: EndianOption::Big,
            .. self
        }
    }

    /// TODO: Document
    pub fn with_native_endian(self) -> Config {
        Config {
            endian: EndianOption::Native,
            .. self
        }
    }

    /// TODO: Document
    pub fn serialize<T: ?Sized + serde::Serialize>(&self, t: &T) -> Result<Vec<u8>> {
        use self::LimitOption::*;
        use self::EndianOption::*;

        match (self.limit, self.endian) {
            (Unlimited, Little) => {
                let opts = DefaultOptions::new().with_no_limit().with_little_endian();
                ::internal::serialize(t, opts)
            }
            (Unlimited, Big) => {
                let opts = DefaultOptions::new().with_no_limit().with_big_endian();
                ::internal::serialize(t, opts)
            }
            (Unlimited, Native) => {
                let opts = DefaultOptions::new().with_no_limit().with_native_endian();
                ::internal::serialize(t, opts)
            }

            (Limited(l), Little) => {
                let opts = DefaultOptions::new().with_limit(l).with_little_endian();
                ::internal::serialize(t, opts)
            }
            (Limited(l), Big) => {
                let opts = DefaultOptions::new().with_limit(l).with_big_endian();
                ::internal::serialize(t, opts)
            }
            (Limited(l), Native) => {
                let opts = DefaultOptions::new().with_limit(l).with_native_endian();
                ::internal::serialize(t, opts)
            }
        }

    }
}
