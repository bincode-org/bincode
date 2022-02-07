use super::DecodeError as SerdeDecodeError;
use crate::{
    config::Config,
    de::{BorrowDecode, BorrowDecoder, Decode},
    error::DecodeError,
};
use core::marker::PhantomData;
use serde_incl::de::*;

/// Decode a borrowed type from the given slice. Some parts of the decoded type are expected to be referring to the given slice
pub fn decode_borrowed_from_slice<'de, T, C>(slice: &'de [u8], config: C) -> Result<T, DecodeError>
where
    T: Deserialize<'de>,
    C: Config,
{
    if C::SKIP_FIXED_ARRAY_LENGTH {
        return Err(SerdeDecodeError::SkipFixedArrayLengthNotSupported.into());
    }
    let reader = crate::de::read::SliceReader::new(slice);
    let mut decoder = crate::de::DecoderImpl::new(reader, config);
    let serde_decoder = SerdeDecoder {
        de: &mut decoder,
        pd: PhantomData,
    };
    T::deserialize(serde_decoder)
}

pub(super) struct SerdeDecoder<'a, 'de, DE: BorrowDecoder<'de>> {
    pub(super) de: &'a mut DE,
    pub(super) pd: PhantomData<&'de ()>,
}

impl<'a, 'de, DE: BorrowDecoder<'de>> Deserializer<'de> for SerdeDecoder<'a, 'de, DE> {
    type Error = DecodeError;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        Err(SerdeDecodeError::AnyNotSupported.into())
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_bool(Decode::decode(&mut self.de)?)
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_i8(Decode::decode(&mut self.de)?)
    }

    fn deserialize_i16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_i16(Decode::decode(&mut self.de)?)
    }

    fn deserialize_i32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_i32(Decode::decode(&mut self.de)?)
    }

    fn deserialize_i64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_i64(Decode::decode(&mut self.de)?)
    }

    serde_incl::serde_if_integer128! {
        fn deserialize_i128<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde_incl::de::Visitor<'de>,
        {
            visitor.visit_i128(Decode::decode(&mut self.de)?)
        }
    }

    fn deserialize_u8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_u8(Decode::decode(&mut self.de)?)
    }

    fn deserialize_u16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_u16(Decode::decode(&mut self.de)?)
    }

    fn deserialize_u32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_u32(Decode::decode(&mut self.de)?)
    }

    fn deserialize_u64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_u64(Decode::decode(&mut self.de)?)
    }

    serde_incl::serde_if_integer128! {
        fn deserialize_u128<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde_incl::de::Visitor<'de>,
        {
            visitor.visit_u128(Decode::decode(&mut self.de)?)
        }
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_f32(Decode::decode(&mut self.de)?)
    }

    fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_f64(Decode::decode(&mut self.de)?)
    }

    fn deserialize_char<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_char(Decode::decode(&mut self.de)?)
    }

    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        let str = <&'de str>::borrow_decode(&mut self.de)?;
        visitor.visit_borrowed_str(str)
    }

    #[cfg(not(feature = "alloc"))]
    fn deserialize_string<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        Err(SerdeDecodeError::CannotBorrowOwnedData.into())
    }

    #[cfg(feature = "alloc")]
    fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_string(Decode::decode(&mut self.de)?)
    }

    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        let bytes = <&'de [u8]>::borrow_decode(&mut self.de)?;
        visitor.visit_borrowed_bytes(bytes)
    }

    #[cfg(not(feature = "alloc"))]
    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        Err(SerdeDecodeError::CannotBorrowOwnedData.into())
    }

    #[cfg(feature = "alloc")]
    fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_byte_buf(Decode::decode(&mut self.de)?)
    }

    fn deserialize_option<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        let variant = crate::de::decode_option_variant(&mut self.de, "Option<T>")?;
        if variant.is_some() {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        let len = usize::decode(&mut self.de)?;
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        struct Access<'a, 'b, 'de, DE: BorrowDecoder<'de>> {
            deserializer: &'a mut SerdeDecoder<'b, 'de, DE>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a, DE: BorrowDecoder<'de> + 'b> SeqAccess<'de> for Access<'a, 'b, 'de, DE> {
            type Error = DecodeError;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, DecodeError>
            where
                T: DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = DeserializeSeed::deserialize(
                        seed,
                        SerdeDecoder {
                            de: self.deserializer.de,
                            pd: PhantomData,
                        },
                    )?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: &mut self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        struct Access<'a, 'b, 'de, DE: BorrowDecoder<'de>> {
            deserializer: &'a mut SerdeDecoder<'b, 'de, DE>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a, DE: BorrowDecoder<'de> + 'b> MapAccess<'de> for Access<'a, 'b, 'de, DE> {
            type Error = DecodeError;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, DecodeError>
            where
                K: DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let key = DeserializeSeed::deserialize(
                        seed,
                        SerdeDecoder {
                            de: self.deserializer.de,
                            pd: PhantomData,
                        },
                    )?;
                    Ok(Some(key))
                } else {
                    Ok(None)
                }
            }

            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, DecodeError>
            where
                V: DeserializeSeed<'de>,
            {
                let value = DeserializeSeed::deserialize(
                    seed,
                    SerdeDecoder {
                        de: self.deserializer.de,
                        pd: PhantomData,
                    },
                )?;
                Ok(value)
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        let len = usize::decode(&mut self.de)?;

        visitor.visit_map(Access {
            deserializer: &mut self,
            len,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        Err(SerdeDecodeError::IdentifierNotSupported.into())
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        Err(SerdeDecodeError::IgnoredAnyNotSupported.into())
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'de, 'a, DE: BorrowDecoder<'de>> EnumAccess<'de> for SerdeDecoder<'a, 'de, DE> {
    type Error = DecodeError;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let idx = u32::decode(&mut self.de)?;
        let val = seed.deserialize(idx.into_deserializer())?;
        Ok((val, self))
    }
}

impl<'de, 'a, DE: BorrowDecoder<'de>> VariantAccess<'de> for SerdeDecoder<'a, 'de, DE> {
    type Error = DecodeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}
