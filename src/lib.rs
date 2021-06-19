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
pub mod error;
pub mod ser;

pub(crate) mod int_encoding;

pub fn encode_into_slice<E: ser::Encodeable>(val: E, dst: &mut [u8]) -> Result<(), error::Error> {
    let writer = ser::SliceWriter::new(dst);
    let mut encoder = ser::Encoder::<_, config::Default>::new(writer);
    val.encode(&mut encoder)
}

pub fn decode<D: de::Decodable>(src: &mut [u8]) -> Result<D, error::Error> {
    let reader = de::SliceReader::new(src);
    let mut decoder = de::Decoder::<_, config::Default>::new(reader);
    D::decode(&mut decoder)
}
