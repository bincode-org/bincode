use std::io::{Read as IoRead, Result as IoResult, Error as IoError, ErrorKind as IoErrorKind};
use ::Result;
use serde_crate as serde;

/// A byte-oriented reading trait that is specialized for
/// slices and generic readers.
pub trait BincodeRead<'storage>: IoRead {
    #[doc(hidden)]
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) ->  Result<V::Value>
    where V: serde::de::Visitor<'storage>;

    #[doc(hidden)]
    fn forward_read_bytes<V: serde::de::Visitor<'storage>>(&mut self, length: usize, visitor: V) ->  Result<V::Value>
    where V: serde::de::Visitor<'storage>;
}

/// A BincodeRead implementation for byte slices
pub struct SliceReader<'storage> {
    slice: &'storage [u8]
}

/// A BincodeRead implementation for io::Readers
pub struct IoReadReader<R> {
    reader: R,
    temp_buffer: Vec<u8>,
}

impl <'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader {
            slice: bytes,
        }
    }
}

impl <R> IoReadReader<R> {
    /// Constructs an IoReadReader
    pub fn new(r: R) -> IoReadReader<R> {
        IoReadReader {
            reader: r,
            temp_buffer: vec![],
        }
    }
}

impl <'storage> IoRead for SliceReader<'storage> {
    fn read(&mut self, out: & mut [u8]) -> IoResult<usize> {
        (&mut self.slice).read(out)
    }
}

impl <R: IoRead> IoRead for IoReadReader<R> {
    fn read(&mut self, out: & mut [u8]) -> IoResult<usize> {
        self.reader.read(out)
    }
}

impl <'storage> BincodeRead<'storage> for SliceReader<'storage> {
    fn forward_read_str<V: serde::de::Visitor<'storage>>(&mut self, length: usize, visitor: V) ->  Result<V::Value> {
        use ::ErrorKind;
        if length > self.slice.len() {
            return Err(Box::new(ErrorKind::IoError(IoError::new(IoErrorKind::UnexpectedEof, ""))));
        }

        let string = match ::std::str::from_utf8(&self.slice[..length]) {
            Ok(s) => s,
            Err(_) => return Err(Box::new(ErrorKind::InvalidEncoding {
                desc: "string was not valid utf8",
                detail: None,
            })),
        };
        let r = visitor.visit_borrowed_str(string);
        self.slice = &self.slice[length..];
        r
    }
    fn forward_read_bytes<V: serde::de::Visitor<'storage>>(&mut self, length: usize, visitor: V) ->  Result<V::Value> {
        use ::ErrorKind;
        if length > self.slice.len() {
            return Err(Box::new(ErrorKind::IoError(IoError::new(IoErrorKind::UnexpectedEof, ""))));
        }

        let r = visitor.visit_borrowed_bytes(&self.slice[..length]);
        self.slice = &self.slice[length..];
        r
    }
}

impl <R> BincodeRead<'static> for IoReadReader<R> where R: IoRead {
    fn forward_read_str<V: serde::de::Visitor<'static>>(&mut self, length: usize, visitor: V) ->  Result<V::Value> {
        use ::ErrorKind;
        let current_length = self.temp_buffer.len();
        if length > current_length{
            self.temp_buffer.reserve_exact(length - current_length);
        }

        self.reader.read_exact(&mut self.temp_buffer[..length])?;
        let string = match ::std::str::from_utf8(&self.temp_buffer[..length]) {
            Ok(s) => s,
            Err(_) => return Err(Box::new(ErrorKind::InvalidEncoding {
                desc: "string was not valid utf8",
                detail: None,
            })),
        };

        let r = visitor.visit_str(string);
        r
    }
    fn forward_read_bytes<V: serde::de::Visitor<'static>>(&mut self, length: usize, visitor: V) ->  Result<V::Value> {
        let current_length = self.temp_buffer.len();
        if length > current_length{
            self.temp_buffer.reserve_exact(length - current_length);
        }

        self.reader.read_exact(&mut self.temp_buffer[..length])?;

        let r = visitor.visit_bytes(&self.temp_buffer[..length]);
        r
    }
}
