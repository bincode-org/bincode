//! Decoder-based structs and traits.

use crate::error::DecodeError;

mod decoder;
mod impl_tuples;
mod impls;

pub mod read;
pub use self::decoder::Decoder;

/// Trait that makes a type able to be decoded, akin to serde's `DeserializeOwned` trait.
///
/// This trait should be implemented for types which do not have references to data in the reader. For types that contain e.g. `&str` and `&[u8]`, implement [BorrowDecodable] instead.
///
/// Whenever you implement `Decodable` for your type, the base trait `BorrowDecodable` is automatically implemented.
pub trait Decodable: for<'de> BorrowDecodable<'de> {
    /// Attempt to decode this type with the given [Decode].
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError>;
}

/// Trait that makes a type able to be decoded, akin to serde's `Deserialize` trait.
///
/// This trait should be implemented for types that contain borrowed data, like `&str` and `&[u8]`. If your type does not have borrowed data, consider implementing [Decodable] instead.
pub trait BorrowDecodable<'de>: Sized {
    /// Attempt to decode this type with the given [BorrowDecode].
    fn borrow_decode<D: BorrowDecode<'de>>(decoder: D) -> Result<Self, DecodeError>;
}

impl<'de, T: Decodable> BorrowDecodable<'de> for T {
    fn borrow_decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        Decodable::decode(decoder)
    }
}

/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
pub trait Decode {
    /// Attempt to decode a `u8`
    fn decode_u8(&mut self) -> Result<u8, DecodeError>;
    /// Attempt to decode a `u16`
    fn decode_u16(&mut self) -> Result<u16, DecodeError>;
    /// Attempt to decode a `u32`
    fn decode_u32(&mut self) -> Result<u32, DecodeError>;
    /// Attempt to decode a `u64`
    fn decode_u64(&mut self) -> Result<u64, DecodeError>;
    /// Attempt to decode a `u128`
    fn decode_u128(&mut self) -> Result<u128, DecodeError>;
    /// Attempt to decode a `usize`
    fn decode_usize(&mut self) -> Result<usize, DecodeError>;

    /// Attempt to decode a `i8`
    fn decode_i8(&mut self) -> Result<i8, DecodeError>;
    /// Attempt to decode a `i16`
    fn decode_i16(&mut self) -> Result<i16, DecodeError>;
    /// Attempt to decode a `i32`
    fn decode_i32(&mut self) -> Result<i32, DecodeError>;
    /// Attempt to decode a `i64`
    fn decode_i64(&mut self) -> Result<i64, DecodeError>;
    /// Attempt to decode a `i128`
    fn decode_i128(&mut self) -> Result<i128, DecodeError>;
    /// Attempt to decode a `isize`
    fn decode_isize(&mut self) -> Result<isize, DecodeError>;

    /// Attempt to decode a `f32`
    fn decode_f32(&mut self) -> Result<f32, DecodeError>;
    /// Attempt to decode a `f64`
    fn decode_f64(&mut self) -> Result<f64, DecodeError>;
    /// Attempt to decode an array of `N` entries.
    fn decode_array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError>;

    /// Attempt to decode a `char`
    fn decode_char(&mut self) -> Result<char, DecodeError>;
}

/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
///
/// This is an extension of [Decode] that can also return borrowed data.
pub trait BorrowDecode<'de>: Decode {
    /// Decode `len` bytes, returning the slice as borrowed data.
    fn decode_slice(&mut self, len: usize) -> Result<&'de [u8], DecodeError>;
}
