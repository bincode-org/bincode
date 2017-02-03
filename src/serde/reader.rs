use std::io::Read;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use std::convert::From;

use byteorder::{BigEndian, ReadBytesExt};
use serde_crate as serde;
use serde_crate::de::value::ValueDeserializer;
use serde_crate::de::Error as DeError;
use ::SizeLimit;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct InvalidEncoding {
    pub desc: &'static str,
    pub detail: Option<String>,
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
    Custom(String)
}

impl Error for DeserializeError {
    fn description(&self) -> &str {
        match *self {
            DeserializeError::IoError(ref err) => Error::description(err),
            DeserializeError::InvalidEncoding(ref ib) => ib.desc,
            DeserializeError::SizeLimit => "the size limit for decoding has been reached",
            DeserializeError::Custom(ref msg) => msg,

        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DeserializeError::IoError(ref err) => err.cause(),
            DeserializeError::InvalidEncoding(_) => None,
            DeserializeError::SizeLimit => None,
            DeserializeError::Custom(_) => None,
        }
    }
}

impl From<IoError> for DeserializeError {
    fn from(err: IoError) -> DeserializeError {
        DeserializeError::IoError(err)
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
            DeserializeError::Custom(ref s) =>
                s.fmt(fmt),
        }
    }
}

impl serde::de::Error for DeserializeError {
    fn custom<T: fmt::Display>(desc: T) -> DeserializeError {
        DeserializeError::Custom(desc.to_string())
    }
}

pub type DeserializeResult<T> = Result<T, DeserializeError>;


/// A Deserializer that reads bytes from a buffer.
///
/// This struct should rarely be used.
/// In most cases, prefer the `decode_from` function.
///
/// ```rust,ignore
/// let d = Deserializer::new(&mut some_reader, SizeLimit::new());
/// serde::Deserialize::deserialize(&mut deserializer);
/// let bytes_read = d.bytes_read();
/// ```
pub struct Deserializer<R> {
    reader: R,
    size_limit: SizeLimit,
    read: u64
}

impl<R: Read> Deserializer<R> {
    pub fn new(r: R, size_limit: SizeLimit) -> Deserializer<R> {
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

    fn read_string(&mut self) -> DeserializeResult<String> {
        let len = try!(serde::Deserialize::deserialize(&mut *self));
        try!(self.read_bytes(len));

        let mut buffer = Vec::new();
        try!(self.reader.by_ref().take(len as u64).read_to_end(&mut buffer));

        String::from_utf8(buffer).map_err(|err|
            DeserializeError::InvalidEncoding(InvalidEncoding {
                desc: "error while decoding utf8 string",
                detail: Some(format!("Deserialize error: {}", err))
            }))
    }
}

macro_rules! impl_nums {
    ($ty:ty, $dser_method:ident, $visitor_method:ident, $reader_method:ident) => {
        #[inline]
        fn $dser_method<V>(self, visitor: V) -> DeserializeResult<V::Value>
            where V: serde::de::Visitor,
        {
            try!(self.read_type::<$ty>());
            let value = try!(self.reader.$reader_method::<BigEndian>());
            visitor.$visitor_method(value)
        }
    }
}


impl<'a, R: Read> serde::Deserializer for &'a mut Deserializer<R> {
    type Error = DeserializeError;

    #[inline]
    fn deserialize<V>(self, _visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        let message = "bincode does not support Deserializer::deserialize";
        Err(DeserializeError::custom(message))
    }

    fn deserialize_bool<V>(self, visitor: V) -> DeserializeResult<V::Value>
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

    impl_nums!(u16, deserialize_u16, visit_u16, read_u16);
    impl_nums!(u32, deserialize_u32, visit_u32, read_u32);
    impl_nums!(u64, deserialize_u64, visit_u64, read_u64);
    impl_nums!(i16, deserialize_i16, visit_i16, read_i16);
    impl_nums!(i32, deserialize_i32, visit_i32, read_i32);
    impl_nums!(i64, deserialize_i64, visit_i64, read_i64);
    impl_nums!(f32, deserialize_f32, visit_f32, read_f32);
    impl_nums!(f64, deserialize_f64, visit_f64, read_f64);


    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        try!(self.read_type::<u8>());
        visitor.visit_u8(try!(self.reader.read_u8()))
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        try!(self.read_type::<i8>());
        visitor.visit_i8(try!(self.reader.read_i8()))
    }

    fn deserialize_unit<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        visitor.visit_unit()
    }

    fn deserialize_char<V>(self, visitor: V) -> DeserializeResult<V::Value>
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

        visitor.visit_char(res)
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        visitor.visit_str(&try!(self.read_string()))
    }

    fn deserialize_string<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        visitor.visit_string(try!(self.read_string()))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_enum<V>(self,
                     _enum: &'static str,
                     _variants: &'static [&'static str],
                     visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor,
    {
        impl<'a, R: Read + 'a> serde::de::EnumVisitor for &'a mut Deserializer<R> {
            type Error = DeserializeError;
            type Variant = Self;

            fn visit_variant_seed<V>(self, seed: V) -> DeserializeResult<(V::Value, Self::Variant)>
                where V: serde::de::DeserializeSeed,
            {
                let idx: u32 = try!(serde::de::Deserialize::deserialize(&mut *self));
                let val: Result<_, DeserializeError> = seed.deserialize(idx.into_deserializer());
                Ok((try!(val), self))
            }
        }

        visitor.visit_enum(self)
    }
    
    fn deserialize_tuple<V>(self,
                      _len: usize,
                      visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        struct TupleVisitor<'a, R: Read + 'a>(&'a mut Deserializer<R>);

        impl<'a, 'b: 'a, R: Read + 'b> serde::de::SeqVisitor for TupleVisitor<'a, R> {
            type Error = DeserializeError;

            fn visit_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
                where T: serde::de::DeserializeSeed,
            {
                let value = try!(serde::de::DeserializeSeed::deserialize(seed, &mut *self.0));
                Ok(Some(value))
            }
        }

        visitor.visit_seq(TupleVisitor(self))
    }

    fn deserialize_seq_fixed_size<V>(self,
                            len: usize,
                            visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        struct SeqVisitor<'a, R: Read + 'a> {
            deserializer: &'a mut Deserializer<R>,
            len: usize,
        }

        impl<'a, 'b: 'a, R: Read + 'b> serde::de::SeqVisitor for SeqVisitor<'a, R> {
            type Error = DeserializeError;

            fn visit_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
                where T: serde::de::DeserializeSeed,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = try!(serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer));
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
        }

        visitor.visit_seq(SeqVisitor { deserializer: self, len: len })
    }

    fn deserialize_option<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        let value: u8 = try!(serde::de::Deserialize::deserialize(&mut *self));
        match value {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(&mut *self),
            _ => Err(DeserializeError::InvalidEncoding(InvalidEncoding {
                desc: "invalid tag when decoding Option",
                detail: Some(format!("Expected 0 or 1, got {}", value))
            })),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        let len = try!(serde::Deserialize::deserialize(&mut *self));

        self.deserialize_seq_fixed_size(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        struct MapVisitor<'a, R: Read + 'a> {
            deserializer: &'a mut Deserializer<R>,
            len: usize,
        }

        impl<'a, 'b: 'a, R: Read + 'b> serde::de::MapVisitor for MapVisitor<'a, R> {
            type Error = DeserializeError;

            fn visit_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
                where K: serde::de::DeserializeSeed,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let key = try!(serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer));
                    Ok(Some(key))
                } else {
                    Ok(None)
                }
            }

            fn visit_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
                where V: serde::de::DeserializeSeed,
            {
                let value = try!(serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer));
                Ok(value)
            }
        }

        let len = try!(serde::Deserialize::deserialize(&mut *self));

        visitor.visit_map(MapVisitor { deserializer: self, len: len })
    }

    fn deserialize_struct<V>(self,
                       _name: &str,
                       fields: &'static [&'static str],
                       visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_struct_field<V>(self,
                                   _visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        let message = "bincode does not support Deserializer::deserialize_struct_field";
        Err(DeserializeError::custom(message))
    }

    fn deserialize_newtype_struct<V>(self,
                               _name: &str,
                               visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_unit_struct<V>(self,
                                  _name: &'static str,
                                  visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        visitor.visit_unit()
    }

    fn deserialize_tuple_struct<V>(self,
                                   _name: &'static str,
                                   len: usize,
                                   visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_ignored_any<V>(self,
                                  _visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor,
    {
        let message = "bincode does not support Deserializer::deserialize_ignored_any";
        Err(DeserializeError::custom(message))
    }
}

impl<'a, R: Read> serde::de::VariantVisitor for &'a mut Deserializer<R> {
    type Error = DeserializeError;

    fn visit_unit(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_newtype_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
        where T: serde::de::DeserializeSeed,
    {
        serde::de::DeserializeSeed::deserialize(seed, self)
    }

    fn visit_tuple<V>(self,
                      len: usize,
                      visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn visit_struct<V>(self,
                       fields: &'static [&'static str],
                       visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
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
