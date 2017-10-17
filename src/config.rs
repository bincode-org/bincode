use super::{SizeLimit, Infinite, Bounded};
use byteorder::{ByteOrder, BigEndian, LittleEndian, NativeEndian};
use std::marker::PhantomData;

pub(crate) trait Options {
    type Limit: SizeLimit + 'static;
    type Endian: ByteOrder + 'static;

    fn limit(&mut self) -> &mut Self::Limit;
}

pub trait Config {
    fn with_no_limit(self: Box<Self>) -> Box<Config>;
    fn with_limit(self: Box<Self>, u64) -> Box<Config>;
    fn with_little_endian(self: Box<Self>) -> Box<Config>;
    fn with_big_endian(self: Box<Self>) -> Box<Config>;
    fn with_native_endian(self: Box<Self>) -> Box<Config>;
}

pub(crate) struct ConfigImpl<L: SizeLimit + 'static, E: ByteOrder + 'static> {
    limit: L,
    _e: PhantomData<E>,
}

impl <L: SizeLimit, E: ByteOrder> ConfigImpl<L, E> {
    fn new(limit: L) -> ConfigImpl<L, E> {
        ConfigImpl {
            limit: limit,
            _e: PhantomData,
        }
    }
}

impl <L: SizeLimit + 'static, E: ByteOrder + 'static> Config for ConfigImpl<L, E> {
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

impl <L: SizeLimit + 'static, E: ByteOrder + 'static> Options for ConfigImpl<L, E> {
    type Limit = L;
    type Endian = E;

    fn limit(&mut self) -> &mut L {
        &mut self.limit
    }
}
