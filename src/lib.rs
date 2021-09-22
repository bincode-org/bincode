#![no_std]

//! Bincode is a crate for encoding and decoding using a tiny binary
//! serialization strategy.  Using it, you can easily go from having
//! an object in memory, quickly serialize it to bytes, and then
//! deserialize it back just as fast!

#![doc(html_root_url = "https://docs.rs/bincode/2.0.0-dev")]
#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod config;
pub mod de;
pub mod enc;
pub mod error;

pub use bincode_derive::{Decodable, Encodable};
use config::Config;

pub(crate) mod varint;

pub fn encode_into_slice<E: enc::Encodeable>(
    val: E,
    dst: &mut [u8],
) -> Result<usize, error::EncodeError> {
    encode_into_slice_with_config(val, dst, config::Default)
}

pub fn encode_into_slice_with_config<E: enc::Encodeable, C: Config>(
    val: E,
    dst: &mut [u8],
    _config: C,
) -> Result<usize, error::EncodeError> {
    let writer = enc::write::SliceWriter::new(dst);
    let mut encoder = enc::Encoder::<_, C>::new(writer);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

pub fn decode<'__de, D: de::BorrowDecodable<'__de>>(
    src: &'__de mut [u8],
) -> Result<D, error::DecodeError> {
    decode_with_config(src, config::Default)
}

pub fn decode_with_config<'__de, D: de::BorrowDecodable<'__de>, C: Config>(
    src: &'__de mut [u8],
    _config: C,
) -> Result<D, error::DecodeError> {
    let reader = de::read::SliceReader::new(src);
    let mut decoder = de::Decoder::<_, C>::new(reader, _config);
    D::borrow_decode(&mut decoder)
}

#[cfg(feature = "std")]
pub fn decode_from<'__de, D: de::Decodable<'__de>, R: std::io::Read>(
    src: &'__de mut R,
) -> Result<D, error::DecodeError> {
    decode_from_with_config(src, config::Default)
}

#[cfg(feature = "std")]
pub fn decode_from_with_config<'__de, D: de::Decodable<'__de>, C: Config, R: std::io::Read>(
    src: &'__de mut R,
    _config: C,
) -> Result<D, error::DecodeError> {
    let mut decoder = de::Decoder::<_, C>::new(src, _config);
    D::decode(&mut decoder)
}
