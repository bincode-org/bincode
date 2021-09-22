use crate::error::DecodeError;

mod decoder;
mod impls;

pub mod read;
pub use self::decoder::Decoder;

pub trait Decodable<'de>: Sized + BorrowDecodable<'de> {
    fn decode<D: Decode<'de>>(decoder: D) -> Result<Self, DecodeError>;
}

pub trait BorrowDecodable<'de>: Sized {
    fn borrow_decode<D: BorrowDecode<'de>>(decoder: D) -> Result<Self, DecodeError>;
}

impl<'de, T: Decodable<'de>> BorrowDecodable<'de> for T {
    fn borrow_decode<D: Decode<'de>>(decoder: D) -> Result<Self, DecodeError> {
        Decodable::decode(decoder)
    }
}

pub trait Decode<'de> {
    fn decode_u8(&mut self) -> Result<u8, DecodeError>;
    fn decode_u16(&mut self) -> Result<u16, DecodeError>;
    fn decode_u32(&mut self) -> Result<u32, DecodeError>;
    fn decode_u64(&mut self) -> Result<u64, DecodeError>;
    fn decode_u128(&mut self) -> Result<u128, DecodeError>;
    fn decode_usize(&mut self) -> Result<usize, DecodeError>;

    fn decode_i8(&mut self) -> Result<i8, DecodeError>;
    fn decode_i16(&mut self) -> Result<i16, DecodeError>;
    fn decode_i32(&mut self) -> Result<i32, DecodeError>;
    fn decode_i64(&mut self) -> Result<i64, DecodeError>;
    fn decode_i128(&mut self) -> Result<i128, DecodeError>;
    fn decode_isize(&mut self) -> Result<isize, DecodeError>;

    fn decode_f32(&mut self) -> Result<f32, DecodeError>;
    fn decode_f64(&mut self) -> Result<f64, DecodeError>;
    fn decode_array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError>;
}

pub trait BorrowDecode<'de>: Decode<'de> {
    fn decode_slice(&mut self, len: usize) -> Result<&'de [u8], DecodeError>;
}
