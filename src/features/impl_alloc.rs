use crate::{
    config,
    de::{Decodable, Decode},
    enc::{self, Encode, Encodeable},
    error::{DecodeError, EncodeError},
    Config,
};
use alloc::{borrow::Cow, boxed::Box, collections::*, rc::Rc, string::String, sync::Arc, vec::Vec};

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
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn encode_to_vec<E: enc::Encodeable>(val: E) -> Result<Vec<u8>, EncodeError> {
    encode_to_vec_with_config(val, config::Default)
}

/// Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information.
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn encode_to_vec_with_config<E: enc::Encodeable, C: Config>(
    val: E,
    config: C,
) -> Result<Vec<u8>, EncodeError> {
    let writer = VecWriter::default();
    let mut encoder = enc::Encoder::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().inner)
}

impl<T> Decodable for BinaryHeap<T>
where
    T: Decodable + Ord,
{
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = usize::decode(&mut decoder)?;
        let mut map = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            let key = T::decode(&mut decoder)?;
            map.push(key);
        }
        Ok(map)
    }
}

impl<T> Encodeable for BinaryHeap<T>
where
    T: Encodeable + Ord,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.len().encode(&mut encoder)?;
        for val in self.iter() {
            val.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl<K, V> Decodable for BTreeMap<K, V>
where
    K: Decodable + Ord,
    V: Decodable,
{
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = usize::decode(&mut decoder)?;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            let key = K::decode(&mut decoder)?;
            let value = V::decode(&mut decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl<K, V> Encodeable for BTreeMap<K, V>
where
    K: Encodeable + Ord,
    V: Encodeable,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.len().encode(&mut encoder)?;
        for (key, val) in self.iter() {
            key.encode(&mut encoder)?;
            val.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl<T> Decodable for BTreeSet<T>
where
    T: Decodable + Ord,
{
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = usize::decode(&mut decoder)?;
        let mut map = BTreeSet::new();
        for _ in 0..len {
            let key = T::decode(&mut decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}

impl<T> Encodeable for BTreeSet<T>
where
    T: Encodeable + Ord,
{
    fn encode<E: Encode>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.len().encode(&mut encoder)?;
        for item in self.iter() {
            item.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl<T> Decodable for VecDeque<T>
where
    T: Decodable,
{
    fn decode<D: Decode>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = usize::decode(&mut decoder)?;
        let mut map = VecDeque::with_capacity(len);
        for _ in 0..len {
            let key = T::decode(&mut decoder)?;
            map.push_back(key);
        }
        Ok(map)
    }
}

impl<T> Encodeable for VecDeque<T>
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

impl<T> Decodable for Vec<T>
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

impl Decodable for String {
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        let bytes = Vec::<u8>::decode(decoder)?;
        String::from_utf8(bytes).map_err(|e| DecodeError::Utf8(e.utf8_error()))
    }
}

impl Encodeable for String {
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}

impl<T> Decodable for Box<T>
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

impl<T> Decodable for Box<[T]>
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

impl<'cow, T> Decodable for Cow<'cow, T>
where
    T: Decodable + Clone,
{
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Cow::Owned(t))
    }
}

impl<'cow, T> Encodeable for Cow<'cow, T>
where
    T: Encodeable + Clone,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        self.as_ref().encode(encoder)
    }
}

impl<T> Decodable for Rc<T>
where
    T: Decodable,
{
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Rc::new(t))
    }
}

impl<T> Encodeable for Rc<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

#[cfg(feature = "atomic")]
impl<T> Decodable for Arc<T>
where
    T: Decodable,
{
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Arc::new(t))
    }
}

#[cfg(feature = "atomic")]
impl<T> Encodeable for Arc<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}
