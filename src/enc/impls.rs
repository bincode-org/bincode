use super::{Encode, Encodeable};
use crate::error::Error;

impl Encodeable for u32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u32(*self)
    }
}