mod de_borrowed;
mod de_owned;
mod ser;

pub use self::de_borrowed::*;
pub use self::de_owned::*;
pub use self::ser::*;

impl serde_incl::de::Error for crate::error::DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        use alloc::string::ToString;
        Self::OtherString(msg.to_string())
    }
}

impl serde_incl::ser::Error for crate::error::EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        use alloc::string::ToString;

        Self::OtherString(msg.to_string())
    }
}

/// Wrapper struct that implements [Decode] and [Encode] on any type that implements serde's [DeserializeOwned] and [Serialize] respectively.
pub struct SerdeToBincode<T>(pub T);

impl<T> crate::Decode for SerdeToBincode<T>
where
    T: serde_incl::de::DeserializeOwned,
{
    fn decode<D: crate::de::Decoder>(decoder: D) -> Result<Self, crate::error::DecodeError> {
        let t: T = __serde_decode_field(decoder)?;
        Ok(Self(t))
    }
}

impl<T> crate::Encode for SerdeToBincode<T>
where
    T: serde_incl::Serialize,
{
    fn encode<E: crate::enc::Encoder>(&self, encoder: E) -> Result<(), crate::error::EncodeError> {
        __serde_encode_field(&self.0, encoder)
    }
}

#[doc(hidden)]
pub fn __serde_encode_field<T, E>(value: T, mut encoder: E) -> Result<(), crate::error::EncodeError>
where
    T: serde_incl::Serialize,
    E: crate::enc::Encoder,
{
    let serializer = ser::SerdeEncoder { enc: &mut encoder };
    value.serialize(serializer)?;
    Ok(())
}

#[doc(hidden)]
pub fn __serde_decode_field<T, D>(mut decoder: D) -> Result<T, crate::error::DecodeError>
where
    T: serde_incl::de::DeserializeOwned,
    D: crate::de::Decoder,
{
    let serde_decoder = de_owned::SerdeDecoder { de: &mut decoder };
    T::deserialize(serde_decoder)
}

#[doc(hidden)]
pub fn __serde_decode_borrow_field<'de, T, D>(
    mut decoder: D,
) -> Result<T, crate::error::DecodeError>
where
    T: serde_incl::de::Deserialize<'de>,
    D: crate::de::BorrowDecoder<'de> + 'de,
{
    let serde_decoder = de_borrowed::SerdeDecoder {
        de: &mut decoder,
        pd: core::marker::PhantomData,
    };
    T::deserialize(serde_decoder)
}
