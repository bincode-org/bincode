use crate::error::Error;

pub trait Reader<'storage> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error>;
    fn forward_read<F, R>(&mut self, length: usize, visitor: F) -> Result<R, Error>
    where
        F: Fn(&'storage [u8]) -> R;
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
    fn get_byte_slice(&mut self, length: usize) -> Result<&'storage [u8], Error> {
        if length > self.slice.len() {
            return Err(Error::UnexpectedEnd);
        }
        let (read_slice, remaining) = self.slice.split_at(length);
        self.slice = remaining;
        Ok(read_slice)
    }
}

impl<'storage> Reader<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn read<'a>(&'a mut self, bytes: &mut [u8]) -> Result<(), Error> {
        if bytes.len() > self.slice.len() {
            return Err(Error::UnexpectedEnd);
        }
        let (read_slice, remaining) = self.slice.split_at(bytes.len());
        bytes.copy_from_slice(read_slice);
        self.slice = remaining;

        Ok(())
    }

    #[inline(always)]
    fn forward_read<F, R>(&mut self, length: usize, visitor: F) -> Result<R, Error>
    where
        F: Fn(&'storage [u8]) -> R,
    {
        Ok(visitor(self.get_byte_slice(length)?))
    }
}