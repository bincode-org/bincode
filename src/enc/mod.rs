//! Encoder-based structs and traits.

mod encoder;
mod impl_tuples;
mod impls;

use self::write::Writer;
use crate::{config::Config, error::EncodeError, utils::Sealed};

pub mod write;

pub use self::encoder::EncoderImpl;

/// Any source that can be encoded. This trait should be implemented for all types that you want to be able to use with any of the `encode_with` methods.
///
/// This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Encode)]` to your trait.
pub trait Encode {
    /// Encode a given type.
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError>;
}

/// Helper trait to encode basic types into.
pub trait Encoder: Sealed {
    /// The concrete [Writer] type
    type W: Writer;

    /// The concrete [Config] type
    type C: Config;

    /// Returns a mutable reference to the writer
    fn writer(&mut self) -> &mut Self::W;

    /// Returns a reference to the config
    fn config(&self) -> &Self::C;
}

impl<'a, T> Encoder for &'a mut T
where
    T: Encoder,
{
    type W = T::W;

    type C = T::C;

    fn writer(&mut self) -> &mut Self::W {
        T::writer(self)
    }

    fn config(&self) -> &Self::C {
        T::config(self)
    }
}

/// Encode the variant of the given option. Will not encode the option itself.
#[inline]
pub(crate) fn encode_option_variant<E: Encoder, T>(
    encoder: &mut E,
    value: &Option<T>,
) -> Result<(), EncodeError> {
    match value {
        None => 0u8.encode(encoder),
        Some(_) => 1u8.encode(encoder),
    }
}

/// Encodes the length of any slice, container, etc into the given encoder
#[inline]
pub(crate) fn encode_slice_len<E: Encoder>(encoder: &mut E, len: usize) -> Result<(), EncodeError> {
    (len as u64).encode(encoder)
}
