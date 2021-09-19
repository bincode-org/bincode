use crate::error::EncodeError;

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError>;
}

pub struct SliceWriter<'storage> {
    slice: &'storage mut [u8],
    idx: usize,
}

impl<'storage> SliceWriter<'storage> {
    pub(crate) fn new(bytes: &'storage mut [u8]) -> SliceWriter<'storage> {
        SliceWriter {
            slice: bytes,
            idx: 0,
        }
    }

    pub(crate) fn bytes_written(&self) -> usize {
        self.idx
    }
}

impl<'storage> Writer for SliceWriter<'storage> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        let remaining = &mut self.slice[self.idx..];
        if bytes.len() > remaining.len() {
            return Err(EncodeError::UnexpectedEnd);
        }
        self.idx += bytes.len();
        let write_slice = &mut remaining[..bytes.len()];
        write_slice.copy_from_slice(bytes);
        Ok(())
    }
}
