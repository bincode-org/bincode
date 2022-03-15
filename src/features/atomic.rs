use crate::{de::Decode, enc::Encode, impl_borrow_decode};
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize, Ordering,
};

impl Encode for AtomicBool {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicBool {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicBool::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicBool);

impl Encode for AtomicU8 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicU8 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU8::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicU8);

impl Encode for AtomicU16 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicU16 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU16::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicU16);

impl Encode for AtomicU32 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicU32 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU32::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicU32);

impl Encode for AtomicU64 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicU64 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU64::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicU64);

impl Encode for AtomicUsize {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicUsize {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicUsize::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicUsize);

impl Encode for AtomicI8 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicI8 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI8::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicI8);

impl Encode for AtomicI16 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicI16 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI16::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicI16);

impl Encode for AtomicI32 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicI32 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI32::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicI32);

impl Encode for AtomicI64 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicI64 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI64::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicI64);

impl Encode for AtomicIsize {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

impl Decode for AtomicIsize {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicIsize::new(Decode::decode(decoder)?))
    }
}
impl_borrow_decode!(AtomicIsize);
