use crate::{
    config::Config,
    enc::{Encode, Encoder},
    error::EncodeError,
};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use serde_incl::ser::*;

#[cfg(feature = "alloc")]
/// Encode a `serde` `Serialize` type into a `Vec<u8>` with the bincode algorithm
pub fn serde_encode_to_vec<T, C>(t: T, config: C) -> Result<Vec<u8>, EncodeError>
where
    T: Serialize,
    C: Config,
{
    let mut encoder = crate::enc::EncoderImpl::new(crate::VecWriter::default(), config);
    let serializer = SerdeEncoder { enc: &mut encoder };
    t.serialize(serializer)?;
    Ok(encoder.into_writer().collect())
}

/// Encode a `serde` `Serialize` type into a given byte slice with the bincode algorithm
pub fn serde_encode_to_slice<T, C>(t: T, slice: &mut [u8], config: C) -> Result<usize, EncodeError>
where
    T: Serialize,
    C: Config,
{
    let mut encoder =
        crate::enc::EncoderImpl::new(crate::enc::write::SliceWriter::new(slice), config);
    let serializer = SerdeEncoder { enc: &mut encoder };
    t.serialize(serializer)?;
    Ok(encoder.into_writer().bytes_written())
}

struct SerdeEncoder<'a, ENC: Encoder> {
    enc: &'a mut ENC,
}

impl<'a, ENC> Serializer for SerdeEncoder<'a, ENC>
where
    ENC: Encoder,
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
        v.encode(self.enc)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        0u8.encode(self.enc)
    }

    fn serialize_some<T: ?Sized>(mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        1u8.encode(&mut self.enc)?;
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
        variant_index.encode(self.enc)
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
        mut self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        variant_index.encode(&mut self.enc)?;
        value.serialize(self)
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.ok_or(EncodeError::SequenceMustHaveLength)?;
        len.encode(&mut self.enc)?;
        Ok(Compound { enc: self.enc })
    }

    fn serialize_tuple(mut self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        len.encode(&mut self.enc)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        mut self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        len.encode(&mut self.enc)?;
        Ok(Compound { enc: self.enc })
    }

    fn serialize_tuple_variant(
        mut self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        variant_index.encode(&mut self.enc)?;
        Ok(Compound { enc: self.enc })
    }

    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.ok_or(EncodeError::SequenceMustHaveLength)?;
        len.encode(&mut self.enc)?;
        Ok(Compound { enc: self.enc })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound { enc: self.enc })
    }

    fn serialize_struct_variant(
        mut self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        variant_index.encode(&mut self.enc)?;
        Ok(Compound { enc: self.enc })
    }
}

type Compound<'a, ENC> = SerdeEncoder<'a, ENC>;

impl<'a, ENC: Encoder> SerializeSeq for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeTuple for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeTupleStruct for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeTupleVariant for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeMap for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(SerdeEncoder { enc: self.enc })
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeStruct for Compound<'a, ENC> {
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
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeStructVariant for Compound<'a, ENC> {
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
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
