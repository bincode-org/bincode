use core::marker::PhantomData;

use crate::{
    config::{Config, Endian},
    error::Error,
};

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error>;
}

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

impl Encodeable for u32 {
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), Error> {
        encoder.encode_u32(*self)
    }
}

pub struct SliceWriter<'storage> {
    slice: &'storage mut [u8],
    cursor: usize,
}

impl<'storage> SliceWriter<'storage> {
    pub(crate) fn new(bytes: &'storage mut [u8]) -> SliceWriter<'storage> {
        SliceWriter {
            slice: bytes,
            cursor: 0,
        }
    }
}

impl<'storage> Writer for SliceWriter<'storage> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        if bytes.len() - self.cursor > self.slice.len() {
            return Err(Error::UnexpectedEnd);
        }
        let temp = &mut self.slice[self.cursor..bytes.len()];

        temp.copy_from_slice(bytes);
        self.cursor += bytes.len();

        Ok(())
    }
}
