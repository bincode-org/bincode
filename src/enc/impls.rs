use super::{Encode, Encodeable};
use crate::error::Error;

impl Encodeable for u8 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u8(*self)
    }
}

impl Encodeable for u32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u32(*self)
    }
}

impl Encodeable for i32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_i32(*self)
    }
}
