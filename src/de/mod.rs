//! Decoder-based structs and traits.

mod decoder;
mod impl_core;
mod impl_tuples;
mod impls;

use self::read::{BorrowReader, Reader};
use crate::{config::Config, error::DecodeError, utils::Sealed};

pub mod read;

pub use self::decoder::DecoderImpl;

/// Trait that makes a type able to be decoded, akin to serde's `DeserializeOwned` trait.
///
/// This trait should be implemented for types which do not have references to data in the reader. For types that contain e.g. `&str` and `&[u8]`, implement [BorrowDecode] instead.
///
/// Whenever you implement `Decode` for your type, the base trait `BorrowDecode` is automatically implemented.
///
/// This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Decode)]` to your type. Note that if the type contains any lifetimes, `BorrowDecode` will be implemented instead.
pub trait Decode: for<'de> BorrowDecode<'de> {
    /// Attempt to decode this type with the given [Decode].
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError>;
}

/// Trait that makes a type able to be decoded, akin to serde's `Deserialize` trait.
///
/// This trait should be implemented for types that contain borrowed data, like `&str` and `&[u8]`. If your type does not have borrowed data, consider implementing [Decode] instead.
///
/// This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Decode)]` to a type with a lifetime.
pub trait BorrowDecode<'de>: Sized {
    /// Attempt to decode this type with the given [BorrowDecode].
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: D) -> Result<Self, DecodeError>;
}

impl<'de, T: Decode> BorrowDecode<'de> for T {
    fn borrow_decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        Decode::decode(decoder)
    }
}

/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
pub trait Decoder: Sealed {
    /// The concrete [Reader] type
    type R: Reader;

    /// The concrete [Config] type
    type C: Config;

    /// Returns a mutable reference to the reader
    fn reader(&mut self) -> &mut Self::R;

    /// Returns a mutable reference to the config
    fn config(&self) -> &Self::C;

    /// Claim that `n` bytes are going to be read from the decoder.
    /// This can be used to validate `Configuration::Limit<N>()`.
    fn claim_bytes_read(&mut self, n: usize) -> Result<(), DecodeError>;

    /// Notify the decoder that `n` bytes are being reclaimed.
    ///
    /// When decoding container types, a typical implementation would claim to read `len * size_of::<T>()` bytes.
    /// This is to ensure that bincode won't allocate several GB of memory while constructing the container.
    ///
    /// Because the implementation claims `len * size_of::<T>()`, but then has to decode each `T`, this would be marked
    /// as double. This function allows us to un-claim each `T` that gets decoded.
    ///
    /// We cannot check if `len * size_of::<T>()` is valid without claiming it, because this would mean that if you have
    /// a nested container (e.g. `Vec<Vec<T>>`), it does not know how much memory is already claimed, and could easily
    /// allocate much more than the user intends.
    /// ```
    /// # use bincode::de::{Decode, Decoder, DecodeError};
    /// # struct Container<T>(Vec<T>);
    /// # impl<T> Container<T> {
    /// #     fn with_capacity(cap: usize) -> Self {
    /// #         Self(Vec::with_capacity(cap))
    /// #     }
    /// #     
    /// #     fn push(&mut self, t: T) {
    /// #         self.0.push(t);
    /// #     }
    /// # }
    /// impl<T: Decode> Decode for Container<T> {
    ///     fn decode<D: Decoder>(&mut self, mut decoder: D) -> Result<(), DecodeError> {
    ///         let len = crate::de::decode_slice_len(&mut decoder)?
    ///         // Make sure we don't allocate too much memory
    ///         d.notify_bytes_read(len * core::mem::size_of::<T>());
    ///
    ///         let mut result = Container::with_capacity(len);
    ///         for _ in 0..len {
    ///             // un-claim the memory
    ///             d.unclaim_bytes_read(core::mem::size_of::<T>());
    ///             result.push(T::decode(&mut decoder)?)
    ///         }
    ///     }
    /// }
    /// ```
    fn unclaim_bytes_read(&mut self, n: usize);
}

/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
///
/// This is an extension of [Decode] that can also return borrowed data.
pub trait BorrowDecoder<'de>: Decoder {
    /// The concrete [BorrowReader] type
    type BR: BorrowReader<'de>;

    /// Rerturns a mutable reference to the borrow reader
    fn borrow_reader(&mut self) -> &mut Self::BR;
}

impl<'a, T> Decoder for &'a mut T
where
    T: Decoder,
{
    type R = T::R;

    type C = T::C;

    fn reader(&mut self) -> &mut Self::R {
        T::reader(self)
    }

    fn config(&self) -> &Self::C {
        T::config(self)
    }

    fn claim_bytes_read(&mut self, n: usize) -> Result<(), DecodeError> {
        T::claim_bytes_read(self, n)
    }

    fn unclaim_bytes_read(&mut self, n: usize) {
        T::unclaim_bytes_read(self, n)
    }
}

impl<'a, 'de, T> BorrowDecoder<'de> for &'a mut T
where
    T: BorrowDecoder<'de>,
{
    type BR = T::BR;

    fn borrow_reader(&mut self) -> &mut Self::BR {
        T::borrow_reader(self)
    }
}

/// Decodes only the option variant from the decoder. Will not read any more data than that.
#[inline]
pub(crate) fn decode_option_variant<D: Decoder>(
    decoder: D,
    type_name: &'static str,
) -> Result<Option<()>, DecodeError> {
    let is_some = u8::decode(decoder)?;
    match is_some {
        0 => Ok(None),
        1 => Ok(Some(())),
        x => Err(DecodeError::UnexpectedVariant {
            found: x as u32,
            allowed: crate::error::AllowedEnumVariants::Range { max: 1, min: 0 },
            type_name,
        }),
    }
}

/// Decodes the length of any slice, container, etc from the decoder
#[inline]
pub(crate) fn decode_slice_len<D: Decoder>(decoder: D) -> Result<usize, DecodeError> {
    u64::decode(decoder).map(|v| v as usize)
}
