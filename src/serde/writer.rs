use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::io::Write;
use std::u32;

use serde_crate as serde;

use byteorder::{BigEndian, WriteBytesExt};

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
pub struct Serializer<W> {
    writer: W,
}

fn wrap_io(err: IoError) -> SerializeError {
    SerializeError::IoError(err)
}

impl serde::ser::Error for SerializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerializeError::Custom(msg.to_string())
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

impl<W: Write> Serializer<W> {
    pub fn new(w: W) -> Serializer<W> {
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

impl<'a, W: Write> serde::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = SerializeError;
    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    fn serialize_unit(self) -> SerializeResult<()> { Ok(()) }

    fn serialize_unit_struct(self, _: &'static str) -> SerializeResult<()> { Ok(()) }

    fn serialize_bool(self, v: bool) -> SerializeResult<()> {
        self.writer.write_u8(if v {1} else {0}).map_err(wrap_io)
    }

    fn serialize_u8(self, v: u8) -> SerializeResult<()> {
        self.writer.write_u8(v).map_err(wrap_io)
    }

    fn serialize_u16(self, v: u16) -> SerializeResult<()> {
        self.writer.write_u16::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_u32(self, v: u32) -> SerializeResult<()> {
        self.writer.write_u32::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_u64(self, v: u64) -> SerializeResult<()> {
        self.writer.write_u64::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_i8(self, v: i8) -> SerializeResult<()> {
        self.writer.write_i8(v).map_err(wrap_io)
    }

    fn serialize_i16(self, v: i16) -> SerializeResult<()> {
        self.writer.write_i16::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_i32(self, v: i32) -> SerializeResult<()> {
        self.writer.write_i32::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_i64(self, v: i64) -> SerializeResult<()> {
        self.writer.write_i64::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_f32(self, v: f32) -> SerializeResult<()> {
        self.writer.write_f32::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_f64(self, v: f64) -> SerializeResult<()> {
        self.writer.write_f64::<BigEndian>(v).map_err(wrap_io)
    }

    fn serialize_str(self, v: &str) -> SerializeResult<()> {
        try!(self.serialize_u64(v.len() as u64));
        self.writer.write_all(v.as_bytes()).map_err(SerializeError::IoError)
    }

    fn serialize_char(self, c: char) -> SerializeResult<()> {
        self.writer.write_all(encode_utf8(c).as_slice()).map_err(SerializeError::IoError)
    }

    fn serialize_bytes(self, v: &[u8]) -> SerializeResult<()> {
        try!(self.serialize_u64(v.len() as u64));
        self.writer.write_all(v).map_err(SerializeError::IoError)
    }

    fn serialize_none(self) -> SerializeResult<()> {
        self.writer.write_u8(0).map_err(wrap_io)
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> SerializeResult<()>
        where T: serde::Serialize,
    {
        try!(self.writer.write_u8(1).map_err(wrap_io));
        v.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> SerializeResult<Self::SerializeSeq> {
        let len = len.expect("do not know how to serialize a sequence with no length");
        try!(self.serialize_u64(len as u64));
        Ok(Compound {ser: self})
    }

    fn serialize_seq_fixed_size(self, len: usize) -> SerializeResult<Self::SerializeSeq> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple(self, _len: usize) -> SerializeResult<Self::SerializeTuple> {
        Ok(Compound {ser: self})
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> SerializeResult<Self::SerializeTupleStruct> {
        Ok(Compound {ser: self})
    }

    fn serialize_tuple_variant(self,
                              _name: &'static str,
                              variant_index: usize,
                              _variant: &'static str,
                              _len: usize) -> SerializeResult<Self::SerializeTupleVariant>
    {
        try!(self.add_enum_tag(variant_index));
        Ok(Compound {ser: self})
    }

    fn serialize_map(self, len: Option<usize>) -> SerializeResult<Self::SerializeMap> {
        let len = len.expect("do not know how to serialize a map with no length");
        try!(self.serialize_u64(len as u64));
        Ok(Compound {ser: self})
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> SerializeResult<Self::SerializeStruct> {
        Ok(Compound {ser: self})
    }

    fn serialize_struct_variant(self,
                               _name: &'static str,
                               variant_index: usize,
                               _variant: &'static str,
                               _len: usize) -> SerializeResult<Self::SerializeStructVariant>
    {
        try!(self.add_enum_tag(variant_index));
        Ok(Compound {ser: self})
    }

    fn serialize_newtype_struct<T: ?Sized>(self,
                               _name: &'static str,
                               value: &T) -> SerializeResult<()>
        where T: serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self,
                               _name: &'static str,
                               variant_index: usize,
                               _variant: &'static str,
                               value: &T) -> SerializeResult<()>
        where T: serde::ser::Serialize,
    {
        try!(self.add_enum_tag(variant_index));
        value.serialize(self)
    }

    fn serialize_unit_variant(self,
                          _name: &'static str,
                          variant_index: usize,
                          _variant: &'static str) -> SerializeResult<()> {
        self.add_enum_tag(variant_index)
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

impl<'a> serde::Serializer for &'a mut SizeChecker {
    type Ok = ();
    type Error = SerializeError;
    type SerializeSeq = SizeCompound<'a>;
    type SerializeTuple = SizeCompound<'a>;
    type SerializeTupleStruct = SizeCompound<'a>;
    type SerializeTupleVariant = SizeCompound<'a>;
    type SerializeMap = SizeCompound<'a>;
    type SerializeStruct = SizeCompound<'a>;
    type SerializeStructVariant = SizeCompound<'a>;

    fn serialize_unit(self) -> SerializeResult<()> { Ok(()) }

    fn serialize_unit_struct(self, _: &'static str) -> SerializeResult<()> { Ok(()) }

    fn serialize_bool(self, _: bool) -> SerializeResult<()> {
        self.add_value(0 as u8)
    }

    fn serialize_u8(self, v: u8) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_u16(self, v: u16) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_u32(self, v: u32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_u64(self, v: u64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_i8(self, v: i8) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_i16(self, v: i16) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_i32(self, v: i32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_i64(self, v: i64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_f32(self, v: f32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_f64(self, v: f64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn serialize_str(self, v: &str) -> SerializeResult<()> {
        try!(self.add_value(0 as u64));
        self.add_raw(v.len())
    }

    fn serialize_char(self, c: char) -> SerializeResult<()> {
        self.add_raw(encode_utf8(c).as_slice().len())
    }

    fn serialize_bytes(self, v: &[u8]) -> SerializeResult<()> {
        try!(self.add_value(0 as u64));
        self.add_raw(v.len())
    }

    fn serialize_none(self) -> SerializeResult<()> {
        self.add_value(0 as u8)
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> SerializeResult<()>
        where T: serde::Serialize,
    {
        try!(self.add_value(1 as u8));
        v.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> SerializeResult<Self::SerializeSeq> {
        let len = len.expect("do not know how to serialize a sequence with no length");

        try!(self.serialize_u64(len as u64));
        Ok(SizeCompound {ser: self})
    }

    fn serialize_seq_fixed_size(self, len: usize) -> SerializeResult<Self::SerializeSeq> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple(self, _len: usize) -> SerializeResult<Self::SerializeTuple> {
        Ok(SizeCompound {ser: self})
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> SerializeResult<Self::SerializeTupleStruct> {
        Ok(SizeCompound {ser: self})
    }

    fn serialize_tuple_variant(self,
                         _name: &'static str,
                         variant_index: usize,
                         _variant: &'static str,
                         _len: usize) -> SerializeResult<Self::SerializeTupleVariant>
    {
        try!(self.add_enum_tag(variant_index));
        Ok(SizeCompound {ser: self})
    }

    fn serialize_map(self, len: Option<usize>) -> SerializeResult<Self::SerializeMap>
    {
        let len = len.expect("do not know how to serialize a map with no length");

        try!(self.serialize_u64(len as u64));
        Ok(SizeCompound {ser: self})
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> SerializeResult<Self::SerializeStruct> {
        Ok(SizeCompound {ser: self})
    }

    fn serialize_struct_variant(self,
                               _name: &'static str,
                               variant_index: usize,
                               _variant: &'static str,
                               _len: usize) -> SerializeResult<Self::SerializeStructVariant>
    {
        try!(self.add_enum_tag(variant_index));
        Ok(SizeCompound {ser: self})
    }

    fn serialize_newtype_struct<V: serde::Serialize + ?Sized>(self, _name: &'static str, v: &V) -> SerializeResult<()> {
        v.serialize(self)
    }

    fn serialize_unit_variant(self,
                          _name: &'static str,
                          variant_index: usize,
                          _variant: &'static str) -> SerializeResult<()> {
        self.add_enum_tag(variant_index)
    }

    fn serialize_newtype_variant<V: serde::Serialize + ?Sized>(self,
                               _name: &'static str,
                               variant_index: usize,
                               _variant: &'static str,
                               value: &V) -> SerializeResult<()>
    {
        try!(self.add_enum_tag(variant_index));
        value.serialize(self)
    }
}

#[doc(hidden)]
pub struct Compound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> serde::ser::SerializeSeq for Compound<'a, W> 
    where W: Write
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeTuple for Compound<'a, W>
    where W: Write
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeTupleStruct for Compound<'a, W> 
    where W: Write
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeTupleVariant for Compound<'a, W> 
    where W: Write
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeMap for Compound<'a, W> 
    where W: Write
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_key<K: ?Sized>(&mut self, value: &K) -> SerializeResult<()> 
    where K: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

        #[inline]
    fn serialize_value<V: ?Sized>(&mut self, value: &V) -> SerializeResult<()> 
    where V: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeStruct for Compound<'a, W> 
    where W: Write
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeStructVariant for Compound<'a, W> 
    where W: Write
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

#[doc(hidden)]
pub struct SizeCompound<'a> {
    ser: &'a mut SizeChecker,
}

impl<'a> serde::ser::SerializeSeq for SizeCompound<'a> 
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTuple for SizeCompound<'a>
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleStruct for SizeCompound<'a> 
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleVariant for SizeCompound<'a>
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeMap for SizeCompound<'a>
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_key<K: ?Sized>(&mut self, value: &K) -> SerializeResult<()> 
    where K: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

        #[inline]
    fn serialize_value<V: ?Sized>(&mut self, value: &V) -> SerializeResult<()> 
    where V: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStruct for SizeCompound<'a> 
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStructVariant for SizeCompound<'a>
{
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> SerializeResult<()> 
    where T: serde::ser::Serialize 
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> SerializeResult<()> {
        Ok(())
    }
}

const TAG_CONT: u8    = 0b1000_0000;
const TAG_TWO_B: u8   = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8  = 0b1111_0000;
const MAX_ONE_B: u32   =     0x80;
const MAX_TWO_B: u32   =    0x800;
const MAX_THREE_B: u32 =  0x10000;

fn encode_utf8(c: char) -> EncodeUtf8 {
    let code = c as u32;
    let mut buf = [0; 4];
    let pos = if code < MAX_ONE_B {
        buf[3] = code as u8;
        3
    } else if code < MAX_TWO_B {
        buf[2] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        2
    } else if code < MAX_THREE_B {
        buf[1] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
        buf[2] = (code >>  6 & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        1
    } else {
        buf[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
        buf[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code >>  6 & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        0
    };
    EncodeUtf8 { buf: buf, pos: pos }
}

struct EncodeUtf8 {
    buf: [u8; 4],
    pos: usize,
}

impl EncodeUtf8 {
    fn as_slice(&self) -> &[u8] {
        &self.buf[self.pos..]
    }
}
