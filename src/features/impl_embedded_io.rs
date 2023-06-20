use crate::{
    config::Config,
    de::{read::Reader, Decode, DecoderImpl},
    enc::{write::Writer, Encode, EncoderImpl},
    error::{DecodeError, EncodeError},
};

/// Decode type `D` from the given reader with the given `Config`. The reader can be any type that implements `embedded_io::blocking::Read`, e.g. `std::fs::File`.
///
/// See the [config] module for more information about config options.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub fn decode_from_embedded_io_read<D: Decode, C: Config, R: embedded_io::blocking::Read>(
    src: &mut R,
    config: C,
) -> Result<D, DecodeError> {
    let reader = EmbeddedIoReader::new(src);
    let mut decoder = DecoderImpl::<_, C>::new(reader, config);
    D::decode(&mut decoder)
}

pub(crate) struct EmbeddedIoReader<R> {
    reader: R,
}

impl<R> EmbeddedIoReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> Reader for EmbeddedIoReader<R>
where
    R: embedded_io::blocking::Read,
{
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        self.reader
            .read_exact(bytes)
            .map_err(|_inner| DecodeError::Io {
                additional: bytes.len(),
            })
    }
}

/// Encode the given value into any type that implements `embedded_io::blocking::Write`, e.g. `std::fs::File`, with the given `Config`.
/// See the [config] module for more information.
/// Returns the amount of bytes written.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
pub fn encode_into_embedded_io_write<E: Encode, C: Config, W: embedded_io::blocking::Write>(
    val: E,
    dst: &mut W,
    config: C,
) -> Result<usize, EncodeError> {
    let writer = IoWriter::new(dst);
    let mut encoder = EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

pub(crate) struct IoWriter<'a, W: embedded_io::blocking::Write> {
    writer: &'a mut W,
    bytes_written: usize,
}

impl<'a, W: embedded_io::blocking::Write> IoWriter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            bytes_written: 0,
        }
    }

    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }
}

impl<'storage, W: embedded_io::blocking::Write> Writer for IoWriter<'storage, W> {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.writer
            .write_all(bytes)
            .map_err(|_inner| EncodeError::Io {
                index: self.bytes_written,
            })?;
        self.bytes_written += bytes.len();
        Ok(())
    }
}
