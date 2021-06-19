use core::marker::PhantomData;

use crate::{
    config::{Config, Endian},
    error::Error,
};
use write::Writer;

mod impls;
pub mod write;


pub trait Encodeable {
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), Error>;
}

pub trait Encode {
    fn encode_u32(&mut self, val: u32) -> Result<(), Error>;
}

pub struct Encoder<W: Writer, C: Config> {
    writer: W,
    config: PhantomData<C>,
}

impl<W: Writer, C: Config> Encoder<W, C> {
    pub fn new(writer: W) -> Encoder<W, C> {
        Encoder {
            writer,
            config: PhantomData,
        }
    }
}

impl<'a, W: Writer, C: Config> Encode for &'a mut Encoder<W, C> {
    fn encode_u32(&mut self, val: u32) -> Result<(), Error> {
        let bytes = match C::ENDIAN {
            Endian::Little => val.to_le_bytes(),
            Endian::Big => val.to_be_bytes(),
        };

        self.writer.write(&bytes)
    }
}