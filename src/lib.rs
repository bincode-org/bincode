#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate "rustc-serialize" as rustc_serialize;

use std::io::{Buffer, MemWriter, MemReader};
use rustc_serialize::{Encodable, Decodable};

pub use writer::{EncoderWriter, EncodingResult, EncodingError};
pub use reader::{DecoderReader, DecodingResult, DecodingError};
use writer::SizeChecker;

mod writer;
mod reader;

#[derive(Clone, Copy)]
pub enum SizeLimit {
    Infinite,
    UpperBound(u64)
}

pub fn encode<T: Encodable>(t: &T, size_limit: SizeLimit) -> EncodingResult<Vec<u8>> {
    let mut w = MemWriter::new();
    match encode_into(t, &mut w, size_limit) {
        Ok(()) => Ok(w.into_inner()),
        Err(e) => Err(e)
    }
}

pub fn decode<T: Decodable>(b: Vec<u8>, size_limit: SizeLimit) -> DecodingResult<T> {
    decode_from(&mut MemReader::new(b), size_limit)
}

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

pub fn decode_from<R: Reader+Buffer, T: Decodable>(r: &mut R, size_limit: SizeLimit) -> DecodingResult<T> {
    Decodable::decode(&mut reader::DecoderReader::new(r, size_limit))
}

#[cfg(test)]
mod test;
