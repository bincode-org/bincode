mod de_borrowed;
mod de_owned;
mod ser;

pub use self::de_borrowed::*;
pub use self::de_owned::*;
pub use self::ser::*;

impl serde_incl::de::Error for crate::error::DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        use alloc::string::ToString;
        Self::OtherString(msg.to_string())
    }
}

impl serde_incl::ser::Error for crate::error::EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        use alloc::string::ToString;

        Self::OtherString(msg.to_string())
    }
}
