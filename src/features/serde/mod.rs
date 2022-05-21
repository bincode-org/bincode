//! Support for serde integration. Enable this with the `serde` feature.
//!
//! To encode/decode type that implement serde's trait, you can use:
//! - [decode_borrowed_from_slice]
//! - [decode_from_slice]
//! - [encode_to_slice]
//! - [encode_to_vec]
//!
//! For interop with bincode's [Decode]/[Encode], you can use:
//! - [Compat]
//! - [BorrowCompat]
//!
//! For interop with bincode's `derive` feature, you can use the `#[bincode(with_serde)]` attribute on each field that implements serde's traits.
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # mod foo {
//! # use bincode::{Decode, Encode};
//! # use serde_derive::{Deserialize, Serialize};
//! #[derive(Serialize, Deserialize)]
//! pub struct SerdeType {
//!     // ...
//! }
//!
//! #[derive(Decode, Encode)]
//! pub struct StructWithSerde {
//!     #[bincode(with_serde)]
//!     pub serde: SerdeType,
//! }
//!
//! #[derive(Decode, Encode)]
//! pub enum EnumWithSerde {
//!     Unit(#[bincode(with_serde)] SerdeType),
//!     Struct {
//!         #[bincode(with_serde)]
//!         serde: SerdeType,
//!     },
//! }
//! # }
//! ```
//!
//! # Known issues
//!
//! Currently the `serde` feature will automatically enable the `alloc` and `std` feature. If you're running in a `#[no_std]` environment consider using bincode's own derive macros.
//!
//! Because bincode is a format without meta data, there are several known issues with serde's attributes. Please do not use any of the following attributes if you plan on using bincode, or use bincode's own `derive` macros.
//! - `#[serde(skip)]`
//! - `#[serde(skip_serializing)]`
//! - `#[serde(skip_deserializing)]`
//! - `#[serde(skip_serializing_if = "path")]`
//! - `#[serde(flatten)]`
//! - `#[serde(untagged)]`
//!
//! **Using any of the above attributes can and will cause issues with bincode and will result in lost data**. Consider using bincode's own derive macro instead.
//!
//! [Decode]: ../de/trait.Decode.html
//! [Encode]: ../enc/trait.Encode.html

mod de_borrowed;
mod de_owned;
mod ser;

pub use self::de_borrowed::*;
pub use self::de_owned::*;
pub use self::ser::*;

/// A serde-specific error that occurred while decoding.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum DecodeError {
    /// Bincode does not support serde's `any` decoding feature
    AnyNotSupported,

    /// Bincode does not support serde identifiers
    IdentifierNotSupported,

    /// Bincode does not support serde's `ignored_any`
    IgnoredAnyNotSupported,

    /// Serde tried decoding a borrowed value from an owned reader. Use `serde_decode_borrowed_from_*` instead
    CannotBorrowOwnedData,

    /// Serde does not support skipping fixed array lengths
    SkipFixedArrayLengthNotSupported,

    /// Could not allocate data like `String` and `Vec<u8>`
    #[cfg(not(feature = "alloc"))]
    CannotAllocate,

    /// Custom serde error but bincode is unable to allocate a string. Set a breakpoint where this is thrown for more information.
    #[cfg(not(feature = "alloc"))]
    CustomError,
}

#[cfg(feature = "alloc")]
impl serde::de::Error for crate::error::DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        use alloc::string::ToString;
        Self::OtherString(msg.to_string())
    }
}

#[cfg(not(feature = "std"))]
impl serde::de::StdError for crate::error::DecodeError {}

#[cfg(not(feature = "alloc"))]
impl serde::de::Error for crate::error::DecodeError {
    fn custom<T>(_: T) -> Self
    where
        T: core::fmt::Display,
    {
        DecodeError::CustomError.into()
    }
}

#[allow(clippy::from_over_into)]
impl Into<crate::error::DecodeError> for DecodeError {
    fn into(self) -> crate::error::DecodeError {
        crate::error::DecodeError::Serde(self)
    }
}

/// A serde-specific error that occurred while encoding.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum EncodeError {
    /// Serde provided bincode with a sequence without a length, which is not supported in bincode
    SequenceMustHaveLength,

    /// Serde does not support skipping fixed array lengths
    SkipFixedArrayLengthNotSupported,

    /// [Serializer::collect_str] got called but bincode was unable to allocate memory.
    #[cfg(not(feature = "alloc"))]
    CannotCollectStr,

    /// Custom serde error but bincode is unable to allocate a string. Set a breakpoint where this is thrown for more information.
    #[cfg(not(feature = "alloc"))]
    CustomError,
}

#[allow(clippy::from_over_into)]
impl Into<crate::error::EncodeError> for EncodeError {
    fn into(self) -> crate::error::EncodeError {
        crate::error::EncodeError::Serde(self)
    }
}

#[cfg(feature = "alloc")]
impl serde::ser::Error for crate::error::EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        use alloc::string::ToString;

        Self::OtherString(msg.to_string())
    }
}

#[cfg(not(feature = "std"))]
impl serde::de::StdError for crate::error::EncodeError {}

#[cfg(not(feature = "alloc"))]
impl serde::ser::Error for crate::error::EncodeError {
    fn custom<T>(_: T) -> Self
    where
        T: core::fmt::Display,
    {
        EncodeError::CustomError.into()
    }
}

/// Wrapper struct that implements [Decode] and [Encode] on any type that implements serde's [DeserializeOwned] and [Serialize] respectively.
///
/// This works for most types, but if you're dealing with borrowed data consider using [BorrowCompat] instead.
///
/// [Decode]: ../de/trait.Decode.html
/// [Encode]: ../enc/trait.Encode.html
/// [DeserializeOwned]: https://docs.rs/serde/1/serde/de/trait.DeserializeOwned.html
/// [Serialize]: https://docs.rs/serde/1/serde/trait.Serialize.html
pub struct Compat<T>(pub T);

impl<T> crate::Decode for Compat<T>
where
    T: serde::de::DeserializeOwned,
{
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        let serde_decoder = de_owned::SerdeDecoder { de: decoder };
        T::deserialize(serde_decoder).map(Compat)
    }
}

impl<T> crate::Encode for Compat<T>
where
    T: serde::Serialize,
{
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        let serializer = ser::SerdeEncoder { enc: encoder };
        self.0.serialize(serializer)?;
        Ok(())
    }
}

/// Wrapper struct that implements [BorrowDecode] and [Encode] on any type that implements serde's [Deserialize] and [Serialize] respectively. This is mostly used on `&[u8]` and `&str`, for other types consider using [Compat] instead.
///
/// [BorrowDecode]: ../de/trait.BorrowDecode.html
/// [Encode]: ../enc/trait.Encode.html
/// [Deserialize]: https://docs.rs/serde/1/serde/de/trait.Deserialize.html
/// [Serialize]: https://docs.rs/serde/1/serde/trait.Serialize.html
pub struct BorrowCompat<T>(pub T);

impl<'de, T> crate::de::BorrowDecode<'de> for BorrowCompat<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn borrow_decode<D: crate::de::BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        let serde_decoder = de_borrowed::SerdeDecoder {
            de: decoder,
            pd: core::marker::PhantomData,
        };
        T::deserialize(serde_decoder).map(BorrowCompat)
    }
}

impl<T> crate::Encode for BorrowCompat<T>
where
    T: serde::Serialize,
{
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        let serializer = ser::SerdeEncoder { enc: encoder };
        self.0.serialize(serializer)?;
        Ok(())
    }
}
