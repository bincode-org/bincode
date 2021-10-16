use crate::{de::Decodable, enc::Encodeable};
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize, Ordering,
};

impl Encodeable for AtomicBool {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicBool {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicBool::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicU8 {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicU8 {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU8::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicU16 {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicU16 {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU16::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicU32 {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicU32 {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU32::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicU64 {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicU64 {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU64::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicUsize {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicUsize {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicUsize::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicI8 {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicI8 {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI8::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicI16 {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicI16 {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI16::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicI32 {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicI32 {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI32::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicI64 {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicI64 {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI64::new(Decodable::decode(decoder)?))
    }
}

impl Encodeable for AtomicIsize {
    fn encode<E: crate::enc::Encode>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decodable for AtomicIsize {
    fn decode<D: crate::de::Decode>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicIsize::new(Decodable::decode(decoder)?))
    }
}
