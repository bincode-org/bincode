use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::Write;
use std::u32;

use serde_crate as serde;

use byteorder::{BigEndian, WriteBytesExt};
use byteorder::Error as ByteOrderError;

pub type SerializeResult<T> = Result<T, SerializeError>;


/// An error that can be produced during encoding.
#[derive(Debug)]
pub enum SerializeError {
    /// An error originating from the underlying `Writer`.
    IoError(IoError),
    /// An object could not be encoded with the given size limit.
    ///
    /// This error is returned before any bytes are written to the
    /// output `Writer`.
    SizeLimit,
    /// A custom error message
    Custom(String)
}

/// An Serializer that encodes values directly into a Writer.
///
/// This struct should not be used often.
/// For most cases, prefer the `encode_into` function.
pub struct Serializer<'a, W: 'a> {
    writer: &'a mut W,
}

fn wrap_io(err: ByteOrderError) -> SerializeError {
    match err {
        ByteOrderError::Io(ioe) => SerializeError::IoError(ioe),
        ByteOrderError::UnexpectedEOF => SerializeError::IoError(
            IoError::new(IoErrorKind::Other,
                         "ByteOrder could not write to the buffer"))
    }
}

impl serde::ser::Error for SerializeError {
    fn custom<T: Into<String>>(msg: T) -> Self {
        SerializeError::Custom(msg.into())
    }
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            SerializeError::IoError(ref err) => write!(f, "IoError: {}", err),
            SerializeError::Custom(ref s) => write!(f, "Custom Error {}", s),
            SerializeError::SizeLimit => write!(f, "SizeLimit"),
        }
    }
}

impl Error for SerializeError {
    fn description(&self) -> &str {
        match *self {
            SerializeError::IoError(ref err) => Error::description(err),
            SerializeError::SizeLimit => "the size limit for decoding has been reached",
            SerializeError::Custom(_) => "a custom serialization error was reported",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SerializeError::IoError(ref err) => err.cause(),
            SerializeError::SizeLimit => None,
            SerializeError::Custom(_) => None,
        }
    }
}

impl<'a, W: Write> Serializer<'a, W> {
    pub fn new(w: &'a mut W) -> Serializer<'a, W> {
        Serializer {
            writer: w,
        }
    }

    fn add_enum_tag(&mut self, tag: usize) -> SerializeResult<()> {
        if tag > u32::MAX as usize {
            panic!("Variant tag doesn't fit in a u32")
        }

        serde::Serializer::serialize_u32(self, tag as u32)
    }
}

impl<'a, W: Write> serde::Serializer for Serializer<'a, W> {
    type Error = SerializeError;

    fn serialize_unit(&mut self) -> SerializeResult<()> { Ok(()) }

    fn serialize_bool(&mut self, v: bool) -> SerializeResult<()> {
        self.writer.write_u8(if v {1} else {0}).map_err(wrap_io)
    }

    fn serialize_u8(&mut self, v: u8) -> SerializeResult<()> {
        self.writer.write_u8(v).map_err(wrap_io)
    }

    fn serialize_u16(&mut self, v: u16) -> SerializeResult<()> {
        self.writer.write_u16::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_u32(&mut self, v: u32) -> SerializeResult<()> {
        self.writer.write_u32::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_u64(&mut self, v: u64) -> SerializeResult<()> {
        self.writer.write_u64::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_i8(&mut self, v: i8) -> SerializeResult<()> {
        self.writer.write_i8(v).map_err(wrap_io)
    }

    fn serialize_i16(&mut self, v: i16) -> SerializeResult<()> {
        self.writer.write_i16::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_i32(&mut self, v: i32) -> SerializeResult<()> {
        self.writer.write_i32::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_i64(&mut self, v: i64) -> SerializeResult<()> {
        self.writer.write_i64::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_f32(&mut self, v: f32) -> SerializeResult<()> {
        self.writer.write_f32::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_f64(&mut self, v: f64) -> SerializeResult<()> {
        self.writer.write_f64::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_str(&mut self, v: &str) -> SerializeResult<()> {
        try!(self.serialize_usize(v.len()));
        self.writer.write_all(v.as_bytes()).map_err(SerializeError::IoError)
    }

    fn serialize_none(&mut self) -> SerializeResult<()> {
        self.writer.write_u8(0).map_err(wrap_io)
    }

    fn serialize_some<T>(&mut self, v: T) -> SerializeResult<()>
        where T: serde::Serialize,
    {
        try!(self.writer.write_u8(1).map_err(wrap_io));
        v.serialize(self)
    }

    fn serialize_seq<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        let len = match visitor.len() {
            Some(len) => len,
            None => panic!("do not know how to serialize a sequence with no length"),
        };

        try!(self.serialize_usize(len));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_tuple<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_seq_elt<V>(&mut self, value: V) -> SerializeResult<()>
        where V: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_map<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        let len = match visitor.len() {
            Some(len) => len,
            None => panic!("do not know how to serialize a map with no length"),
        };

        try!(self.serialize_usize(len));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_map_elt<K, V>(&mut self, key: K, value: V) -> SerializeResult<()>
        where K: serde::Serialize,
              V: serde::Serialize,
    {
        try!(key.serialize(self));
        value.serialize(self)
    }

    fn serialize_struct<V>(&mut self, _name: &str, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_struct_elt<V>(&mut self, _key: &str, value: V) -> SerializeResult<()>
        where V: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_struct<T>(&mut self,
                               _name: &str,
                               value: T) -> SerializeResult<()>
        where T: serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit_variant(&mut self,
                          _name: &str,
                          variant_index: usize,
                          _variant: &str) -> SerializeResult<()> {
        self.add_enum_tag(variant_index)
    }

    fn serialize_tuple_variant<V>(&mut self,
                              _name: &str,
                              variant_index: usize,
                              _variant: &str,
                              mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        try!(self.add_enum_tag(variant_index));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_struct_variant<V>(&mut self,
                               _name: &str,
                               variant_index: usize,
                               _variant: &str,
                               mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        try!(self.add_enum_tag(variant_index));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }
}

pub struct SizeChecker {
    pub size_limit: u64,
    pub written: u64
}

impl SizeChecker {
    pub fn new(limit: u64) -> SizeChecker {
        SizeChecker {
            size_limit: limit,
            written: 0
        }
    }

    fn add_raw(&mut self, size: usize) -> SerializeResult<()> {
        self.written += size as u64;
        if self.written <= self.size_limit {
            Ok(())
        } else {
            Err(SerializeError::SizeLimit)
        }
    }

    fn add_value<T>(&mut self, t: T) -> SerializeResult<()> {
        use std::mem::size_of_val;
        self.add_raw(size_of_val(&t))
    }

    fn add_enum_tag(&mut self, tag: usize) -> SerializeResult<()> {
        if tag > u32::MAX as usize {
            panic!("Variant tag doesn't fit in a u32")
        }

        self.add_value(tag as u32)
    }
}

impl serde::Serializer for SizeChecker {
    type Error = SerializeError;

    fn serialize_unit(&mut self) -> SerializeResult<()> { Ok(()) }

    fn serialize_bool(&mut self, _: bool) -> SerializeResult<()> {
        self.add_value(0 as u8)
    }

    fn serialize_u8(&mut self, v: u8) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_u16(&mut self, v: u16) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_u32(&mut self, v: u32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_u64(&mut self, v: u64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_i8(&mut self, v: i8) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_i16(&mut self, v: i16) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_i32(&mut self, v: i32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_i64(&mut self, v: i64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_f32(&mut self, v: f32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_f64(&mut self, v: f64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_str(&mut self, v: &str) -> SerializeResult<()> {
        try!(self.add_value(0 as u64));
        self.add_raw(v.len())
    }

    fn serialize_none(&mut self) -> SerializeResult<()> {
        self.add_value(0 as u8)
    }

    fn serialize_some<T>(&mut self, v: T) -> SerializeResult<()>
        where T: serde::Serialize,
    {
        try!(self.add_value(1 as u8));
        v.serialize(self)
    }

    fn serialize_seq<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        let len = match visitor.len() {
            Some(len) => len,
            None => panic!("do not know how to serialize a sequence with no length"),
        };

        try!(self.serialize_usize(len));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_tuple<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_seq_elt<V>(&mut self, value: V) -> SerializeResult<()>
        where V: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_map<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        let len = match visitor.len() {
            Some(len) => len,
            None => panic!("do not know how to serialize a map with no length"),
        };

        try!(self.serialize_usize(len));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_map_elt<K, V>(&mut self, key: K, value: V) -> SerializeResult<()>
        where K: serde::Serialize,
              V: serde::Serialize,
    {
        try!(key.serialize(self));
        value.serialize(self)
    }

    fn serialize_struct<V>(&mut self, _name: &str, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_struct_elt<V>(&mut self, _key: &str, value: V) -> SerializeResult<()>
        where V: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit_variant(&mut self,
                          _name: &str,
                          variant_index: usize,
                          _variant: &str) -> SerializeResult<()> {
        self.add_enum_tag(variant_index)
    }

    fn serialize_tuple_variant<V>(&mut self,
                         _name: &str,
                         variant_index: usize,
                         _variant: &str,
                         mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        try!(self.add_enum_tag(variant_index));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn serialize_struct_variant<V>(&mut self,
                               _name: &str,
                               variant_index: usize,
                               _variant: &str,
                               mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        try!(self.add_enum_tag(variant_index));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }
}
