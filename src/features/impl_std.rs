use crate::{
    config::{self, Config},
    de::{read::Reader, Decodable, Decoder},
    error::DecodeError,
};

pub fn decode_from<D: Decodable, R: std::io::Read>(src: &mut R) -> Result<D, DecodeError> {
    decode_from_with_config(src, config::Default)
}

pub fn decode_from_with_config<D: Decodable, C: Config, R: std::io::Read>(
    src: &mut R,
    _config: C,
) -> Result<D, DecodeError> {
    let mut decoder = Decoder::<_, C>::new(src, _config);
    D::decode(&mut decoder)
}

impl<'storage, R: std::io::Read> Reader<'storage> for R {
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        match self.read_exact(bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(DecodeError::UnexpectedEnd),
        }
    }
}