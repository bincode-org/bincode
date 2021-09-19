use crate::int_encoding::FixintEncoding;

pub(crate) use self::internal::*;

pub trait Config: InternalConfig + Sized {}

pub struct Default;

impl InternalConfig for Default {
    type IntEncoding = FixintEncoding;
}

impl<T: InternalConfig> Config for T {}

mod internal {
    use crate::int_encoding::IntEncoding;

    pub trait InternalConfig {
        const ENDIAN: Endian = Endian::Little;
        const LIMIT: Option<u64> = None;
        const ALLOW_TRAILING: bool = true;

        type IntEncoding: IntEncoding;
    }

    #[derive(PartialEq, Eq)]
    pub enum Endian {
        Little,
        Big,
    }

    impl<'a, C: InternalConfig> InternalConfig for &'a mut C {
        const ENDIAN: Endian = C::ENDIAN;
        const LIMIT: Option<u64> = C::LIMIT;
        const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;

        type IntEncoding = C::IntEncoding;
    }
}
