use crate::de::BorrowDecoder;
use crate::de::Decode;
use crate::de::Decoder;
use crate::enc::Encode;
use crate::enc::Encoder;
use crate::error::DecodeError;
use crate::error::EncodeError;
use crate::BorrowDecode;
use smallvec::{Array, SmallVec};

impl<A> Decode for SmallVec<A>
where
    A: Array,
    A::Item: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<A::Item>(len)?;

        let mut vec = SmallVec::with_capacity(len);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<A::Item>());

            vec.push(A::Item::decode(decoder)?);
        }
        Ok(vec)
    }
}

impl<'de, A> BorrowDecode<'de> for SmallVec<A>
where
    A: Array,
    A::Item: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<A::Item>(len)?;

        let mut vec = SmallVec::with_capacity(len);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<A::Item>());

            vec.push(A::Item::borrow_decode(decoder)?);
        }
        Ok(vec)
    }
}

impl<A> Encode for SmallVec<A>
where
    A: Array,
    A::Item: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}
