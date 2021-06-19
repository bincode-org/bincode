use core::marker::PhantomData;

use crate::{
    config::{Config, Endian},
    error::Error,
};
use read::Reader;

mod impls;
pub mod read;



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


