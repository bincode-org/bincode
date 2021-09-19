pub(crate) use self::internal::*;

pub trait Config: InternalConfig + Sized {}

pub struct Default;

impl InternalConfig for Default {}

impl<T: InternalConfig> Config for T {}

mod internal {
    pub trait InternalConfig {
        const ENDIAN: Endian = Endian::Little;
        const INT_ENCODING: IntEncoding = IntEncoding::Variable;
        const LIMIT: Option<u64> = None;
        const ALLOW_TRAILING: bool = true;
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

    impl<'a, C: InternalConfig> InternalConfig for &'a mut C {
        const ENDIAN: Endian = C::ENDIAN;
        const INT_ENCODING: IntEncoding = C::INT_ENCODING;
        const LIMIT: Option<u64> = C::LIMIT;
        const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
    }
}
