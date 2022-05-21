use super::{write::Writer, Encode, Encoder};
use crate::{
    config::{
        Endian, IntEncoding, InternalArrayLengthConfig, InternalEndianConfig,
        InternalIntEncodingConfig,
    },
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

impl Encode for () {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl<T> Encode for PhantomData<T> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl Encode for bool {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        if *self { 1u8 } else { 0u8 }.encode(encoder)
    }
}

impl Encode for u8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&[*self])
    }
}

impl Encode for NonZeroU8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for u16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u16(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroU16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for u32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u32(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroU32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for u64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u64(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroU64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for u128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u128(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroU128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for usize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_usize(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&(*self as u64).to_be_bytes()),
                Endian::Little => encoder.writer().write(&(*self as u64).to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroUsize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&[*self as u8])
    }
}

impl Encode for NonZeroI8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i16(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroI16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i32(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroI32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i64(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroI64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i128(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroI128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for isize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_isize(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&(*self as i64).to_be_bytes()),
                Endian::Little => encoder.writer().write(&(*self as i64).to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroIsize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for f32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::ENDIAN {
            Endian::Big => encoder.writer().write(&self.to_be_bytes()),
            Endian::Little => encoder.writer().write(&self.to_le_bytes()),
        }
    }
}

impl Encode for f64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::ENDIAN {
            Endian::Big => encoder.writer().write(&self.to_be_bytes()),
            Endian::Little => encoder.writer().write(&self.to_le_bytes()),
        }
    }
}

impl Encode for char {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_utf8(encoder.writer(), *self)
    }
}

// impl Encode for &'_ [u8] {
//     fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
//         super::encode_slice_len(encoder, self.len())?;
//         encoder.writer().write(self)
//     }
// }

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;

fn encode_utf8(writer: &mut impl Writer, c: char) -> Result<(), EncodeError> {
    let code = c as u32;

    if code < MAX_ONE_B {
        writer.write(&[c as u8])
    } else if code < MAX_TWO_B {
        let mut buf = [0u8; 2];
        buf[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
        buf[1] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    } else if code < MAX_THREE_B {
        let mut buf = [0u8; 3];
        buf[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
        buf[1] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    } else {
        let mut buf = [0u8; 4];
        buf[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
        buf[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    }
}

// BlockedTODO: https://github.com/rust-lang/rust/issues/37653
//
// We'll want to implement encoding for both &[u8] and &[T: Encode],
// but those implementations overlap because u8 also implements Encode
// impl Encode for &'_ [u8] {
//     fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
//         encoder.writer().write(*self)
//     }
// }

impl<T: Encode> Encode for &'_ [T] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.len().encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl Encode for str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}

impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        if !E::C::SKIP_FIXED_ARRAY_LENGTH {
            super::encode_slice_len(encoder, N)?;
        }
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        super::encode_option_variant(encoder, self)?;
        if let Some(val) = self {
            val.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T, U> Encode for Result<T, U>
where
    T: Encode,
    U: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Ok(val) => {
                0u32.encode(encoder)?;
                val.encode(encoder)
            }
            Err(err) => {
                1u32.encode(encoder)?;
                err.encode(encoder)
            }
        }
    }
}

impl<T> Encode for Cell<T>
where
    T: Encode + Copy,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(&self.get(), encoder)
    }
}

impl<T> Encode for RefCell<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let borrow_guard = self
            .try_borrow()
            .map_err(|e| EncodeError::RefCellAlreadyBorrowed {
                inner: e,
                type_name: core::any::type_name::<RefCell<T>>(),
            })?;
        T::encode(&borrow_guard, encoder)
    }
}

impl Encode for Duration {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_secs().encode(encoder)?;
        self.subsec_nanos().encode(encoder)?;
        Ok(())
    }
}

impl<T> Encode for Range<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.start.encode(encoder)?;
        self.end.encode(encoder)?;
        Ok(())
    }
}

impl<T> Encode for RangeInclusive<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.start().encode(encoder)?;
        self.end().encode(encoder)?;
        Ok(())
    }
}

impl<T> Encode for Bound<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Self::Unbounded => {
                0u32.encode(encoder)?;
            }
            Self::Included(val) => {
                1u32.encode(encoder)?;
                val.encode(encoder)?;
            }
            Self::Excluded(val) => {
                2u32.encode(encoder)?;
                val.encode(encoder)?;
            }
        }
        Ok(())
    }
}

impl<'a, T> Encode for &'a T
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}
