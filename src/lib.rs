#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate "rustc-serialize" as rustc_serialize;

use std::io::{Buffer, MemWriter};
use rustc_serialize::{Encodable, Decodable};

pub use writer::{EncoderWriter, EncodingResult, EncodingError};
pub use reader::{DecoderReader, DecodingResult, DecodingError};
use writer::SizeChecker;

mod writer;
mod reader;
#[cfg(test)] mod test;

#[derive(Clone, Copy)]
pub enum SizeLimit {
    Infinite,
    UpperBound(u64)
}

/// Encodes an encodable object into a `Vec` of bytes.
///
/// If the encoding would take more bytes than allowed by `size_limit`,
/// an error is returned.
pub fn encode<T: Encodable>(t: &T, size_limit: SizeLimit) -> EncodingResult<Vec<u8>> {
    let mut w = MemWriter::new();
    match encode_into(t, &mut w, size_limit) {
        Ok(()) => Ok(w.into_inner()),
        Err(e) => Err(e)
    }
}

/// Decodes a slice of bytes into an object.
pub fn decode<T: Decodable>(b: &[u8]) -> DecodingResult<T> {
    let mut b = b;
    decode_from(&mut b, SizeLimit::Infinite)
}

/// Encodes an object directly into a `Writer`.
///
/// If the encoding would take more bytes than allowed by `size_limit`, an error
/// is returned and *no bytes* will be written into the `Writer`.
pub fn encode_into<T: Encodable, W: Writer>(t: &T, w: &mut W, size_limit: SizeLimit) -> EncodingResult<()> {
    try!(match size_limit {
        SizeLimit::Infinite => Ok(()),
        SizeLimit::UpperBound(x) => {
            let mut size_checker = SizeChecker::new(x);
            t.encode(&mut size_checker)
        }
    });

    t.encode(&mut writer::EncoderWriter::new(w, size_limit))
}

/// Decoes an object directly from a Buffered Reader.
///
/// If the provided `SizeLimit` is reached, the decode will bail immediately.
/// A SizeLimit can help prevent an attacker from flooding your server with
/// a neverending stream of values that runs your server out of memory.
pub fn decode_from<R: Buffer, T: Decodable>(r: &mut R, size_limit: SizeLimit) ->
DecodingResult<T> {
    Decodable::decode(&mut reader::DecoderReader::new(r, size_limit))
}
