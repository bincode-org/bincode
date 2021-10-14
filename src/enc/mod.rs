//! Encoder-based structs and traits.

mod encoder;
mod impl_tuples;
mod impls;

use crate::error::EncodeError;

pub mod write;

pub use self::encoder::Encoder;

/// Any source that can encode types. This type is most notably implemented for [Encoder].
///
/// [Encoder]: ../struct.Encoder.html
pub trait Encodeable {
    /// Encode a given type.
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError>;
}

/// Helper trait to encode basic types into.
pub trait Encode {
    /// Encode an `u8`
    fn encode_u8(&mut self, val: u8) -> Result<(), EncodeError>;
    /// Encode an `u16`
    fn encode_u16(&mut self, val: u16) -> Result<(), EncodeError>;
    /// Encode an `u32`
    fn encode_u32(&mut self, val: u32) -> Result<(), EncodeError>;
    /// Encode an `u64`
    fn encode_u64(&mut self, val: u64) -> Result<(), EncodeError>;
    /// Encode an `u128`
    fn encode_u128(&mut self, val: u128) -> Result<(), EncodeError>;
    /// Encode an `usize`
    fn encode_usize(&mut self, val: usize) -> Result<(), EncodeError>;

    /// Encode an `i8`
    fn encode_i8(&mut self, val: i8) -> Result<(), EncodeError>;
    /// Encode an `i16`
    fn encode_i16(&mut self, val: i16) -> Result<(), EncodeError>;
    /// Encode an `i32`
    fn encode_i32(&mut self, val: i32) -> Result<(), EncodeError>;
    /// Encode an `i64`
    fn encode_i64(&mut self, val: i64) -> Result<(), EncodeError>;
    /// Encode an `i128`
    fn encode_i128(&mut self, val: i128) -> Result<(), EncodeError>;
    /// Encode an `isize`
    fn encode_isize(&mut self, val: isize) -> Result<(), EncodeError>;

    /// Encode an `f32`
    fn encode_f32(&mut self, val: f32) -> Result<(), EncodeError>;
    /// Encode an `f64`
    fn encode_f64(&mut self, val: f64) -> Result<(), EncodeError>;
    /// Encode a slice. Exactly `val.len()` bytes must be encoded, else an error should be thrown.
    fn encode_slice(&mut self, val: &[u8]) -> Result<(), EncodeError>;
    /// Encode an array. Exactly `N` bytes must be encoded, else an error should be thrown.
    fn encode_array<const N: usize>(&mut self, val: [u8; N]) -> Result<(), EncodeError>;

    /// Encode a single utf8 char
    fn encode_char(&mut self, val: char) -> Result<(), EncodeError>;
}
