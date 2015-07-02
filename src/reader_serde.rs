use std::io;
use std::io::Read;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use std::convert::From;

use byteorder::Error as ByteOrderError;
use byteorder::{BigEndian, ReadBytesExt};
use num;
use serde;
use serde::de::value::ValueDeserializer;

use super::SizeLimit;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct InvalidEncoding {
    desc: &'static str,
    detail: Option<String>,
}

impl fmt::Display for InvalidEncoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidEncoding { detail: None, desc } =>
                write!(fmt, "{}", desc),
            InvalidEncoding { detail: Some(ref detail), desc } =>
                write!(fmt, "{} ({})", desc, detail)
        }
    }
}

/// An error that can be produced during decoding.
///
/// If decoding from a Buffer, assume that the buffer has been left
/// in an invalid state.
#[derive(Debug)]
pub enum DeserializeError {
    /// If the error stems from the reader that is being used
    /// during decoding, that error will be stored and returned here.
    IoError(IoError),
    /// If the bytes in the reader are not decodable because of an invalid
    /// encoding, this error will be returned.  This error is only possible
    /// if a stream is corrupted.  A stream produced from `encode` or `encode_into`
    /// should **never** produce an InvalidEncoding error.
    InvalidEncoding(InvalidEncoding),
    /// If decoding a message takes more than the provided size limit, this
    /// error is returned.
    SizeLimit,
    SyntaxError,
    EndOfStreamError,
    UnknownFieldError,
    MissingFieldError,
}

impl Error for DeserializeError {
    fn description(&self) -> &str {
        match *self {
            DeserializeError::IoError(ref err) => Error::description(err),
            DeserializeError::InvalidEncoding(ref ib) => ib.desc,
            DeserializeError::SizeLimit => "the size limit for decoding has been reached",
            DeserializeError::SyntaxError => "syntax error",
            DeserializeError::EndOfStreamError => "Unexpected EOF while reading a multi-byte number",
            DeserializeError::UnknownFieldError => "unknown field error",
            DeserializeError::MissingFieldError => "missing field error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DeserializeError::IoError(ref err) => err.cause(),
            DeserializeError::InvalidEncoding(_) => None,
            DeserializeError::SizeLimit => None,
            DeserializeError::SyntaxError => None,
            DeserializeError::EndOfStreamError => None,
            DeserializeError::UnknownFieldError => None,
            DeserializeError::MissingFieldError => None,
        }
    }
}

impl From<IoError> for DeserializeError {
    fn from(err: IoError) -> DeserializeError {
        DeserializeError::IoError(err)
    }
}

impl From<ByteOrderError> for DeserializeError {
    fn from(err: ByteOrderError) -> DeserializeError {
        match err {
            ByteOrderError::Io(ioe) => DeserializeError::IoError(ioe),
            ByteOrderError::UnexpectedEOF => DeserializeError::EndOfStreamError,
        }
    }
}

impl From<serde::de::value::Error> for DeserializeError {
    fn from(err: serde::de::value::Error) -> DeserializeError {
        use serde::de::value::Error;

        match err {
            Error::SyntaxError => DeserializeError::SyntaxError,
            Error::EndOfStreamError => {
                DeserializeError::EndOfStreamError
            }
            Error::UnknownFieldError(_) => DeserializeError::UnknownFieldError,
            Error::MissingFieldError(_) => DeserializeError::MissingFieldError,
        }
    }
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializeError::IoError(ref ioerr) =>
                write!(fmt, "IoError: {}", ioerr),
            DeserializeError::InvalidEncoding(ref ib) =>
                write!(fmt, "InvalidEncoding: {}", ib),
            DeserializeError::SizeLimit =>
                write!(fmt, "SizeLimit"),
            DeserializeError::SyntaxError =>
                write!(fmt, "SyntaxError"),
            DeserializeError::EndOfStreamError =>
                write!(fmt, "EndOfStreamError"),
            DeserializeError::UnknownFieldError =>
                write!(fmt, "UnknownFieldError"),
            DeserializeError::MissingFieldError =>
                write!(fmt, "MissingFieldError"),
        }
    }
}

impl serde::de::Error for DeserializeError {
    fn syntax_error() -> DeserializeError {
        DeserializeError::SyntaxError
    }

    fn end_of_stream_error() -> DeserializeError {
        DeserializeError::EndOfStreamError
    }

    fn unknown_field_error(_field: &str) -> DeserializeError {
        DeserializeError::UnknownFieldError
    }

    fn missing_field_error(_field: &'static str) -> DeserializeError {
        DeserializeError::MissingFieldError
    }
}

pub type DeserializeResult<T> = Result<T, DeserializeError>;


/// A Deserializer that reads bytes from a buffer.
///
/// This struct should rarely be used.
/// In most cases, prefer the `decode_from` function.
pub struct Deserializer<'a, R: 'a> {
    reader: &'a mut R,
    size_limit: SizeLimit,
    read: u64
}

impl<'a, R: Read> Deserializer<'a, R> {
    pub fn new(r: &'a mut R, size_limit: SizeLimit) -> Deserializer<'a, R> {
        Deserializer {
            reader: r,
            size_limit: size_limit,
            read: 0
        }
    }

    /// Returns the number of bytes read from the contained Reader.
    pub fn bytes_read(&self) -> u64 {
        self.read
    }
}

impl <'a, A> Deserializer<'a, A> {
    fn read_bytes(&mut self, count: u64) -> Result<(), DeserializeError> {
        self.read += count;
        match self.size_limit {
            SizeLimit::Infinite => Ok(()),
            SizeLimit::Bounded(x) if self.read <= x => Ok(()),
            SizeLimit::Bounded(_) => Err(DeserializeError::SizeLimit)
        }
    }

    fn read_type<T>(&mut self) -> Result<(), DeserializeError> {
        use std::mem::size_of;
        self.read_bytes(size_of::<T>() as u64)
    }
}

macro_rules! impl_nums {
    ($ty:ty, $visitor_method:ident, $reader_method:ident) => {
        #[inline]
        fn $visitor_method<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
            where V: serde::de::Visitor,
        {
            try!(self.read_type::<$ty>());
            let value = try!(self.reader.$reader_method::<BigEndian>());
            visitor.$visitor_method(value)
        }
    }
}


impl<'a, R: Read> serde::Deserializer for Deserializer<'a, R> {
    type Error = DeserializeError;

    #[inline]
    fn visit<V>(&mut self, _visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        Err(serde::de::Error::syntax_error())
    }

    fn visit_bool<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        let value: u8 = try!(serde::Deserialize::deserialize(self));
        match value {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            value => {
                Err(DeserializeError::InvalidEncoding(InvalidEncoding {
                    desc: "invalid u8 when decoding bool",
                    detail: Some(format!("Expected 0 or 1, got {}", value))
                }))
            }
        }
    }

    #[inline]
    fn visit_u8<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        try!(self.read_type::<u8>());
        visitor.visit_u8(try!(self.reader.read_u8()))
    }

    impl_nums!(u16, visit_u16, read_u16);
    impl_nums!(u32, visit_u32, read_u32);
    impl_nums!(u64, visit_u64, read_u64);

    #[inline]
    fn visit_usize<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        try!(self.read_type::<u64>());
        let value = try!(self.reader.read_u64::<BigEndian>());
        match num::cast(value) {
            Some(value) => visitor.visit_usize(value),
            None => Err(serde::de::Error::syntax_error()),
        }
    }

    #[inline]
    fn visit_i8<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        try!(self.read_type::<i8>());
        visitor.visit_i8(try!(self.reader.read_i8()))
    }

    impl_nums!(i16, visit_i16, read_i16);
    impl_nums!(i32, visit_i32, read_i32);
    impl_nums!(i64, visit_i64, read_i64);

    #[inline]
    fn visit_isize<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        try!(self.read_type::<i64>());
        let value = try!(self.reader.read_i64::<BigEndian>());
        match num::cast(value) {
            Some(value) => visitor.visit_isize(value),
            None => Err(serde::de::Error::syntax_error()),
        }
    }

    impl_nums!(f32, visit_f32, read_f32);
    impl_nums!(f64, visit_f64, read_f64);

    fn visit_unit<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        visitor.visit_unit()
    }

    fn visit_char<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        use std::str;

        let error = DeserializeError::InvalidEncoding(InvalidEncoding {
            desc: "Invalid char encoding",
            detail: None
        });

        let mut buf = [0];

        let _ = try!(self.reader.read(&mut buf[..]));
        let first_byte = buf[0];
        let width = utf8_char_width(first_byte);
        if width == 1 { return visitor.visit_char(first_byte as char) }
        if width == 0 { return Err(error)}

        let mut buf = [first_byte, 0, 0, 0];
        {
            let mut start = 1;
            while start < width {
                match try!(self.reader.read(&mut buf[start .. width])) {
                    n if n == width - start => break,
                    n if n < width - start => { start += n; }
                    _ => return Err(error)
                }
            }
        }

        let res = try!(match str::from_utf8(&buf[..width]).ok() {
            Some(s) => Ok(s.chars().next().unwrap()),
            None => Err(error)
        });

        try!(self.read_bytes(res.len_utf8() as u64));
        visitor.visit_char(res)
    }

    fn visit_string<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        let len = try!(serde::Deserialize::deserialize(self));
        try!(self.read_bytes(len));

        let mut buffer = Vec::new();
        try!(self.reader.by_ref().take(len as u64).read_to_end(&mut buffer));

        match String::from_utf8(buffer) {
            Ok(s) => visitor.visit_string(s),
            Err(err) => Err(DeserializeError::InvalidEncoding(InvalidEncoding {
                desc: "error while decoding utf8 string",
                detail: Some(format!("Deserialize error: {}", err))
            })),
        }
    }

    fn visit_enum<V>(&mut self, _enum: &str, mut visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::EnumVisitor,
    {
        visitor.visit(self)
    }

    fn visit_tuple<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        struct TupleVisitor<'a, 'b: 'a, R: Read + 'b>(&'a mut Deserializer<'b, R>);
 
        impl<'a, 'b: 'a, R: Read + 'b> serde::de::SeqVisitor for TupleVisitor<'a, 'b, R> {
            type Error = DeserializeError;

            fn visit<T>(&mut self) -> Result<Option<T>, Self::Error>
                where T: serde::de::Deserialize,
            {
                let value = try!(serde::Deserialize::deserialize(self.0));
                Ok(Some(value))
            }

            fn end(&mut self) -> Result<(), Self::Error> {
                Ok(())
            }
        }

        visitor.visit_seq(TupleVisitor(self))
    }

    fn visit_named_seq<V>(&mut self, _name: &str, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        self.visit_tuple(visitor)
    }

    fn visit_option<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        let value: u8 = try!(serde::de::Deserialize::deserialize(self));
        match value {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(DeserializeError::InvalidEncoding(InvalidEncoding {
                desc: "invalid tag when decoding Option",
                detail: Some(format!("Expected 0 or 1, got {}", value))
            })),
        }
    }

    fn visit_seq<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        struct SeqVisitor<'a, 'b: 'a, R: Read + 'b> {
            deserializer: &'a mut Deserializer<'b, R>,
            len: usize,
        }
 
        impl<'a, 'b: 'a, R: Read + 'b> serde::de::SeqVisitor for SeqVisitor<'a, 'b, R> {
            type Error = DeserializeError;

            fn visit<T>(&mut self) -> Result<Option<T>, Self::Error>
                where T: serde::de::Deserialize,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = try!(serde::Deserialize::deserialize(self.deserializer));
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn end(&mut self) -> Result<(), Self::Error> {
                if self.len == 0 {
                    Ok(())
                } else {
                    Err(serde::de::Error::syntax_error())
                }
            }
        }

        let len = try!(serde::Deserialize::deserialize(self));

        visitor.visit_seq(SeqVisitor { deserializer: self, len: len })
    }

    fn visit_map<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        struct MapVisitor<'a, 'b: 'a, R: Read + 'b> {
            deserializer: &'a mut Deserializer<'b, R>,
            len: usize,
        }
 
        impl<'a, 'b: 'a, R: Read + 'b> serde::de::MapVisitor for MapVisitor<'a, 'b, R> {
            type Error = DeserializeError;

            fn visit_key<K>(&mut self) -> Result<Option<K>, Self::Error>
                where K: serde::de::Deserialize,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let key = try!(serde::Deserialize::deserialize(self.deserializer));
                    Ok(Some(key))
                } else {
                    Ok(None)
                }
            }

            fn visit_value<V>(&mut self) -> Result<V, Self::Error>
                where V: serde::de::Deserialize,
            {
                let value = try!(serde::Deserialize::deserialize(self.deserializer));
                Ok(value)
            }

            fn end(&mut self) -> Result<(), Self::Error> {
                if self.len == 0 {
                    Ok(())
                } else {
                    Err(serde::de::Error::syntax_error())
                }
            }
        }

        let len = try!(serde::Deserialize::deserialize(self));

        visitor.visit_map(MapVisitor { deserializer: self, len: len })
    }

    fn visit_named_map<V>(&mut self,
                          _name: &str,
                          fields: &'static [&'static str],
                          mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        struct StructVisitor<'a, 'b: 'a, R: Read + 'b> {
            deserializer: &'a mut Deserializer<'b, R>,
            index: usize,
            fields: &'static [&'static str],
        }
 
        impl<'a, 'b: 'a, R: Read + 'b> serde::de::MapVisitor for StructVisitor<'a, 'b, R> {
            type Error = DeserializeError;

            fn visit_key<K>(&mut self) -> Result<Option<K>, Self::Error>
                where K: serde::de::Deserialize,
            {
                if self.index < self.fields.len() {
                    let mut deserializer = self.index.into_deserializer();
                    self.index += 1;

                    let key = try!(serde::Deserialize::deserialize(&mut deserializer));
                    Ok(Some(key))
                } else {
                    Ok(None)
                }
            }

            fn visit_value<V>(&mut self) -> Result<V, Self::Error>
                where V: serde::de::Deserialize,
            {
                let value = try!(serde::Deserialize::deserialize(self.deserializer));
                Ok(value)
            }

            fn end(&mut self) -> Result<(), Self::Error> {
                if self.index < self.fields.len() {
                    Err(serde::de::Error::missing_field_error(self.fields[self.index]))
                } else {
                    Ok(())
                }
            }
        }

        visitor.visit_map(StructVisitor {
            deserializer: self,
            index: 0,
            fields: fields,
        })
    }
}

impl<'a, R: Read> serde::de::VariantVisitor for Deserializer<'a, R> {
    type Error = DeserializeError;

    fn visit_variant<V>(&mut self) -> Result<V, Self::Error>
        where V: serde::Deserialize,
    {
        let index: u32 = try!(serde::Deserialize::deserialize(self));
        let mut deserializer = (index as usize).into_deserializer();
        Ok(try!(serde::Deserialize::deserialize(&mut deserializer)))
    }

    fn visit_unit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_seq<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor,
    {
        serde::de::Deserializer::visit_named_seq(self, "", visitor)
    }

    fn visit_map<V>(&mut self,
                    fields: &'static [&'static str],
                    visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor,
    {
        serde::de::Deserializer::visit_named_map(self, "", fields, visitor)
    }
    
}

pub fn from_reader<R, T>(reader: &mut R, size_limit: SizeLimit) -> DeserializeResult<T>
    where R: io::Read,
          T: serde::Deserialize,
{
    let mut deserializer = Deserializer::new(reader, size_limit);
    serde::Deserialize::deserialize(&mut deserializer)
}

pub fn from_slice<T>(bytes: &[u8], size_limit: SizeLimit) -> DeserializeResult<T>
    where T: serde::Deserialize,
{
    let mut reader = bytes;
    from_reader(&mut reader, size_limit)
}

static UTF8_CHAR_WIDTH: [u8; 256] = [
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
