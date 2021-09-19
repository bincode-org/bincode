use super::{Decodable, Decode};
use crate::error::DecodeError;

impl Decodable for u32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        decoder.decode_u32()
    }
}
