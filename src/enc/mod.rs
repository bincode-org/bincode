mod encoder;
mod impls;

use crate::error::EncodeError;

pub mod write;

pub use self::encoder::Encoder;

pub trait Encodeable {
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError>;
}

pub trait Encode {
    fn encode_u8(&mut self, val: u8) -> Result<(), EncodeError>;
    fn encode_u16(&mut self, val: u16) -> Result<(), EncodeError>;
    fn encode_u32(&mut self, val: u32) -> Result<(), EncodeError>;
    fn encode_u64(&mut self, val: u64) -> Result<(), EncodeError>;
    fn encode_u128(&mut self, val: u128) -> Result<(), EncodeError>;
    fn encode_usize(&mut self, val: usize) -> Result<(), EncodeError>;

    fn encode_i8(&mut self, val: i8) -> Result<(), EncodeError>;
    fn encode_i16(&mut self, val: i16) -> Result<(), EncodeError>;
    fn encode_i32(&mut self, val: i32) -> Result<(), EncodeError>;
    fn encode_i64(&mut self, val: i64) -> Result<(), EncodeError>;
    fn encode_i128(&mut self, val: i128) -> Result<(), EncodeError>;
    fn encode_isize(&mut self, val: isize) -> Result<(), EncodeError>;

    fn encode_f32(&mut self, val: f32) -> Result<(), EncodeError>;
    fn encode_f64(&mut self, val: f64) -> Result<(), EncodeError>;
    fn encode_slice(&mut self, val: &[u8]) -> Result<(), EncodeError>;
}
