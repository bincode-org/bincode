//! Functions that work with `embedded-io`'s traits.
//!
//! |Trait name|Function name|
//! |----------|-------------|
//! |[`embedded_io::Write`]|[`encode_into_write`]|
//! |[`embedded_io::Read`]|[`decode_from_read`]|
//! |[`embedded_io::BufRead`]|[`decode_from_buf_read`]|
//!
//! Note that all these functions currently block. There is no partial decoding or waiting for a reader/writer to be ready with [`embedded_io::ReadReady`] or [`embedded_io::WriteReady`].

use crate::{
    config::Config,
    de::DecoderImpl,
    enc::EncoderImpl,
    error::{DecodeError, EncodeError},
    Decode, Encode,
};

/// Encode the given value into a [`embedded_io::Write`] with the given `Config`. See the [config] module for more information.
///
/// [config]: ../config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub fn encode_into_write<E: Encode, C: Config, W: embedded_io::Write>(
    val: E,
    dst: &mut W,
    config: C,
) -> Result<usize, EncodeError> {
    let writer = IoWriter {
        writer: dst,
        bytes_written: 0,
    };
    let mut encoder = EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written)
}

struct IoWriter<'a, W: embedded_io::Write> {
    writer: &'a mut W,
    bytes_written: usize,
}

impl<'a, W> crate::enc::write::Writer for IoWriter<'a, W>
where
    W: embedded_io::Write,
{
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.writer
            .write_all(bytes)
            .map_err(|_| EncodeError::EmbeddedIo)?;
        self.bytes_written += bytes.len();
        Ok(())
    }
}

/// Decode type `D` from the given reader with the given `Config`. The reader can be any type that implements [`embedded_io::Read`].
///
/// See the [config] module for more information about config options.
///
/// [config]: ../config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub fn decode_from_read<D: Decode, C: Config, R: embedded_io::Read>(
    src: &mut R,
    config: C,
) -> Result<D, DecodeError> {
    let reader = IoRead { reader: src };
    let mut decoder = DecoderImpl::<_, C>::new(reader, config);
    D::decode(&mut decoder)
}

struct IoRead<'a, R: embedded_io::Read> {
    reader: &'a mut R,
}

impl<'a, R: embedded_io::Read> crate::de::read::Reader for IoRead<'a, R> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        self.reader
            .read_exact(bytes)
            .map_err(|_| DecodeError::EmbeddedIo)
    }
}

/// Decode type `D` from the given reader with the given `Config`. The reader can be any type that implements [`embedded_io::BufRead`].
///
/// See the [config] module for more information about config options.
///
/// [config]: ../config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub fn decode_from_buf_read<D: Decode, C: Config, R: embedded_io::BufRead>(
    src: &mut R,
    config: C,
) -> Result<D, DecodeError> {
    let reader = IoBufRead { reader: src };
    let mut decoder = DecoderImpl::<_, C>::new(reader, config);
    D::decode(&mut decoder)
}

struct IoBufRead<'a, R: embedded_io::BufRead> {
    reader: &'a mut R,
}

impl<'a, R: embedded_io::BufRead> crate::de::read::Reader for IoBufRead<'a, R> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        let mut buf = self
            .reader
            .fill_buf()
            .map_err(|_| DecodeError::EmbeddedIo)?;
        let mut last_len = buf.len();

        // loop to read more until we have enough
        while buf.len() < bytes.len() {
            buf = self
                .reader
                .fill_buf()
                .map_err(|_| DecodeError::EmbeddedIo)?;
            // Detect infinite loops where 0 bytes get read
            if buf.len() == last_len {
                return Err(DecodeError::EmbeddedIo);
            }
            last_len = buf.len();
        }

        bytes.copy_from_slice(&buf[..bytes.len()]);
        self.reader.consume(bytes.len());

        Ok(())
    }
}
