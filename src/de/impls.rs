use super::{Decode, Decodable};
use crate::error::Error;

impl Decodable for u32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, Error> {
        decoder.decode_u32()
    }
}