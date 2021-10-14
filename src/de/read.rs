//! This module contains reader-based structs and traits.
//!
//! Because `std::io::Read` is only limited to `std` and not `core`, we provide 2 alternative readers.
//!
//! [Reader] is a reader for sources that do not own their data. It is assumed that the reader's data is dropped after the `read` method is called. This reader is incapable of reading borrowed data, like `&str` and `&[u8]`.
//!
//! [BorrowReader] is an extension of `Reader` that also allows returning borrowed data. A `BorrowReader` allows reading `&str` and `&[u8]`.
//!
//! Specifically the `Reader` trait is used by [Decodable] and the `BorrowReader` trait is used by `[BorrowDecodable]`.
//!
//! [Decodable]: ../trait.Decodable.html
//! [BorrowDecodable]: ../trait.BorrowDecodable.html

use crate::error::DecodeError;

/// A reader for owned data. See the module documentation for more information.
pub trait Reader<'storage> {
    /// Fill the given `bytes` argument with values. Exactly the length of the given slice must be filled, or else an error must be returned.
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError>;
}

/// A reader for borrowed data. Implementors of this must also implement the [Reader] trait. See the module documentation for more information.
pub trait BorrowReader<'storage>: Reader<'storage> {
    /// Read exactly `length` bytes and return a slice to this data. If not enough bytes could be read, an error should be returned.
    ///
    /// *note*: Exactly `length` bytes must be returned. If less bytes are returned, bincode may panic. If more bytes are returned, the excess bytes may be discarded.
    fn take_bytes(&mut self, length: usize) -> Result<&'storage [u8], DecodeError>;
}

/// A reader type for `&[u8]` slices. Implements both [Reader] and [BorrowReader], and thus can be used for borrowed data.
pub struct SliceReader<'storage> {
    slice: &'storage [u8],
}

impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
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
