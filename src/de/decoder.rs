use super::{
    read::{BorrowReader, Reader},
    BorrowDecoder, Decoder,
};
use crate::{config::Config, error::DecodeError, utils::Sealed};

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
/// let mut decoder = DecoderImpl::new(some_reader, bincode::config::standard(), ());
/// // this u32 can be any Decode
/// let value = u32::decode(&mut decoder).unwrap();
/// ```
pub struct DecoderImpl<R, C: Config, Ctx> {
    reader: R,
    config: C,
    bytes_read: usize,
    ctx: Ctx,
}

impl<R: Reader, C: Config, Ctx> DecoderImpl<R, C, Ctx> {
    /// Construct a new Decoder
    pub const fn new(reader: R, config: C, ctx: Ctx) -> DecoderImpl<R, C, Ctx> {
        DecoderImpl {
            reader,
            config,
            bytes_read: 0,
            ctx,
        }
    }
}

impl<R, C: Config, Ctx> Sealed for DecoderImpl<R, C, Ctx> {}

impl<'de, R: BorrowReader<'de>, C: Config, Ctx> BorrowDecoder<'de> for DecoderImpl<R, C, Ctx> {
    type BR = R;

    fn borrow_reader(&mut self) -> &mut Self::BR {
        &mut self.reader
    }
}

impl<R: Reader, C: Config, Ctx> Decoder for DecoderImpl<R, C, Ctx> {
    type R = R;

    type C = C;
    type Ctx = Ctx;

    fn reader(&mut self) -> &mut Self::R {
        &mut self.reader
    }

    fn config(&self) -> &Self::C {
        &self.config
    }

    #[inline]
    fn claim_bytes_read(&mut self, n: usize) -> Result<(), DecodeError> {
        // C::LIMIT is a const so this check should get compiled away
        if let Some(limit) = C::LIMIT {
            // Make sure we don't accidentally overflow `bytes_read`
            self.bytes_read = self
                .bytes_read
                .checked_add(n)
                .ok_or(DecodeError::LimitExceeded)?;
            if self.bytes_read > limit {
                Err(DecodeError::LimitExceeded)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    #[inline]
    fn unclaim_bytes_read(&mut self, n: usize) {
        // C::LIMIT is a const so this check should get compiled away
        if C::LIMIT.is_some() {
            // We should always be claiming more than we unclaim, so this should never underflow
            self.bytes_read -= n;
        }
    }

    fn ctx(&mut self) -> &mut Self::Ctx {
        &mut self.ctx
    }
}
