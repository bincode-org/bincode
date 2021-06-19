use core::marker::PhantomData;

use crate::{
    config::{Config, Endian},
    error::Error,
};

pub trait Reader<'storage> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error>;
    fn forward_read<F, R>(&mut self, length: usize, visitor: F) -> Result<R, Error>
    where
        F: Fn(&'storage [u8]) -> R;
}

pub trait Decodable: Sized {
    fn decode<D: Decode>(decoder: D) -> Result<Self, Error>;
}

pub trait Decode {
    fn decode_u32(&mut self) -> Result<u32, Error>;
}

pub struct Decoder<R, C: Config> {
    reader: R,
    config: PhantomData<C>,
}

impl<'de, R: Reader<'de>, C: Config> Decoder<R, C> {
    pub fn new(reader: R) -> Decoder<R, C> {
        Decoder {
            reader,
            config: PhantomData,
        }
    }
}

impl<'a, 'de, R: Reader<'de>, C: Config> Decode for &'a mut Decoder<R, C> {
    fn decode_u32(&mut self) -> Result<u32, Error> {
        let mut bytes = [0u8; 4];

        self.reader.read(bytes.as_mut())?;
        Ok(match C::ENDIAN {
            Endian::Little => u32::from_le_bytes(bytes),
            Endian::Big => u32::from_be_bytes(bytes),
        })
    }
}

impl Decodable for u32 {
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, Error> {
        decoder.decode_u32()
    }
}

pub struct SliceReader<'storage> {
    slice: &'storage [u8],
}

impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub(crate) fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader { slice: bytes }
    }

    #[inline(always)]
    fn get_byte_slice(&mut self, length: usize) -> Result<&'storage [u8], Error> {
        if length > self.slice.len() {
            return Err(Error::UnexpectedEnd);
        }
        let (read_slice, remaining) = self.slice.split_at(length);
        self.slice = remaining;
        Ok(read_slice)
    }
}

impl<'storage> Reader<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        if bytes.len() > self.slice.len() {
            return Err(Error::UnexpectedEnd);
        }
        let (read_slice, remaining) = self.slice.split_at(bytes.len());
        bytes.copy_from_slice(read_slice);
        self.slice = remaining;

        Ok(())
    }

    #[inline(always)]
    fn forward_read<F, R>(&mut self, length: usize, visitor: F) -> Result<R, Error>
    where
        F: Fn(&'storage [u8]) -> R,
    {
        Ok(visitor(self.get_byte_slice(length)?))
    }
}
