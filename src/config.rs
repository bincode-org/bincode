pub(crate) use self::internal::*;
use std::marker::PhantomData;

pub trait Config: InternalConfig + Copy + Clone + Sized {
    fn with_big_endian(self) -> BigEndian<Self> {
        BigEndian { _pd: PhantomData }
    }
    fn with_little_endian(self) -> LittleEndian<Self> {
        LittleEndian { _pd: PhantomData }
    }
    fn with_variable_int_encoding(self) -> Varint<Self> {
        Varint { _pd: PhantomData }
    }
    fn with_fixed_int_encoding(self) -> Fixint<Self> {
        Fixint { _pd: PhantomData }
    }
}

impl<T: InternalConfig> Config for T {}

#[derive(Copy, Clone)]
pub struct Default;

impl InternalConfig for Default {
    const ENDIAN: Endian = Endian::Little;
    const INT_ENCODING: IntEncoding = IntEncoding::Variable;
    const LIMIT: Option<u64> = None;
    const ALLOW_TRAILING: bool = true;
}

#[derive(Copy, Clone)]
pub struct BigEndian<C: Config> {
    _pd: PhantomData<C>,
}

impl<C: InternalConfig> InternalConfig for BigEndian<C> {
    const ENDIAN: Endian = Endian::Big;
    const INT_ENCODING: IntEncoding = C::INT_ENCODING;
    const LIMIT: Option<u64> = C::LIMIT;
    const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
}

#[derive(Copy, Clone)]
pub struct LittleEndian<C: Config> {
    _pd: PhantomData<C>,
}

impl<C: InternalConfig> InternalConfig for LittleEndian<C> {
    const ENDIAN: Endian = Endian::Little;
    const INT_ENCODING: IntEncoding = C::INT_ENCODING;
    const LIMIT: Option<u64> = C::LIMIT;
    const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
}

#[derive(Copy, Clone)]
pub struct Fixint<C: Config> {
    _pd: PhantomData<C>,
}

impl<C: InternalConfig> InternalConfig for Fixint<C> {
    const ENDIAN: Endian = C::ENDIAN;
    const INT_ENCODING: IntEncoding = IntEncoding::Fixed;
    const LIMIT: Option<u64> = C::LIMIT;
    const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
}

#[derive(Copy, Clone)]
pub struct Varint<C: Config> {
    _pd: PhantomData<C>,
}

impl<C: InternalConfig> InternalConfig for Varint<C> {
    const ENDIAN: Endian = C::ENDIAN;
    const INT_ENCODING: IntEncoding = IntEncoding::Variable;
    const LIMIT: Option<u64> = C::LIMIT;
    const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
}

mod internal {
    pub trait InternalConfig: Copy + Clone {
        const ENDIAN: Endian;
        const INT_ENCODING: IntEncoding;
        const LIMIT: Option<u64>;
        const ALLOW_TRAILING: bool;
    }

    #[derive(PartialEq, Eq)]
    pub enum Endian {
        Little,
        Big,
    }

    #[derive(PartialEq, Eq)]
    pub enum IntEncoding {
        Fixed,
        Variable,
    }

    impl<'a, C: InternalConfig> InternalConfig for &'a mut C
    where
        &'a mut C: Copy + Clone,
    {
        const ENDIAN: Endian = C::ENDIAN;
        const INT_ENCODING: IntEncoding = C::INT_ENCODING;
        const LIMIT: Option<u64> = C::LIMIT;
        const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
    }
}
