use super::DecodeError as SerdeDecodeError;
use crate::{
    config::Config,
    de::{read::Reader, Decode, Decoder},
    error::DecodeError,
};
use serde_incl::de::*;

/// Decode an owned type from the given slice. Will return the decoded type `T` as well as the amount of bytes that were read.
///
/// Note that this does not work with borrowed types like `&str` or `&[u8]`. For that use [decode_borrowed_from_slice].
///
/// [decode_borrowed_from_slice]: fn.decode_borrowed_from_slice.html
pub fn decode_from_slice<T, C>(slice: &[u8], config: C) -> Result<(T, usize), DecodeError>
where
    T: DeserializeOwned,
    C: Config,
{
    if C::SKIP_FIXED_ARRAY_LENGTH {
        return Err(SerdeDecodeError::SkipFixedArrayLengthNotSupported.into());
    }
    let reader = crate::de::read::SliceReader::new(slice);
    let mut decoder = crate::de::DecoderImpl::new(reader, config);
    let serde_decoder = SerdeDecoder { de: &mut decoder };
    let result = T::deserialize(serde_decoder)?;
    let bytes_read = slice.len() - decoder.reader().slice.len();
    Ok((result, bytes_read))
}

/// Decode an owned type from the given `std::io::Read`.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn decode_from_std_read<D: DeserializeOwned, C: Config, R: std::io::Read>(
    src: &mut R,
    config: C,
) -> Result<D, DecodeError> {
    if C::SKIP_FIXED_ARRAY_LENGTH {
        return Err(SerdeDecodeError::SkipFixedArrayLengthNotSupported.into());
    }
    let reader = crate::IoReader::new(src);
    let mut decoder = crate::de::DecoderImpl::new(reader, config);
    let serde_decoder = SerdeDecoder { de: &mut decoder };
    D::deserialize(serde_decoder)
}

/// Attempt to decode a given type `D` from the given [Reader].
///
/// See the [config] module for more information on configurations.
///
/// [config]: config/index.html
pub fn decode_from_reader<D: DeserializeOwned, R: Reader, C: Config>(
    reader: R,
    config: C,
) -> Result<D, DecodeError> {
    if C::SKIP_FIXED_ARRAY_LENGTH {
        return Err(SerdeDecodeError::SkipFixedArrayLengthNotSupported.into());
    }
    let mut decoder = crate::de::DecoderImpl::<_, C>::new(reader, config);
    let serde_decoder = SerdeDecoder { de: &mut decoder };
    D::deserialize(serde_decoder)
}

pub(crate) struct SerdeDecoder<'a, DE: Decoder> {
    pub(crate) de: &'a mut DE,
}

impl<'a, 'de, DE: Decoder> Deserializer<'de> for SerdeDecoder<'a, DE> {
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

    #[cfg(feature = "alloc")]
    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_string(Decode::decode(&mut self.de)?)
    }

    #[cfg(not(feature = "alloc"))]
    fn deserialize_str<V>(self, _: V) -> Result<V::Value, Self::Error>
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

    #[cfg(not(feature = "alloc"))]
    fn deserialize_string<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        Err(SerdeDecodeError::CannotAllocate.into())
    }

    #[cfg(feature = "alloc")]
    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        visitor.visit_byte_buf(Decode::decode(&mut self.de)?)
    }

    #[cfg(not(feature = "alloc"))]
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
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
    #[cfg(not(feature = "alloc"))]
    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde_incl::de::Visitor<'de>,
    {
        Err(SerdeDecodeError::CannotAllocate.into())
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
        struct Access<'a, 'b, DE: Decoder> {
            deserializer: &'a mut SerdeDecoder<'b, DE>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a, DE: Decoder + 'b> SeqAccess<'de> for Access<'a, 'b, DE> {
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
        struct Access<'a, 'b, DE: Decoder> {
            deserializer: &'a mut SerdeDecoder<'b, DE>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a, DE: Decoder + 'b> MapAccess<'de> for Access<'a, 'b, DE> {
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

impl<'de, 'a, DE: Decoder> EnumAccess<'de> for SerdeDecoder<'a, DE> {
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

impl<'de, 'a, DE: Decoder> VariantAccess<'de> for SerdeDecoder<'a, DE> {
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
