use super::{
    read::{BorrowReader, Reader},
    BorrowDecode, BorrowDecoder, Decode, Decoder,
};
use crate::{
    config::{
        Endian, IntEncoding, InternalArrayLengthConfig, InternalEndianConfig,
        InternalIntEncodingConfig,
    },
    error::{DecodeError, IntegerType},
    impl_borrow_decode,
};
use core::{
    any::TypeId,
    cell::{Cell, RefCell},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Bound, Range, RangeInclusive},
    time::Duration,
};

impl Decode for bool {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match u8::decode(decoder)? {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(DecodeError::InvalidBooleanValue(x)),
        }
    }
}
impl_borrow_decode!(bool);

impl Decode for u8 {
    #[inline]
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(1)?;
        if let Some(buf) = decoder.reader().peek_read(1) {
            let byte = buf[0];
            decoder.reader().consume(1);
            Ok(byte)
        } else {
            let mut bytes = [0u8; 1];
            decoder.reader().read(&mut bytes)?;
            Ok(bytes[0])
        }
    }
}
impl_borrow_decode!(u8);

impl Decode for NonZeroU8 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU8::new(u8::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::U8,
        })
    }
}
impl_borrow_decode!(NonZeroU8);

impl Decode for u16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(2)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u16(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => u16::from_le_bytes(bytes),
                    Endian::Big => u16::from_be_bytes(bytes),
                })
            }
        }
    }
}
impl_borrow_decode!(u16);

impl Decode for NonZeroU16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU16::new(u16::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::U16,
        })
    }
}
impl_borrow_decode!(NonZeroU16);

impl Decode for u32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(4)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u32(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => u32::from_le_bytes(bytes),
                    Endian::Big => u32::from_be_bytes(bytes),
                })
            }
        }
    }
}
impl_borrow_decode!(u32);

impl Decode for NonZeroU32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU32::new(u32::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::U32,
        })
    }
}
impl_borrow_decode!(NonZeroU32);

impl Decode for u64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u64(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => u64::from_le_bytes(bytes),
                    Endian::Big => u64::from_be_bytes(bytes),
                })
            }
        }
    }
}
impl_borrow_decode!(u64);

impl Decode for NonZeroU64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU64::new(u64::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::U64,
        })
    }
}
impl_borrow_decode!(NonZeroU64);

impl Decode for u128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(16)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u128(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => u128::from_le_bytes(bytes),
                    Endian::Big => u128::from_be_bytes(bytes),
                })
            }
        }
    }
}
impl_borrow_decode!(u128);

impl Decode for NonZeroU128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU128::new(u128::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::U128,
        })
    }
}
impl_borrow_decode!(NonZeroU128);

impl Decode for usize {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_usize(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;

                let value = match D::C::ENDIAN {
                    Endian::Little => u64::from_le_bytes(bytes),
                    Endian::Big => u64::from_be_bytes(bytes),
                };

                value
                    .try_into()
                    .map_err(|_| DecodeError::OutsideUsizeRange(value))
            }
        }
    }
}
impl_borrow_decode!(usize);

impl Decode for NonZeroUsize {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroUsize::new(usize::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::Usize,
        })
    }
}
impl_borrow_decode!(NonZeroUsize);

impl Decode for i8 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(1)?;
        let mut bytes = [0u8; 1];
        decoder.reader().read(&mut bytes)?;
        Ok(bytes[0] as i8)
    }
}
impl_borrow_decode!(i8);

impl Decode for NonZeroI8 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI8::new(i8::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::I8,
        })
    }
}
impl_borrow_decode!(NonZeroI8);

impl Decode for i16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(2)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i16(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => i16::from_le_bytes(bytes),
                    Endian::Big => i16::from_be_bytes(bytes),
                })
            }
        }
    }
}
impl_borrow_decode!(i16);

impl Decode for NonZeroI16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI16::new(i16::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::I16,
        })
    }
}
impl_borrow_decode!(NonZeroI16);

impl Decode for i32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(4)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i32(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => i32::from_le_bytes(bytes),
                    Endian::Big => i32::from_be_bytes(bytes),
                })
            }
        }
    }
}
impl_borrow_decode!(i32);

impl Decode for NonZeroI32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI32::new(i32::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::I32,
        })
    }
}
impl_borrow_decode!(NonZeroI32);

impl Decode for i64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i64(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => i64::from_le_bytes(bytes),
                    Endian::Big => i64::from_be_bytes(bytes),
                })
            }
        }
    }
}
impl_borrow_decode!(i64);

impl Decode for NonZeroI64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI64::new(i64::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::I64,
        })
    }
}
impl_borrow_decode!(NonZeroI64);

impl Decode for i128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(16)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i128(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => i128::from_le_bytes(bytes),
                    Endian::Big => i128::from_be_bytes(bytes),
                })
            }
        }
    }
}
impl_borrow_decode!(i128);

impl Decode for NonZeroI128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI128::new(i128::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::I128,
        })
    }
}
impl_borrow_decode!(NonZeroI128);

impl Decode for isize {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_isize(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endian::Little => i64::from_le_bytes(bytes),
                    Endian::Big => i64::from_be_bytes(bytes),
                } as isize)
            }
        }
    }
}
impl_borrow_decode!(isize);

impl Decode for NonZeroIsize {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroIsize::new(isize::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::Isize,
        })
    }
}
impl_borrow_decode!(NonZeroIsize);

impl Decode for f32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(4)?;
        let mut bytes = [0u8; 4];
        decoder.reader().read(&mut bytes)?;
        Ok(match D::C::ENDIAN {
            Endian::Little => f32::from_le_bytes(bytes),
            Endian::Big => f32::from_be_bytes(bytes),
        })
    }
}
impl_borrow_decode!(f32);

impl Decode for f64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        let mut bytes = [0u8; 8];
        decoder.reader().read(&mut bytes)?;
        Ok(match D::C::ENDIAN {
            Endian::Little => f64::from_le_bytes(bytes),
            Endian::Big => f64::from_be_bytes(bytes),
        })
    }
}
impl_borrow_decode!(f64);

impl Decode for char {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut array = [0u8; 4];

        // Look at the first byte to see how many bytes must be read
        decoder.reader().read(&mut array[..1])?;

        let width = utf8_char_width(array[0]);
        if width == 0 {
            return Err(DecodeError::InvalidCharEncoding(array));
        }
        // Normally we have to `.claim_bytes_read` before reading, however in this
        // case the amount of bytes read from `char` can vary wildly, and it should
        // only read up to 4 bytes too much.
        decoder.claim_bytes_read(width)?;
        if width == 1 {
            return Ok(array[0] as char);
        }

        // read the remaining pain
        decoder.reader().read(&mut array[1..width])?;
        let res = core::str::from_utf8(&array[..width])
            .ok()
            .and_then(|s| s.chars().next())
            .ok_or(DecodeError::InvalidCharEncoding(array))?;
        Ok(res)
    }
}
impl_borrow_decode!(char);

impl<'a, 'de: 'a> BorrowDecode<'de> for &'a [u8] {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = super::decode_slice_len(decoder)?;
        decoder.claim_bytes_read(len)?;
        decoder.borrow_reader().take_bytes(len)
    }
}

impl<'a, 'de: 'a> BorrowDecode<'de> for &'a str {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let slice = <&[u8]>::borrow_decode(decoder)?;
        core::str::from_utf8(slice).map_err(|inner| DecodeError::Utf8 { inner })
    }
}

impl<T, const N: usize> Decode for [T; N]
where
    T: Decode + Sized + 'static,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        if !D::C::SKIP_FIXED_ARRAY_LENGTH {
            let length = super::decode_slice_len(decoder)?;
            if length != N {
                return Err(DecodeError::ArrayLengthMismatch {
                    found: length,
                    required: N,
                });
            }
        }

        decoder.claim_bytes_read(core::mem::size_of::<[T; N]>())?;

        // Optimize for `[u8; N]`
        if TypeId::of::<u8>() == TypeId::of::<T>() {
            let mut buf = [0u8; N];
            decoder.reader().read(&mut buf)?;
            let ptr = &mut buf as *mut _ as *mut [T; N];

            // Safety: we know that T is a u8, so it is perfectly safe to
            // translate an array of u8 into an array of T
            let res = unsafe { ptr.read() };
            Ok(res)
        } else {
            let result = super::impl_core::collect_into_array(&mut (0..N).map(|_| {
                // See the documentation on `unclaim_bytes_read` as to why we're doing this here
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                T::decode(decoder)
            }));

            // result is only None if N does not match the values of `(0..N)`, which it always should
            // So this unwrap should never occur
            result.unwrap()
        }
    }
}

impl<'de, T, const N: usize> BorrowDecode<'de> for [T; N]
where
    T: BorrowDecode<'de> + Sized + 'static,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        if !D::C::SKIP_FIXED_ARRAY_LENGTH {
            let length = super::decode_slice_len(decoder)?;
            if length != N {
                return Err(DecodeError::ArrayLengthMismatch {
                    found: length,
                    required: N,
                });
            }
        }

        decoder.claim_bytes_read(core::mem::size_of::<[T; N]>())?;

        // Optimize for `[u8; N]`
        if TypeId::of::<u8>() == TypeId::of::<T>() {
            let mut buf = [0u8; N];
            decoder.reader().read(&mut buf)?;
            let ptr = &mut buf as *mut _ as *mut [T; N];

            // Safety: we know that T is a u8, so it is perfectly safe to
            // translate an array of u8 into an array of T
            let res = unsafe { ptr.read() };
            Ok(res)
        } else {
            let result = super::impl_core::collect_into_array(&mut (0..N).map(|_| {
                // See the documentation on `unclaim_bytes_read` as to why we're doing this here
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                T::borrow_decode(decoder)
            }));

            // result is only None if N does not match the values of `(0..N)`, which it always should
            // So this unwrap should never occur
            result.unwrap()
        }
    }
}

impl Decode for () {
    fn decode<D: Decoder>(_: &mut D) -> Result<Self, DecodeError> {
        Ok(())
    }
}
impl_borrow_decode!(());

impl<T> Decode for core::marker::PhantomData<T> {
    fn decode<D: Decoder>(_: &mut D) -> Result<Self, DecodeError> {
        Ok(core::marker::PhantomData)
    }
}
impl<'de, T> BorrowDecode<'de> for core::marker::PhantomData<T> {
    fn borrow_decode<D: BorrowDecoder<'de>>(_: &mut D) -> Result<Self, DecodeError> {
        Ok(core::marker::PhantomData)
    }
}

impl<T> Decode for Option<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match super::decode_option_variant(decoder, core::any::type_name::<Option<T>>())? {
            Some(_) => {
                let val = T::decode(decoder)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }
}

impl<'de, T> BorrowDecode<'de> for Option<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        match super::decode_option_variant(decoder, core::any::type_name::<Option<T>>())? {
            Some(_) => {
                let val = T::borrow_decode(decoder)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }
}

// BlockedTODO: https://github.com/rust-lang/rust/issues/37653
//
// We'll want to implement BorrowDecode for both Option<&[u8]> and Option<&[T: Encode]>,
// but those implementations overlap because &'a [u8] also implements BorrowDecode
// impl<'a, 'de: 'a> BorrowDecode<'de> for Option<&'a [u8]> {
//     fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
//         match super::decode_option_variant(decoder, core::any::type_name::<Option<&[u8]>>())? {
//             Some(_) => {
//                 let val = BorrowDecode::borrow_decode(decoder)?;
//                 Ok(Some(val))
//             }
//             None => Ok(None),
//         }
//     }
// }

impl<T, U> Decode for Result<T, U>
where
    T: Decode,
    U: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let is_ok = u32::decode(decoder)?;
        match is_ok {
            0 => {
                let t = T::decode(decoder)?;
                Ok(Ok(t))
            }
            1 => {
                let u = U::decode(decoder)?;
                Ok(Err(u))
            }
            x => Err(DecodeError::UnexpectedVariant {
                found: x as u32,
                allowed: crate::error::AllowedEnumVariants::Range { max: 1, min: 0 },
                type_name: core::any::type_name::<Result<T, U>>(),
            }),
        }
    }
}

impl<'de, T, U> BorrowDecode<'de> for Result<T, U>
where
    T: BorrowDecode<'de>,
    U: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let is_ok = u32::decode(decoder)?;
        match is_ok {
            0 => {
                let t = T::borrow_decode(decoder)?;
                Ok(Ok(t))
            }
            1 => {
                let u = U::borrow_decode(decoder)?;
                Ok(Err(u))
            }
            x => Err(DecodeError::UnexpectedVariant {
                found: x as u32,
                allowed: crate::error::AllowedEnumVariants::Range { max: 1, min: 0 },
                type_name: core::any::type_name::<Result<T, U>>(),
            }),
        }
    }
}

impl<T> Decode for Cell<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Cell::new(t))
    }
}

impl<'de, T> BorrowDecode<'de> for Cell<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Cell::new(t))
    }
}

impl<T> Decode for RefCell<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(RefCell::new(t))
    }
}

impl<'de, T> BorrowDecode<'de> for RefCell<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(RefCell::new(t))
    }
}

impl Decode for Duration {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        const NANOS_PER_SEC: u64 = 1_000_000_000;
        let secs: u64 = Decode::decode(decoder)?;
        let nanos: u32 = Decode::decode(decoder)?;
        if secs.checked_add(u64::from(nanos) / NANOS_PER_SEC).is_none() {
            return Err(DecodeError::InvalidDuration { secs, nanos });
        }
        Ok(Duration::new(secs, nanos))
    }
}
impl_borrow_decode!(Duration);

impl<T> Decode for Range<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let min = T::decode(decoder)?;
        let max = T::decode(decoder)?;
        Ok(min..max)
    }
}
impl<'de, T> BorrowDecode<'de> for Range<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let min = T::borrow_decode(decoder)?;
        let max = T::borrow_decode(decoder)?;
        Ok(min..max)
    }
}

impl<T> Decode for RangeInclusive<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let min = T::decode(decoder)?;
        let max = T::decode(decoder)?;
        Ok(RangeInclusive::new(min, max))
    }
}

impl<'de, T> BorrowDecode<'de> for RangeInclusive<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let min = T::borrow_decode(decoder)?;
        let max = T::borrow_decode(decoder)?;
        Ok(RangeInclusive::new(min, max))
    }
}

impl<T> Decode for Bound<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match u32::decode(decoder)? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(T::decode(decoder)?)),
            2 => Ok(Bound::Excluded(T::decode(decoder)?)),
            x => Err(DecodeError::UnexpectedVariant {
                allowed: crate::error::AllowedEnumVariants::Range { max: 2, min: 0 },
                found: x,
                type_name: core::any::type_name::<Bound<T>>(),
            }),
        }
    }
}

impl<'de, T> BorrowDecode<'de> for Bound<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        match u32::decode(decoder)? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(T::borrow_decode(decoder)?)),
            2 => Ok(Bound::Excluded(T::borrow_decode(decoder)?)),
            x => Err(DecodeError::UnexpectedVariant {
                allowed: crate::error::AllowedEnumVariants::Range { max: 2, min: 0 },
                found: x,
                type_name: core::any::type_name::<Bound<T>>(),
            }),
        }
    }
}

const UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x1F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x3F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x5F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x7F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0xBF
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];

// This function is a copy of core::str::utf8_char_width
const fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
