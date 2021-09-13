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
    fn encode_u8(&mut self, val: u8) -> Result<(), Error>;
    fn encode_u32(&mut self, val: u32) -> Result<(), Error>;
    fn encode_i32(&mut self, val: i32) -> Result<(), Error>;
}

impl<'a, T> Encode for &'a mut T
where
    T: Encode,
{
    fn encode_u8(&mut self, val: u8) -> Result<(), Error> {
        T::encode_u8(self, val)
    }
    fn encode_u32(&mut self, val: u32) -> Result<(), Error> {
        T::encode_u32(self, val)
    }
    fn encode_i32(&mut self, val: i32) -> Result<(), Error> {
        T::encode_i32(self, val)
    }
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

    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<'a, W: Writer, C: Config> Encode for &'a mut Encoder<W, C> {
    fn encode_u8(&mut self, val: u8) -> Result<(), Error> {
        self.writer.write(&[val])
    }

    fn encode_u32(&mut self, val: u32) -> Result<(), Error> {
        let bytes = match C::ENDIAN {
            Endian::Little => val.to_le_bytes(),
            Endian::Big => val.to_be_bytes(),
        };

        self.writer.write(&bytes)
    }

    fn encode_i32(&mut self, val: i32) -> Result<(), Error> {
        let bytes = match C::ENDIAN {
            Endian::Little => val.to_le_bytes(),
            Endian::Big => val.to_be_bytes(),
        };

        self.writer.write(&bytes)
    }
}
