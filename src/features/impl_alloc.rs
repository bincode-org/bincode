use crate::{
    de::{Decode, Decoder},
    enc::{self, Encode, Encoder},
    error::{DecodeError, EncodeError},
    Config,
};
#[cfg(feature = "atomic")]
use alloc::sync::Arc;
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::*,
    rc::Rc,
    string::String,
    vec::Vec,
};

#[derive(Default)]
pub(crate) struct VecWriter {
    inner: Vec<u8>,
}

impl VecWriter {
    // May not be used in all feature combinations
    #[allow(dead_code)]
    pub(crate) fn collect(self) -> Vec<u8> {
        self.inner
    }
}

impl enc::write::Writer for VecWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.inner.extend_from_slice(bytes);
        Ok(())
    }
}

/// Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn encode_to_vec<E: enc::Encode, C: Config>(val: E, config: C) -> Result<Vec<u8>, EncodeError> {
    let writer = VecWriter::default();
    let mut encoder = enc::EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().inner)
}

impl<T> Decode for BinaryHeap<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(&mut decoder)?;
        let mut map = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            let key = T::decode(&mut decoder)?;
            map.push(key);
        }
        Ok(map)
    }
}

impl<T> Encode for BinaryHeap<T>
where
    T: Encode + Ord,
{
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(&mut encoder, self.len())?;
        for val in self.iter() {
            val.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl<K, V> Decode for BTreeMap<K, V>
where
    K: Decode + Ord,
    V: Decode,
{
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(&mut decoder)?;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            let key = K::decode(&mut decoder)?;
            let value = V::decode(&mut decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl<K, V> Encode for BTreeMap<K, V>
where
    K: Encode + Ord,
    V: Encode,
{
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(&mut encoder, self.len())?;
        for (key, val) in self.iter() {
            key.encode(&mut encoder)?;
            val.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl<T> Decode for BTreeSet<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(&mut decoder)?;
        let mut map = BTreeSet::new();
        for _ in 0..len {
            let key = T::decode(&mut decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}

impl<T> Encode for BTreeSet<T>
where
    T: Encode + Ord,
{
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(&mut encoder, self.len())?;
        for item in self.iter() {
            item.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl<T> Decode for VecDeque<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(&mut decoder)?;
        let mut map = VecDeque::with_capacity(len);
        for _ in 0..len {
            let key = T::decode(&mut decoder)?;
            map.push_back(key);
        }
        Ok(map)
    }
}

impl<T> Encode for VecDeque<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(&mut encoder, self.len())?;
        for item in self.iter() {
            item.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl<T> Decode for Vec<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(&mut decoder)?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(&mut decoder)?);
        }
        Ok(vec)
    }
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(&mut encoder, self.len())?;
        for item in self.iter() {
            item.encode(&mut encoder)?;
        }
        Ok(())
    }
}

impl Decode for String {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let bytes = Vec::<u8>::decode(decoder)?;
        String::from_utf8(bytes).map_err(|e| DecodeError::Utf8(e.utf8_error()))
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}

impl<T> Decode for Box<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Box::new(t))
    }
}

impl<T> Encode for Box<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

impl<T> Decode for Box<[T]>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let vec = Vec::decode(decoder)?;
        Ok(vec.into_boxed_slice())
    }
}

impl<T> Encode for Box<[T]>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(&mut encoder, self.len())?;
        for item in self.iter() {
            item.encode(&mut encoder)?;
        }
        Ok(())
    }
}

// BlockedTODO: https://github.com/rust-lang/rust/issues/31844
// Cow should be able to decode a borrowed value
// Currently this conflicts with the owned `Decode` implementation below

// impl<'cow, T> BorrowDecode<'cow> for Cow<'cow, T>
// where
//     T: BorrowDecode<'cow>,
// {
//     fn borrow_decode<D: crate::de::BorrowDecoder<'cow>>(decoder: D) -> Result<Self, DecodeError> {
//         let t = T::borrow_decode(decoder)?;
//         Ok(Cow::Borrowed(t))
//     }
// }

impl<'cow, T> Decode for Cow<'cow, T>
where
    T: ToOwned,
    <T as ToOwned>::Owned: Decode,
{
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let t = <T as ToOwned>::Owned::decode(decoder)?;
        Ok(Cow::Owned(t))
    }
}

impl<'cow, T> Encode for Cow<'cow, T>
where
    T: Encode + Clone,
{
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        self.as_ref().encode(encoder)
    }
}

impl<T> Decode for Rc<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Rc::new(t))
    }
}

impl<T> Encode for Rc<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

#[cfg(feature = "atomic")]
impl<T> Decode for Arc<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Arc::new(t))
    }
}

#[cfg(feature = "atomic")]
impl<T> Encode for Arc<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}
