use crate::error::Result;
use std::io;

/// An optional Read trait for advanced Bincode usage.
///
/// It is highly recommended to use bincode with `io::Read` or `&[u8]` before
/// implementing a custom `BincodeRead`.
///
/// The forward_read_* methods are necessary because some byte sources want
/// to pass a long-lived borrow to the visitor and others want to pass a
/// transient slice.
pub trait BincodeRead<'storage>: io::Read {
    /// Check that the next `length` bytes are a valid string and pass
    /// it on to the serde reader.
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>;

    /// Transfer ownership of the next `length` bytes to the caller.
    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>>;

    /// Pass a slice of the next `length` bytes on to the serde reader.
    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>;

    /// Read a single byte
    fn bincode_read_u8(&mut self) -> Result<u8> {
        <Self as ::byteorder::ReadBytesExt>::read_u8(self).map_err(Into::into)
    }

    /// Read a u16
    fn bincode_read_u16<O: ::byteorder::ByteOrder>(&mut self) -> Result<u16> {
        <Self as ::byteorder::ReadBytesExt>::read_u16::<O>(self).map_err(Into::into)
    }

    /// Read a u32
    fn bincode_read_u32<O: ::byteorder::ByteOrder>(&mut self) -> Result<u32> {
        <Self as ::byteorder::ReadBytesExt>::read_u32::<O>(self).map_err(Into::into)
    }

    /// Read a u64
    fn bincode_read_u64<O: ::byteorder::ByteOrder>(&mut self) -> Result<u64> {
        <Self as ::byteorder::ReadBytesExt>::read_u64::<O>(self).map_err(Into::into)
    }

    serde_if_integer128! {
        /// Read a u128
        fn bincode_read_u128<O: ::byteorder::ByteOrder>(&mut self) -> Result<u128> {
            <Self as ::byteorder::ReadBytesExt>::read_u128::<O>(self).map_err(Into::into)
        }
    }
}

impl<'a, 'storage, T> BincodeRead<'storage> for &'a mut T
where
    T: BincodeRead<'storage>,
{
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        (*self).forward_read_str(length, visitor)
    }

    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
        (*self).get_byte_buffer(length)
    }

    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        (*self).forward_read_bytes(length, visitor)
    }

    #[inline]
    fn bincode_read_u8(&mut self) -> Result<u8> {
        (*self).bincode_read_u8()
    }

    fn bincode_read_u16<O: ::byteorder::ByteOrder>(&mut self) -> Result<u16> {
        (*self).bincode_read_u16::<O>()
    }

    fn bincode_read_u32<O: ::byteorder::ByteOrder>(&mut self) -> Result<u32> {
        (*self).bincode_read_u32::<O>()
    }

    fn bincode_read_u64<O: ::byteorder::ByteOrder>(&mut self) -> Result<u64> {
        (*self).bincode_read_u64::<O>()
    }

    serde_if_integer128! {
        fn bincode_read_u128<O: ::byteorder::ByteOrder>(&mut self) -> Result<u128> {
            (*self).bincode_read_u128::<O>()
        }
    }
}

/// A BincodeRead implementation for byte slices
pub struct SliceReader<'storage> {
    slice: &'storage [u8],
}

/// A BincodeRead implementation for `io::Read`ers
pub struct IoReader<R> {
    reader: R,
    temp_buffer: Vec<u8>,
}

impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub(crate) fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader { slice: bytes }
    }

    #[inline(always)]
    fn get_byte_slice(&mut self, length: usize) -> Result<&'storage [u8]> {
        if length > self.slice.len() {
            return Err(SliceReader::unexpected_eof());
        }
        let (read_slice, remaining) = self.slice.split_at(length);
        self.slice = remaining;
        Ok(read_slice)
    }

    #[inline]
    pub(crate) fn is_finished(&self) -> bool {
        self.slice.is_empty()
    }
}

impl<R> IoReader<R> {
    /// Constructs an IoReadReader
    pub(crate) fn new(r: R) -> IoReader<R> {
        IoReader {
            reader: r,
            temp_buffer: vec![],
        }
    }
}

impl<'storage> io::Read for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.len() > self.slice.len() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        let (read_slice, remaining) = self.slice.split_at(out.len());
        out.copy_from_slice(read_slice);
        self.slice = remaining;

        Ok(out.len())
    }

    #[inline(always)]
    fn read_exact(&mut self, out: &mut [u8]) -> io::Result<()> {
        self.read(out).map(|_| ())
    }
}

impl<R: io::Read> io::Read for IoReader<R> {
    #[inline(always)]
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        self.reader.read(out)
    }
    #[inline(always)]
    fn read_exact(&mut self, out: &mut [u8]) -> io::Result<()> {
        self.reader.read_exact(out)
    }
}

impl<'storage> SliceReader<'storage> {
    #[inline(never)]
    #[cold]
    fn unexpected_eof() -> Box<crate::ErrorKind> {
        Box::new(crate::ErrorKind::Io(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "",
        )))
    }
}

impl<'storage> BincodeRead<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        use crate::ErrorKind;
        let string = match ::std::str::from_utf8(self.get_byte_slice(length)?) {
            Ok(s) => s,
            Err(e) => return Err(ErrorKind::InvalidUtf8Encoding(e).into()),
        };
        visitor.visit_borrowed_str(string)
    }

    #[inline(always)]
    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
        self.get_byte_slice(length).map(|x| x.to_vec())
    }

    #[inline(always)]
    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        visitor.visit_borrowed_bytes(self.get_byte_slice(length)?)
    }

    fn bincode_read_u64<O: ::byteorder::ByteOrder>(&mut self) -> Result<u64> {
        if self.slice.len() < 8 {
            Err(Self::unexpected_eof())
        } else {
            let (this, remaining) = self.slice.split_at(8);
            self.slice = remaining;
            Ok(O::read_u64(this))
        }
    }

    fn bincode_read_u32<O: ::byteorder::ByteOrder>(&mut self) -> Result<u32> {
        if self.slice.len() < 4 {
            Err(Self::unexpected_eof())
        } else {
            let (this, remaining) = self.slice.split_at(4);
            self.slice = remaining;
            Ok(O::read_u32(this))
        }
    }

    fn bincode_read_u16<O: ::byteorder::ByteOrder>(&mut self) -> Result<u16> {
        if self.slice.len() < 2 {
            Err(Self::unexpected_eof())
        } else {
            let (this, remaining) = self.slice.split_at(2);
            self.slice = remaining;
            Ok(O::read_u16(this))
        }
    }

    #[inline]
    fn bincode_read_u8(&mut self) -> Result<u8> {
        if self.slice.is_empty() {
            Err(Self::unexpected_eof())
        } else {
            let v = self.slice[0];
            self.slice = &self.slice[1..];
            Ok(v)
        }
    }
}

impl<R> IoReader<R>
where
    R: io::Read,
{
    fn fill_buffer(&mut self, length: usize) -> Result<()> {
        self.temp_buffer.resize(length, 0);

        self.reader.read_exact(&mut self.temp_buffer)?;

        Ok(())
    }
}

impl<'a, R> BincodeRead<'a> for IoReader<R>
where
    R: io::Read,
{
    fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'a>,
    {
        self.fill_buffer(length)?;

        let string = match ::std::str::from_utf8(&self.temp_buffer[..]) {
            Ok(s) => s,
            Err(e) => return Err(crate::ErrorKind::InvalidUtf8Encoding(e).into()),
        };

        visitor.visit_str(string)
    }

    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
        self.fill_buffer(length)?;
        Ok(::std::mem::replace(&mut self.temp_buffer, Vec::new()))
    }

    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'a>,
    {
        self.fill_buffer(length)?;
        visitor.visit_bytes(&self.temp_buffer[..])
    }
}

#[cfg(test)]
mod test {
    use super::IoReader;

    #[test]
    fn test_fill_buffer() {
        let buffer = vec![0u8; 64];
        let mut reader = IoReader::new(buffer.as_slice());

        reader.fill_buffer(20).unwrap();
        assert_eq!(20, reader.temp_buffer.len());

        reader.fill_buffer(30).unwrap();
        assert_eq!(30, reader.temp_buffer.len());

        reader.fill_buffer(5).unwrap();
        assert_eq!(5, reader.temp_buffer.len());
    }
}
