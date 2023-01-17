use super::EncodeError as SerdeEncodeError;
use crate::{config::Config, error::EncodeError, size::EncodedSize};
use core::marker::PhantomData;
use serde::ser::*;

/// Calculate the encoded size for the given value.
pub fn encoded_size<T, C>(t: T, _config: C) -> Result<usize, EncodeError>
where
    T: Serialize,
    C: Config,
{
    if C::SKIP_FIXED_ARRAY_LENGTH {
        return Err(SerdeEncodeError::SkipFixedArrayLengthNotSupported.into());
    }
    let mut encoded_size: usize = 0;
    let serializer = SerdeEncodedSize::<'_, C>::new(&mut encoded_size);
    t.serialize(serializer)?;
    Ok(encoded_size)
}

pub(super) struct SerdeEncodedSize<'a, C: Config> {
    encoded_size: &'a mut usize,
    config: PhantomData<C>,
}

impl<'a, C> SerdeEncodedSize<'a, C>
where
    C: Config,
{
    pub(super) fn new(encoded_size: &'a mut usize) -> Self {
        SerdeEncodedSize {
            encoded_size,
            config: PhantomData,
        }
    }
}

impl<'a, C> Serializer for SerdeEncodedSize<'a, C>
where
    C: Config,
{
    type Ok = ();

    type Error = EncodeError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    serde::serde_if_integer128! {
        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
            *self.encoded_size += v.encoded_size::<C>()?; Ok(())
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    serde::serde_if_integer128! {
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            *self.encoded_size += v.encoded_size::<C>()?; Ok(())
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += v.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += 0u8.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        *self.encoded_size += 1u8.encoded_size::<C>()?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        *self.encoded_size += variant_index.encoded_size::<C>()?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        *self.encoded_size += variant_index.encoded_size::<C>()?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.ok_or_else(|| SerdeEncodeError::SequenceMustHaveLength.into())?;
        *self.encoded_size += len.encoded_size::<C>()?;
        Ok(Compound {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        *self.encoded_size += variant_index.encoded_size::<C>()?;
        Ok(Compound {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.ok_or_else(|| SerdeEncodeError::SequenceMustHaveLength.into())?;
        *self.encoded_size += len.encoded_size::<C>()?;
        Ok(Compound {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        *self.encoded_size += variant_index.encoded_size::<C>()?;
        Ok(Compound {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    #[cfg(not(feature = "alloc"))]
    fn collect_str<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: core::fmt::Display,
    {
        Err(SerdeEncodeError::CannotCollectStr.into())
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

type Compound<'a, C> = SerdeEncodedSize<'a, C>;

impl<'a, C: Config> SerializeSeq for Compound<'a, C> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncodedSize {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, C: Config> SerializeTuple for Compound<'a, C> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncodedSize {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, C: Config> SerializeTupleStruct for Compound<'a, C> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncodedSize {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, C: Config> SerializeTupleVariant for Compound<'a, C> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncodedSize {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, C: Config> SerializeMap for Compound<'a, C> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(SerdeEncodedSize {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncodedSize {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, C: Config> SerializeStruct for Compound<'a, C> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncodedSize {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, C: Config> SerializeStructVariant for Compound<'a, C> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncodedSize {
            encoded_size: self.encoded_size,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
