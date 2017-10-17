use super::{SizeLimit, Infinite, Bounded};
use byteorder::{ByteOrder, BigEndian, LittleEndian, NativeEndian};
use serde;
use std::marker::PhantomData;

pub(crate) trait Options {
    type Limit: SizeLimit + 'static;
    type Endian: ByteOrder + 'static;

    fn limit(&mut self) -> &mut Self::Limit;
}

impl <'a, O: Options> Options for &'a mut O {
    type Limit = O::Limit;
    type Endian = O::Endian;

    fn limit(&mut self) -> &mut Self::Limit {
        self.limit()
    }
}

pub trait Config {
    fn with_no_limit(self: Box<Self>) -> Box<Config>;
    fn with_limit(self: Box<Self>, u64) -> Box<Config>;
    fn with_little_endian(self: Box<Self>) -> Box<Config>;
    fn with_big_endian(self: Box<Self>) -> Box<Config>;
    fn with_native_endian(self: Box<Self>) -> Box<Config>;
}

struct DefaultOptions {
    infinite: Infinite
}

pub(crate) struct WithOtherLimit<O: Options, L: SizeLimit> {
    _options: O,
    pub (crate) new_limit: L,
}

pub(crate) struct WithOtherEndian<O: Options, E: ByteOrder> {
    options: O,
    _endian: PhantomData<E>,
}

// This is a "shallow" config impl.  This matters when boxing.
pub(crate) struct ConfigImpl<L: SizeLimit + 'static, E: ByteOrder + 'static> {
    limit: L,
    _e: PhantomData<E>,
 }

impl Options for DefaultOptions {
    type Limit = Infinite;
    type Endian = LittleEndian;

    fn limit(&mut self) -> &mut Self::Limit {
        &mut self.infinite
    }
}

impl <L: SizeLimit + 'static, E: ByteOrder + 'static> Options for ConfigImpl<L, E> {
    type Limit = L;
    type Endian = E;

    fn limit(&mut self) -> &mut Self::Limit {
        &mut self.limit
    }
}
impl <L: SizeLimit, E: ByteOrder> ConfigImpl<L, E> {
    fn new(limit: L) -> ConfigImpl<L, E> {
        ConfigImpl {
            limit: limit,
            _e: PhantomData,
        }
    }
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

impl Config for DefaultOptions {
    fn with_no_limit(self: Box<Self>) -> Box<Config> {
        Box::new(WithOtherLimit::new(*self, Infinite))
    }
    fn with_limit(self: Box<Self>, limit: u64) -> Box<Config> {
        Box::new(WithOtherLimit::new(*self, Bounded(limit)))
    }
    fn with_little_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, LittleEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
    fn with_big_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, BigEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
    fn with_native_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, NativeEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
}

impl <O: Options + 'static, L: SizeLimit + 'static> Config for WithOtherLimit<O, L>{
    fn with_no_limit(self: Box<Self>) -> Box<Config> {
        Box::new(WithOtherLimit::new(*self, Infinite))
    }
    fn with_limit(self: Box<Self>, limit: u64) -> Box<Config> {
        Box::new(WithOtherLimit::new(*self, Bounded(limit)))
    }
    fn with_little_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, LittleEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
    fn with_big_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, BigEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
    fn with_native_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, NativeEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
}

impl <O: Options + 'static, E: ByteOrder + 'static> Config for WithOtherEndian<O, E>{
    fn with_no_limit(self: Box<Self>) -> Box<Config> {
        Box::new(WithOtherLimit::new(*self, Infinite))
    }
    fn with_limit(self: Box<Self>, limit: u64) -> Box<Config> {
        Box::new(WithOtherLimit::new(*self, Bounded(limit)))
    }
    fn with_little_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, LittleEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
    fn with_big_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, BigEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
    fn with_native_endian(self: Box<Self>) -> Box<Config> {
        let r: WithOtherEndian<_, NativeEndian> = WithOtherEndian::new(*self);
        Box::new(r)
    }
}

impl <L: SizeLimit + 'static, E: ByteOrder + 'static> Config for ConfigImpl<L, E>{
    fn with_no_limit(self: Box<Self>) -> Box<Config> {
        let r: ConfigImpl<Infinite, E> = ConfigImpl::new(Infinite);
        Box::new(r)
    }
    fn with_limit(self: Box<Self>, limit: u64) -> Box<Config> {
        let r: ConfigImpl<Bounded, E> = ConfigImpl::new(Bounded(limit));
        Box::new(r)
    }
    fn with_little_endian(self: Box<Self>) -> Box<Config> {
        let r: ConfigImpl<L, LittleEndian> = ConfigImpl::new(self.limit);
        Box::new(r)
    }
    fn with_big_endian(self: Box<Self>) -> Box<Config> {
        let r: ConfigImpl<L, BigEndian> = ConfigImpl::new(self.limit);
        Box::new(r)
    }
    fn with_native_endian(self: Box<Self>) -> Box<Config> {
        let r: ConfigImpl<L, NativeEndian> = ConfigImpl::new(self.limit);
        Box::new(r)
    }
}
