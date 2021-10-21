use super::{write::Writer, Encoder};
use crate::{config::Config, utils::Sealed};

/// An Encoder that writes bytes into a given writer `W`.
///
/// This struct should rarely be used.
/// In most cases, prefer any of the `encode` functions.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to write integers to the writer.
///
/// ```
/// # use bincode::enc::{write::SliceWriter, EncoderImpl, Encode};
/// # use bincode::config::{self, Config};
/// # let config = config::Configuration::standard().with_fixed_int_encoding().with_big_endian();
/// let slice: &mut [u8] = &mut [0, 0, 0, 0];
/// let mut encoder = EncoderImpl::new(SliceWriter::new(slice), config);
/// // this u32 can be any Encodable
/// 5u32.encode(&mut encoder).unwrap();
/// assert_eq!(encoder.into_writer().bytes_written(), 4);
/// assert_eq!(slice, [0, 0, 0, 5]);
/// ```
pub struct EncoderImpl<W: Writer, C: Config> {
    writer: W,
    config: C,
}

impl<W: Writer, C: Config> EncoderImpl<W, C> {
    /// Create a new Encoder
    pub fn new(writer: W, config: C) -> EncoderImpl<W, C> {
        EncoderImpl { writer, config }
    }

    /// Return the underlying writer
    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<'a, W: Writer, C: Config> Encoder for &'a mut EncoderImpl<W, C> {
    type W = W;

    type C = C;

    fn writer(&mut self) -> &mut Self::W {
        &mut self.writer
    }

    fn config(&self) -> &Self::C {
        &self.config
    }
}

impl<'a, W: Writer, C: Config> Sealed for &'a mut EncoderImpl<W, C> {}
