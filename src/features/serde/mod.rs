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
//! # #[serde(crate = "serde_incl")]
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
//! Because bincode is a format without meta data, there are several known issues with serde's `skip` attributes. Please do not use `skip` attributes if you plan on using bincode, or use bincode's own `derive` macros.
//!
//! This includes:
//! - `#[serde(skip)]`
//! - `#[serde(skip_serializing)]`
//! - `#[serde(skip_deserializing)]`
//! - `#[serde(skip_serializing_if = "path")]`
//! - `#[serde(flatten)]`
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
    T: serde_incl::de::DeserializeOwned,
{
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        let serde_decoder = de_owned::SerdeDecoder { de: decoder };
        T::deserialize(serde_decoder).map(Compat)
    }
}

impl<T> crate::Encode for Compat<T>
where
    T: serde_incl::Serialize,
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
    T: serde_incl::de::Deserialize<'de>,
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
    T: serde_incl::Serialize,
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
