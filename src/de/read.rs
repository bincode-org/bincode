use crate::error::DecodeError;

pub trait Reader<'storage> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError>;
}

pub trait BorrowReader<'storage>: Reader<'storage> {
    fn take_bytes(&mut self, length: usize) -> Result<&'storage [u8], DecodeError>;
}

pub struct SliceReader<'storage> {
    slice: &'storage [u8],
}

impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub(crate) fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader { slice: bytes }
    }

    #[inline(always)]
    fn get_byte_slice(&mut self, length: usize) -> Result<&'storage [u8], DecodeError> {
        if length > self.slice.len() {
            return Err(DecodeError::UnexpectedEnd);
        }
        let (read_slice, remaining) = self.slice.split_at(length);
        self.slice = remaining;
        Ok(read_slice)
    }
}

impl<'storage> Reader<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        if bytes.len() > self.slice.len() {
            return Err(DecodeError::UnexpectedEnd);
        }
        let (read_slice, remaining) = self.slice.split_at(bytes.len());
        bytes.copy_from_slice(read_slice);
        self.slice = remaining;

        Ok(())
    }
}

impl<'storage> BorrowReader<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn take_bytes(&mut self, length: usize) -> Result<&'storage [u8], DecodeError> {
        self.get_byte_slice(length)
    }
}

#[cfg(feature = "std")]
impl<'storage, R: std::io::Read> Reader<'storage> for R {
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        match self.read_exact(bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(DecodeError::UnexpectedEnd),
        }
    }
}
