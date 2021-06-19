use crate::{error::Error, enc::write::Writer};

pub trait IntEncoding {
    fn encode_u32<W: Writer>(writer: &mut W, val: u32) -> Result<(), Error>;
}

#[derive(Copy, Clone)]
pub struct FixintEncoding;
