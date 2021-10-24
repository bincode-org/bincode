//! This module contains writer-based structs and traits.
//!
//! Because `std::io::Write` is only limited to `std` and not `core`, we provide our own [Writer].

use crate::error::EncodeError;

/// Trait that indicates that a struct can be used as a destination to encode data too. This is used by [Encode]
///
/// [Encode]: ../trait.Encode.html
pub trait Writer {
    /// Write `bytes` to the underlying writer. Exactly `bytes.len()` bytes must be written, or else an error should be returned.
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError>;
}

/// A helper struct that implements `Writer` for a `&[u8]` slice.
///
/// ```
/// use bincode::enc::write::{Writer, SliceWriter};
///
/// let destination = &mut [0u8; 100];
/// let mut writer = SliceWriter::new(destination);
/// writer.write(&[1, 2, 3, 4, 5]).unwrap();
///
/// assert_eq!(writer.bytes_written(), 5);
/// assert_eq!(destination[0..6], [1, 2, 3, 4, 5, 0]);
/// ```
pub struct SliceWriter<'storage> {
    slice: &'storage mut [u8],
    original_length: usize,
}

impl<'storage> SliceWriter<'storage> {
    /// Create a new instance of `SliceWriter` with the given byte array.
    pub fn new(bytes: &'storage mut [u8]) -> SliceWriter<'storage> {
        let original = bytes.len();
        SliceWriter {
            slice: bytes,
            original_length: original,
        }
    }

    /// Return the amount of bytes written so far.
    pub fn bytes_written(&self) -> usize {
        self.original_length - self.slice.len()
    }
}

impl<'storage> Writer for SliceWriter<'storage> {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        if bytes.len() > self.slice.len() {
            return Err(EncodeError::UnexpectedEnd);
        }
        let (a, b) = core::mem::replace(&mut self.slice, &mut []).split_at_mut(bytes.len());
        a.copy_from_slice(bytes);
        self.slice = b;

        Ok(())
    }
}
