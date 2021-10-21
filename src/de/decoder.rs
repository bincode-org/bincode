use super::{
    read::{BorrowReader, Reader},
    BorrowDecoder, Decoder,
};
use crate::{config::Config, utils::Sealed};

/// A Decoder that reads bytes from a given reader `R`.
///
/// This struct should rarely be used.
/// In most cases, prefer any of the `decode` functions.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to read integers out of the reader.
///
/// ```
/// # let slice: &[u8] = &[0, 0, 0, 0];
/// # let some_reader = bincode::de::read::SliceReader::new(slice);
/// use bincode::de::{DecoderImpl, Decode};
/// use bincode::config;
/// let mut decoder = DecoderImpl::new(some_reader, config::Configuration::standard());
/// // this u32 can be any Decode
/// let value = u32::decode(&mut decoder).unwrap();
/// ```
pub struct DecoderImpl<R, C: Config> {
    reader: R,
    config: C,
}

impl<R: Reader, C: Config> DecoderImpl<R, C> {
    /// Construct a new Decoder
    pub fn new(reader: R, config: C) -> DecoderImpl<R, C> {
        DecoderImpl { reader, config }
    }
}

impl<'a, R, C: Config> Sealed for &'a mut DecoderImpl<R, C> {}

impl<'a, 'de, R: BorrowReader<'de>, C: Config> BorrowDecoder<'de> for &'a mut DecoderImpl<R, C> {
    type BR = R;

    fn borrow_reader(&mut self) -> &mut Self::BR {
        &mut self.reader
    }
}

impl<'a, R: Reader, C: Config> Decoder for &'a mut DecoderImpl<R, C> {
    type R = R;

    type C = C;

    fn reader(&mut self) -> &mut Self::R {
        &mut self.reader
    }

    fn config(&self) -> &Self::C {
        &self.config
    }
}
