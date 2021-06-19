use crate::error::Error;

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error>;
}

pub struct SliceWriter<'storage> {
    slice: &'storage mut [u8],
}

impl<'storage> SliceWriter<'storage> {
    pub(crate) fn new(bytes: &'storage mut [u8]) -> SliceWriter<'storage> {
        SliceWriter {
            slice: bytes,
        }
    }
}

impl<'storage> Writer for SliceWriter<'storage> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        if bytes.len() > self.slice.len() {
            return Err(Error::UnexpectedEnd);
        }
        let data = core::mem::take(&mut self.slice);
        let (write_slice, remaining) = data.split_at_mut(bytes.len());
        write_slice.copy_from_slice(bytes);
        self.slice = remaining;
        Ok(())
    }
}