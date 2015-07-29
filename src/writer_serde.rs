use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::Write;
use std::u32;
use std::u64;

use serde;

use byteorder::{BigEndian, WriteBytesExt};
use byteorder::Error as ByteOrderError;

use super::SizeLimit;

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
    SizeLimit
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

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            SerializeError::IoError(ref err) => write!(f, "IoError: {}", err),
            SerializeError::SizeLimit => write!(f, "SizeLimit")
        }
    }
}

impl Error for SerializeError {
    fn description(&self) -> &str {
        match *self {
            SerializeError::IoError(ref err) => Error::description(err),
            SerializeError::SizeLimit => "the size limit for decoding has been reached"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SerializeError::IoError(ref err)     => err.cause(),
            SerializeError::SizeLimit => None
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

        serde::Serializer::visit_u32(self, tag as u32)
    }
}

impl<'a, W: Write> serde::Serializer for Serializer<'a, W> {
    type Error = SerializeError;

    fn visit_unit(&mut self) -> SerializeResult<()> { Ok(()) }

    fn visit_bool(&mut self, v: bool) -> SerializeResult<()> {
        self.writer.write_u8(if v {1} else {0}).map_err(wrap_io)
    }

    fn visit_u8(&mut self, v: u8) -> SerializeResult<()> {
        self.writer.write_u8(v).map_err(wrap_io)
    }

    fn visit_u16(&mut self, v: u16) -> SerializeResult<()> {
        self.writer.write_u16::<BigEndian>(v).map_err(wrap_io)
    }

    fn visit_u32(&mut self, v: u32) -> SerializeResult<()> {
        self.writer.write_u32::<BigEndian>(v).map_err(wrap_io)
    }

    fn visit_u64(&mut self, v: u64) -> SerializeResult<()> {
        self.writer.write_u64::<BigEndian>(v).map_err(wrap_io)
    }

    fn visit_i8(&mut self, v: i8) -> SerializeResult<()> {
        self.writer.write_i8(v).map_err(wrap_io)
    }

    fn visit_i16(&mut self, v: i16) -> SerializeResult<()> {
        self.writer.write_i16::<BigEndian>(v).map_err(wrap_io)
    }

    fn visit_i32(&mut self, v: i32) -> SerializeResult<()> {
        self.writer.write_i32::<BigEndian>(v).map_err(wrap_io)
    }

    fn visit_i64(&mut self, v: i64) -> SerializeResult<()> {
        self.writer.write_i64::<BigEndian>(v).map_err(wrap_io)
    }

    fn visit_f32(&mut self, v: f32) -> SerializeResult<()> {
        self.writer.write_f32::<BigEndian>(v).map_err(wrap_io)
    }

    fn visit_f64(&mut self, v: f64) -> SerializeResult<()> {
        self.writer.write_f64::<BigEndian>(v).map_err(wrap_io)
    }

    fn visit_str(&mut self, v: &str) -> SerializeResult<()> {
        try!(self.visit_usize(v.len()));
        self.writer.write_all(v.as_bytes()).map_err(SerializeError::IoError)
    }

    fn visit_none(&mut self) -> SerializeResult<()> {
        self.writer.write_u8(0).map_err(wrap_io)
    }

    fn visit_some<T>(&mut self, v: T) -> SerializeResult<()>
        where T: serde::Serialize,
    {
        try!(self.writer.write_u8(1).map_err(wrap_io));
        v.serialize(self)
    }

    fn visit_seq<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        let len = match visitor.len() {
            Some(len) => len,
            None => panic!("do not know how to serialize a sequence with no length"),
        };

        try!(self.visit_usize(len));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn visit_tuple<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn visit_seq_elt<V>(&mut self, value: V) -> SerializeResult<()>
        where V: serde::Serialize,
    {
        value.serialize(self)
    }

    fn visit_map<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        let len = match visitor.len() {
            Some(len) => len,
            None => panic!("do not know how to serialize a map with no length"),
        };

        try!(self.visit_usize(len));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> SerializeResult<()>
        where K: serde::Serialize,
              V: serde::Serialize,
    {
        try!(key.serialize(self));
        value.serialize(self)
    }

    fn visit_struct<V>(&mut self, _name: &str, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn visit_struct_elt<K, V>(&mut self, _key: K, value: V) -> SerializeResult<()>
        where K: serde::Serialize,
              V: serde::Serialize,
    {
        value.serialize(self)
    }

    fn visit_unit_variant(&mut self,
                          _name: &str,
                          variant_index: usize,
                          _variant: &str) -> SerializeResult<()> {
        self.add_enum_tag(variant_index)
    }

    fn visit_tuple_variant<V>(&mut self,
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

    fn visit_struct_variant<V>(&mut self,
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

struct SizeChecker {
    size_limit: u64,
    written: u64
}

impl SizeChecker {
    fn new(limit: u64) -> SizeChecker {
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

    fn visit_unit(&mut self) -> SerializeResult<()> { Ok(()) }

    fn visit_bool(&mut self, _: bool) -> SerializeResult<()> {
        self.add_value(0 as u8)
    }

    fn visit_u8(&mut self, v: u8) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_u16(&mut self, v: u16) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_u32(&mut self, v: u32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_u64(&mut self, v: u64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_i8(&mut self, v: i8) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_i16(&mut self, v: i16) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_i32(&mut self, v: i32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_i64(&mut self, v: i64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_f32(&mut self, v: f32) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_f64(&mut self, v: f64) -> SerializeResult<()> {
        self.add_value(v)
    }

    fn visit_str(&mut self, v: &str) -> SerializeResult<()> {
        try!(self.add_value(0 as u64));
        self.add_raw(v.len())
    }

    fn visit_none(&mut self) -> SerializeResult<()> {
        self.add_value(0 as u8)
    }

    fn visit_some<T>(&mut self, v: T) -> SerializeResult<()>
        where T: serde::Serialize,
    {
        try!(self.add_value(1 as u8));
        v.serialize(self)
    }

    fn visit_seq<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        let len = match visitor.len() {
            Some(len) => len,
            None => panic!("do not know how to serialize a sequence with no length"),
        };

        try!(self.visit_usize(len));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn visit_tuple<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::SeqVisitor,
    {
        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn visit_seq_elt<V>(&mut self, value: V) -> SerializeResult<()>
        where V: serde::Serialize,
    {
        value.serialize(self)
    }

    fn visit_map<V>(&mut self, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        let len = match visitor.len() {
            Some(len) => len,
            None => panic!("do not know how to serialize a map with no length"),
        };

        try!(self.visit_usize(len));

        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> SerializeResult<()>
        where K: serde::Serialize,
              V: serde::Serialize,
    {
        try!(key.serialize(self));
        value.serialize(self)
    }

    fn visit_struct<V>(&mut self, _name: &str, mut visitor: V) -> SerializeResult<()>
        where V: serde::ser::MapVisitor,
    {
        while let Some(()) = try!(visitor.visit(self)) { }

        Ok(())
    }

    fn visit_struct_elt<K, V>(&mut self, _key: K, value: V) -> SerializeResult<()>
        where K: serde::Serialize,
              V: serde::Serialize,
    {
        value.serialize(self)
    }

    fn visit_unit_variant(&mut self,
                          _name: &str,
                          variant_index: usize,
                          _variant: &str) -> SerializeResult<()> {
        self.add_enum_tag(variant_index)
    }

    fn visit_tuple_variant<V>(&mut self,
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

    fn visit_struct_variant<V>(&mut self,
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




    /*
    type Error = SerializeError;

    fn emit_nil(&mut self) -> SerializeResult<()> { Ok(()) }
    fn emit_usize(&mut self, v: usize) -> SerializeResult<()> {
        self.add_value(v as u64)
    }
    fn emit_u64(&mut self, v: u64) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_u32(&mut self, v: u32) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_u16(&mut self, v: u16) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_u8(&mut self, v: u8) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_isize(&mut self, v: isize) -> SerializeResult<()> {
        self.add_value(v as i64)
    }
    fn emit_i64(&mut self, v: i64) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_i32(&mut self, v: i32) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_i16(&mut self, v: i16) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_i8(&mut self, v: i8) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_bool(&mut self, _: bool) -> SerializeResult<()> {
        self.add_value(0 as u8)
    }
    fn emit_f64(&mut self, v: f64) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_f32(&mut self, v: f32) -> SerializeResult<()> {
        self.add_value(v)
    }
    fn emit_char(&mut self, v: char) -> SerializeResult<()> {
        self.add_raw(v.len_utf8())
    }
    fn emit_str(&mut self, v: &str) -> SerializeResult<()> {
        try!(self.add_value(0 as u64));
        self.add_raw(v.len())
    }
    fn emit_enum<F>(&mut self, __: &str, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_enum_variant<F>(&mut self, _: &str,
                            v_id: usize,
                            _: usize,
                            f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            try!(self.add_value(v_id as u32));
            f(self)
        }
    fn emit_enum_variant_arg<F>(&mut self, _: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_enum_struct_variant<F>(&mut self, _: &str,
                                   _: usize,
                                   _: usize,
                                   f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_enum_struct_variant_field<F>(&mut self,
                                         _: &str,
                                         _: usize,
                                         f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_struct<F>(&mut self, _: &str, _: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_struct_field<F>(&mut self, _: &str, _: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_tuple<F>(&mut self, _: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_tuple_arg<F>(&mut self, _: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_tuple_struct<F>(&mut self, _: &str, len: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            self.emit_tuple(len, f)
        }
    fn emit_tuple_struct_arg<F>(&mut self, f_idx: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            self.emit_tuple_arg(f_idx, f)
        }
    fn emit_option<F>(&mut self, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_option_none(&mut self) -> SerializeResult<()> {
        self.add_value(0 as u8)
    }
    fn emit_option_some<F>(&mut self, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            try!(self.add_value(1 as u8));
            f(self)
        }
    fn emit_seq<F>(&mut self, len: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            try!(self.emit_usize(len));
            f(self)
        }
    fn emit_seq_elt<F>(&mut self, _: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_map<F>(&mut self, len: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            try!(self.emit_usize(len));
            f(self)
        }
    fn emit_map_elt_key<F>(&mut self, _: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    fn emit_map_elt_val<F>(&mut self, _: usize, f: F) -> SerializeResult<()> where
        F: FnOnce(&mut SizeChecker) -> SerializeResult<()> {
            f(self)
        }
    */
}

pub fn to_writer<W, T>(writer: &mut W,
                       value: &T,
                       size_limit: SizeLimit) -> SerializeResult<()>
    where W: Write,
          T: serde::Serialize,
{
    match size_limit {
        SizeLimit::Infinite => { }
        SizeLimit::Bounded(x) => {
            let mut size_checker = SizeChecker::new(x);
            try!(value.serialize(&mut size_checker))
        }
    }

    let mut serializer = Serializer::new(writer);
    serde::Serialize::serialize(value, &mut serializer)
}

pub fn to_vec<T>(value: &T, size_limit: SizeLimit) -> SerializeResult<Vec<u8>>
    where T: serde::Serialize,
{
    // Since we are putting values directly into a vector, we can do size
    // computation out here and pre-allocate a buffer of *exactly*
    // the right size.
    let mut writer = match size_limit {
        SizeLimit::Bounded(size_limit) => {
            let actual_size = match serialized_size_bounded(value, size_limit) {
                Some(actual_size) => actual_size,
                None => { return Err(SerializeError::SizeLimit); }
            };
            Vec::with_capacity(actual_size as usize)
        }
        SizeLimit::Infinite => Vec::new()
    };

    try!(to_writer(&mut writer, value, SizeLimit::Infinite));
    Ok(writer)
}

/// Returns the size that an object would be if serialized using bincode.
///
/// This is used internally as part of the check for encode_into, but it can
/// be useful for preallocating buffers if thats your style.
pub fn serialized_size<T: serde::Serialize>(value: &T) -> u64 {
    let mut size_checker = SizeChecker::new(u64::MAX);
    value.serialize(&mut size_checker).ok();
    size_checker.written
}

/// Given a maximum size limit, check how large an object would be if it
/// were to be serialized.
///
/// If it can be serialized in `max` or fewer bytes, that number will be returned
/// inside `Some`.  If it goes over bounds, then None is returned.
pub fn serialized_size_bounded<T: serde::Serialize>(value: &T, max: u64) -> Option<u64> {
    let mut size_checker = SizeChecker::new(max);
    value.serialize(&mut size_checker).ok().map(|_| size_checker.written)
}
