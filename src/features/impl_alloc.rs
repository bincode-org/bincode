use crate::{config, enc, error, Config};
use alloc::vec::Vec;

#[derive(Default)]
struct VecWriter {
    inner: Vec<u8>,
}

impl enc::write::Writer for VecWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), error::EncodeError> {
        self.inner.extend_from_slice(bytes);
        Ok(())
    }
}

/// Encode the given value into a `Vec<u8>`.
pub fn encode_to_vec<E: enc::Encodeable>(val: E) -> Result<Vec<u8>, error::EncodeError> {
    encode_to_vec_with_config(val, config::Default)
}

/// Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information.
pub fn encode_to_vec_with_config<E: enc::Encodeable, C: Config>(
    val: E,
    config: C,
) -> Result<Vec<u8>, error::EncodeError> {
    let writer = VecWriter::default();
    let mut encoder = enc::Encoder::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().inner)
}
