use crate::{
    config,
    de::{Decodable, Decode},
    enc::{self, Encode, Encodeable},
    error::{DecodeError, EncodeError},
    Config,
};
use alloc::{boxed::Box, vec::Vec};

#[derive(Default)]
struct VecWriter {
    inner: Vec<u8>,
}

impl enc::write::Writer for VecWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.inner.extend_from_slice(bytes);
        Ok(())
    }
}

/// Encode the given value into a `Vec<u8>`.
pub fn encode_to_vec<E: enc::Encodeable>(val: E) -> Result<Vec<u8>, EncodeError> {
    encode_to_vec_with_config(val, config::Default)
}

/// Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information.
pub fn encode_to_vec_with_config<E: enc::Encodeable, C: Config>(
    val: E,
    config: C,
) -> Result<Vec<u8>, EncodeError> {
    let writer = VecWriter::default();
    let mut encoder = enc::Encoder::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().inner)
}

impl<'de, T> Decodable for Vec<T>
where
    T: Decodable,
{
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = usize::decode(&mut decoder)?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(&mut decoder)?);
        }
        Ok(vec)
    }
}

impl<T> Encodeable for Vec<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.len().encode(&mut encoder)?;
        for item in self.iter() {
            item.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl<'de, T> Decodable for Box<T>
where
    T: Decodable,
{
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Box::new(t))
    }
}

impl<T> Encodeable for Box<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

impl<'de, T> Decodable for Box<[T]>
where
    T: Decodable,
{
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        let vec = Vec::decode(decoder)?;
        Ok(vec.into_boxed_slice())
    }
}

impl<T> Encodeable for Box<[T]>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.len().encode(&mut encoder)?;
        for item in self.iter() {
            item.encode(&mut encoder)?;
        }
        Ok(())
    }
}
