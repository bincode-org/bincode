use super::{Encode, Encodeable};
use crate::error::EncodeError;
use core::{
    cell::{Cell, RefCell},
    ops::{Bound, Range, RangeInclusive},
    time::Duration,
};

impl Encodeable for bool {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u8(if *self { 1 } else { 0 })
    }
}

impl Encodeable for u8 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u8(*self)
    }
}

impl Encodeable for u16 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u16(*self)
    }
}

impl Encodeable for u32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u32(*self)
    }
}

impl Encodeable for u64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u64(*self)
    }
}

impl Encodeable for u128 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_u128(*self)
    }
}

impl Encodeable for usize {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_usize(*self)
    }
}

impl Encodeable for i8 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i8(*self)
    }
}

impl Encodeable for i16 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i16(*self)
    }
}

impl Encodeable for i32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i32(*self)
    }
}

impl Encodeable for i64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i64(*self)
    }
}

impl Encodeable for i128 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_i128(*self)
    }
}

impl Encodeable for isize {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_isize(*self)
    }
}

impl Encodeable for f32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_f32(*self)
    }
}

impl Encodeable for f64 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_f64(*self)
    }
}

impl Encodeable for char {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_char(*self)
    }
}

impl Encodeable for &'_ [u8] {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_slice(*self)
    }
}

// BlockedTODO: https://github.com/rust-lang/rust/issues/37653
//
// We'll want to implement encoding for both &[u8] and &[T: Encodeable],
// but those implementations overlap because u8 also implements Encodeabl
//
// default impl Encodeable for &'_ [u8] {
//     fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
//         encoder.encode_slice(*self)
//     }
// }
//
// impl<T: Encodeable> Encodeable for &'_ [T] {
//     fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
//         self.len().encode(&mut encoder)?;
//         for item in self.iter() {
//             item.encode(&mut encoder)?;
//         }
//         Ok(())
//     }
// }

impl Encodeable for &'_ str {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_slice(self.as_bytes())
    }
}

impl<const N: usize> Encodeable for [u8; N] {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        encoder.encode_array(*self)
    }
}

impl<T> Encodeable for Option<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        if let Some(val) = self {
            1u8.encode(&mut encoder)?;
            val.encode(encoder)
        } else {
            0u8.encode(encoder)
        }
    }
}

impl<T, U> Encodeable for Result<T, U>
where
    T: Encodeable,
    U: Encodeable,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        match self {
            Ok(val) => {
                0u8.encode(&mut encoder)?;
                val.encode(encoder)
            }
            Err(err) => {
                1u8.encode(&mut encoder)?;
                err.encode(encoder)
            }
        }
    }
}

impl<T> Encodeable for Cell<T>
where
    T: Encodeable + Copy,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        T::encode(&self.get(), encoder)
    }
}

impl<T> Encodeable for RefCell<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        let borrow_guard = self
            .try_borrow()
            .map_err(|e| EncodeError::RefCellAlreadyBorrowed {
                inner: e,
                type_name: core::any::type_name::<RefCell<T>>(),
            })?;
        T::encode(&borrow_guard, encoder)
    }
}

impl Encodeable for Duration {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.as_secs().encode(&mut encoder)?;
        self.subsec_nanos().encode(&mut encoder)?;
        Ok(())
    }
}

impl<T> Encodeable for Range<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.start.encode(&mut encoder)?;
        self.end.encode(&mut encoder)?;
        Ok(())
    }
}

impl<T> Encodeable for RangeInclusive<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.start().encode(&mut encoder)?;
        self.end().encode(&mut encoder)?;
        Ok(())
    }
}

impl<T> Encodeable for Bound<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        match self {
            Self::Unbounded => {
                0u32.encode(encoder)?;
            }
            Self::Included(val) => {
                1u32.encode(&mut encoder)?;
                val.encode(encoder)?;
            }
            Self::Excluded(val) => {
                2u32.encode(&mut encoder)?;
                val.encode(encoder)?;
            }
        }
        Ok(())
    }
}

impl<'a, T> Encodeable for &'a T
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

impl<'a, T> Encode for &'a mut T
where
    T: Encode,
{
    fn encode_u8(&mut self, val: u8) -> Result<(), EncodeError> {
        T::encode_u8(self, val)
    }
    fn encode_u16(&mut self, val: u16) -> Result<(), EncodeError> {
        T::encode_u16(self, val)
    }
    fn encode_u32(&mut self, val: u32) -> Result<(), EncodeError> {
        T::encode_u32(self, val)
    }
    fn encode_u64(&mut self, val: u64) -> Result<(), EncodeError> {
        T::encode_u64(self, val)
    }
    fn encode_u128(&mut self, val: u128) -> Result<(), EncodeError> {
        T::encode_u128(self, val)
    }
    fn encode_usize(&mut self, val: usize) -> Result<(), EncodeError> {
        T::encode_usize(self, val)
    }

    fn encode_i8(&mut self, val: i8) -> Result<(), EncodeError> {
        T::encode_i8(self, val)
    }
    fn encode_i16(&mut self, val: i16) -> Result<(), EncodeError> {
        T::encode_i16(self, val)
    }
    fn encode_i32(&mut self, val: i32) -> Result<(), EncodeError> {
        T::encode_i32(self, val)
    }
    fn encode_i64(&mut self, val: i64) -> Result<(), EncodeError> {
        T::encode_i64(self, val)
    }
    fn encode_i128(&mut self, val: i128) -> Result<(), EncodeError> {
        T::encode_i128(self, val)
    }
    fn encode_isize(&mut self, val: isize) -> Result<(), EncodeError> {
        T::encode_isize(self, val)
    }

    fn encode_f32(&mut self, val: f32) -> Result<(), EncodeError> {
        T::encode_f32(self, val)
    }
    fn encode_f64(&mut self, val: f64) -> Result<(), EncodeError> {
        T::encode_f64(self, val)
    }
    fn encode_slice(&mut self, val: &[u8]) -> Result<(), EncodeError> {
        T::encode_slice(self, val)
    }
    fn encode_array<const N: usize>(&mut self, val: [u8; N]) -> Result<(), EncodeError> {
        T::encode_array(self, val)
    }

    fn encode_char(&mut self, val: char) -> Result<(), EncodeError> {
        T::encode_char(self, val)
    }
}
