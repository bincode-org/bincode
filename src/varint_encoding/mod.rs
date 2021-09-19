mod signed;
mod unsigned;

pub use self::signed::{
    varint_encode_i128, varint_encode_i16, varint_encode_i32, varint_encode_i64,
    varint_encode_isize,
};
pub use self::unsigned::{
    varint_encode_u128, varint_encode_u16, varint_encode_u32, varint_encode_u64,
    varint_encode_usize,
};

pub(self) const SINGLE_BYTE_MAX: u8 = 250;
pub(self) const U16_BYTE: u8 = 251;
pub(self) const U32_BYTE: u8 = 252;
pub(self) const U64_BYTE: u8 = 253;
pub(self) const U128_BYTE: u8 = 254;