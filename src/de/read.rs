use crate::imports::io::{ErrorKind, Read};
use crate::imports::str;
use error::Result;
use serde;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::imports::{mem, vec, Vec};

type IoResult<T = ()> = core::result::Result<T, crate::imports::io::Error>;

/// An optional Read trait for advanced Bincode usage.
///
/// It is highly recommended to use bincode with `io::Read` or `&[u8]` before
/// implementing a custom `BincodeRead`.
///
/// The forward_read_* methods are necessary because some byte sources want
/// to pass a long-lived borrow to the visitor and others want to pass a
/// transient slice.
pub trait BincodeRead<'storage>: Read {
    /// Check that the next `length` bytes are a valid string and pass
    /// it on to the serde reader.
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>;

    #[cfg(any(feature = "std", feature = "alloc"))]
    /// Transfer ownership of the next `length` bytes to the caller.
    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>>;

    /// Pass a slice of the next `length` bytes on to the serde reader.
    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>;
}

/// A BincodeRead implementation for byte slices
pub struct SliceReader<'storage> {
    slice: &'storage [u8],
}

impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub(crate) fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader { slice: bytes }
    }

    #[inline(always)]
    fn get_byte_slice(&mut self, length: usize) -> IoResult<&'storage [u8]> {
        if length > self.slice.len() {
            return Err(ErrorKind::UnexpectedEof.into());
        }
        let (read_slice, remaining) = self.slice.split_at(length);
        self.slice = remaining;
        Ok(read_slice)
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.slice.is_empty()
    }
}

impl<'storage> Read for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, out: &mut [u8]) -> IoResult<usize> {
        if out.len() > self.slice.len() {
            return Err(ErrorKind::UnexpectedEof.into());
        }
        let (read_slice, remaining) = self.slice.split_at(out.len());
        out.copy_from_slice(read_slice);
        self.slice = remaining;

        Ok(out.len())
    }

    #[inline(always)]
    fn read_exact(&mut self, out: &mut [u8]) -> IoResult<()> {
        self.read(out).map(|_| ())
    }
}

/// A BincodeRead implementation for `io::Read`ers
pub struct IoReader<R> {
    reader: R,
    #[cfg(any(feature = "std", feature = "alloc"))]
    temp_buffer: Vec<u8>,
}

impl<R> IoReader<R> {
    /// Constructs an IoReadReader
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub(crate) fn new(r: R) -> IoReader<R> {
        IoReader {
            reader: r,
            temp_buffer: vec![],
        }
    }

    /// Constructs an IoReadReader
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    pub(crate) fn new(r: R) -> IoReader<R> {
        IoReader { reader: r }
    }
}

impl<R: Read> Read for IoReader<R> {
    #[inline(always)]
    fn read(&mut self, out: &mut [u8]) -> IoResult<usize> {
        Ok(self.reader.read(out)?)
    }
    #[inline(always)]
    fn read_exact(&mut self, out: &mut [u8]) -> IoResult<()> {
        Ok(self.reader.read_exact(out)?)
    }
}

impl<'storage> SliceReader<'storage> {
    #[inline(always)]
    fn unexpected_eof() -> ErrorKind {
        ErrorKind::UnexpectedEof
    }
}

impl<'storage> BincodeRead<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        use ErrorKind;
        let string = match str::from_utf8(self.get_byte_slice(length)?) {
            Ok(s) => s,
            Err(e) => return Err(ErrorKind::InvalidUtf8Encoding(e).into()),
        };
        visitor.visit_borrowed_str(string)
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[inline(always)]
    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
        let bytes = self.get_byte_slice(length)?;
        Ok(bytes.to_vec())
    }

    #[inline(always)]
    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        visitor.visit_borrowed_bytes(self.get_byte_slice(length)?)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<R> IoReader<R>
where
    R: Read,
{
    fn fill_buffer(&mut self, length: usize) -> Result<()> {
        self.temp_buffer.resize(length, 0);

        self.reader.read_exact(&mut self.temp_buffer)?;

        Ok(())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, R> BincodeRead<'a> for IoReader<R>
where
    R: Read,
{
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'a>,
    {
        self.fill_buffer(length)?;

        let string = match str::from_utf8(&self.temp_buffer[..]) {
            Ok(s) => s,
            Err(e) => return Err(::ErrorKind::InvalidUtf8Encoding(e).into()),
        };

        visitor.visit_str(string)
    }

    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
        self.fill_buffer(length)?;
        Ok(mem::replace(&mut self.temp_buffer, Vec::new()))
    }

    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'a>,
    {
        self.fill_buffer(length)?;
        visitor.visit_bytes(&self.temp_buffer[..])
    }
}

#[cfg(all(test, any(feature = "std", feature = "alloc")))]
mod test {
    use super::IoReader;
    extern crate alloc;

    #[test]
    fn test_fill_buffer() {
        let buffer = alloc::vec![0u8; 64];
        let mut reader = IoReader::new(buffer.as_slice());

        reader.fill_buffer(20).unwrap();
        assert_eq!(20, reader.temp_buffer.len());

        reader.fill_buffer(30).unwrap();
        assert_eq!(30, reader.temp_buffer.len());

        reader.fill_buffer(5).unwrap();
        assert_eq!(5, reader.temp_buffer.len());
    }
}
