use super::EncodedSize;
use crate::{
    config::{Config, IntEncoding},
    error::EncodeError,
};
use core::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Bound, Range, RangeInclusive},
    time::Duration,
};

impl EncodedSize for () {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(0)
    }
}

impl<T> EncodedSize for PhantomData<T> {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(0)
    }
}

impl EncodedSize for bool {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        u8::from(*self).encoded_size::<C>()
    }
}

impl EncodedSize for u8 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(1)
    }
}

impl EncodedSize for NonZeroU8 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for u16 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_u16(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroU16 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for u32 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_u32(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroU32 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for u64 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_u64(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroU64 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for u128 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_u128(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroU128 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for usize {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_usize(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<u64>()),
        }
    }
}

impl EncodedSize for NonZeroUsize {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for i8 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(1)
    }
}

impl EncodedSize for NonZeroI8 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for i16 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_i16(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroI16 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for i32 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_i32(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroI32 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for i64 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_i64(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroI64 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for i128 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_i128(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroI128 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for isize {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => Ok(crate::varint::varint_size_isize(*self)),
            IntEncoding::Fixed => Ok(std::mem::size_of::<Self>()),
        }
    }
}

impl EncodedSize for NonZeroIsize {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.get().encoded_size::<C>()
    }
}

impl EncodedSize for f32 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(std::mem::size_of::<Self>())
    }
}

impl EncodedSize for f64 {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(std::mem::size_of::<Self>())
    }
}

impl EncodedSize for char {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(encoded_size_utf8(*self))
    }
}

// BlockedTODO: https://github.com/rust-lang/rust/issues/37653
//
// We'll want to implement encoding for both &[u8] and &[T: EncodedSizedSize],
// but those implementations overlap because u8 also implements EncodedSize
// impl EncodedSize for &'_ [u8] {
//     fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
//         encoder.writer().write(*self)
//     }
// }

impl<T> EncodedSize for [T]
where
    T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = super::size_slice_len::<C>(self.len())?;
        for item in self {
            size += item.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;

fn encoded_size_utf8(c: char) -> usize {
    let code = c as u32;

    if code < MAX_ONE_B {
        1
    } else if code < MAX_TWO_B {
        2
    } else if code < MAX_THREE_B {
        3
    } else {
        4
    }
}

impl EncodedSize for str {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.as_bytes().encoded_size::<C>()
    }
}

impl<T, const N: usize> EncodedSize for [T; N]
where
    T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = 0;
        if !C::SKIP_FIXED_ARRAY_LENGTH {
            size += super::size_slice_len::<C>(N)?;
        }
        for item in self.iter() {
            size += item.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

impl<T> EncodedSize for Option<T>
where
    T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = 1;
        if let Some(val) = self {
            size += val.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

impl<T, U> EncodedSize for Result<T, U>
where
    T: EncodedSize,
    U: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match self {
            Ok(val) => Ok(0u32.encoded_size::<C>()? + val.encoded_size::<C>()?),
            Err(err) => Ok(1u32.encoded_size::<C>()? + err.encoded_size::<C>()?),
        }
    }
}

impl<T> EncodedSize for Cell<T>
where
    T: EncodedSize + Copy,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        T::encoded_size::<C>(&self.get())
    }
}

impl<T> EncodedSize for RefCell<T>
where
    T: EncodedSize + ?Sized,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let borrow_guard = self
            .try_borrow()
            .map_err(|e| EncodeError::RefCellAlreadyBorrowed {
                inner: e,
                type_name: core::any::type_name::<RefCell<T>>(),
            })?;
        T::encoded_size::<C>(&borrow_guard)
    }
}

impl EncodedSize for Duration {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.as_secs().encoded_size::<C>()? + self.subsec_nanos().encoded_size::<C>()?)
    }
}

impl<T> EncodedSize for Range<T>
where
    T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.start.encoded_size::<C>()? + self.end.encoded_size::<C>()?)
    }
}

impl<T> EncodedSize for RangeInclusive<T>
where
    T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.start().encoded_size::<C>()? + self.end().encoded_size::<C>()?)
    }
}

impl<T> EncodedSize for Bound<T>
where
    T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        match self {
            Self::Unbounded => 0u32.encoded_size::<C>(),
            Self::Included(val) => Ok(1u32.encoded_size::<C>()? + val.encoded_size::<C>()?),
            Self::Excluded(val) => Ok(2u32.encoded_size::<C>()? + val.encoded_size::<C>()?),
        }
    }
}

impl<'a, T> EncodedSize for &'a T
where
    T: EncodedSize + ?Sized,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        T::encoded_size::<C>(self)
    }
}
