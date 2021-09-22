use super::{Decodable, Decode};
use crate::error::DecodeError;

impl Decodable for u8 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u8()
    }
}

impl Decodable for u16 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u16()
    }
}

impl Decodable for u32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u32()
    }
}

impl Decodable for u64 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u64()
    }
}

impl Decodable for u128 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u128()
    }
}

impl Decodable for usize {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_usize()
    }
}

impl Decodable for i8 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i8()
    }
}

impl Decodable for i16 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i16()
    }
}

impl Decodable for i32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i32()
    }
}

impl Decodable for i64 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i64()
    }
}

impl Decodable for i128 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_i128()
    }
}

impl Decodable for isize {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_isize()
    }
}

impl Decodable for f32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_f32()
    }
}

impl Decodable for f64 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_f64()
    }
}

impl<const N: usize> Decodable for [u8; N] {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_array()
    }
}

impl<'a, T> Decode for &'a mut T
where
    T: Decode,
{
    fn decode_u8(&mut self) -> Result<u8, DecodeError> {
        T::decode_u8(self)
    }

    fn decode_u16(&mut self) -> Result<u16, DecodeError> {
        T::decode_u16(self)
    }

    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        T::decode_u32(self)
    }

    fn decode_u64(&mut self) -> Result<u64, DecodeError> {
        T::decode_u64(self)
    }

    fn decode_u128(&mut self) -> Result<u128, DecodeError> {
        T::decode_u128(self)
    }

    fn decode_usize(&mut self) -> Result<usize, DecodeError> {
        T::decode_usize(self)
    }

    fn decode_i8(&mut self) -> Result<i8, DecodeError> {
        T::decode_i8(self)
    }

    fn decode_i16(&mut self) -> Result<i16, DecodeError> {
        T::decode_i16(self)
    }

    fn decode_i32(&mut self) -> Result<i32, DecodeError> {
        T::decode_i32(self)
    }

    fn decode_i64(&mut self) -> Result<i64, DecodeError> {
        T::decode_i64(self)
    }

    fn decode_i128(&mut self) -> Result<i128, DecodeError> {
        T::decode_i128(self)
    }

    fn decode_isize(&mut self) -> Result<isize, DecodeError> {
        T::decode_isize(self)
    }

    fn decode_f32(&mut self) -> Result<f32, DecodeError> {
        T::decode_f32(self)
    }

    fn decode_f64(&mut self) -> Result<f64, DecodeError> {
        T::decode_f64(self)
    }

    fn decode_slice(&mut self, slice: &mut [u8]) -> Result<(), DecodeError> {
        T::decode_slice(self, slice)
    }

    fn decode_array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        T::decode_array::<N>(self)
    }
}
