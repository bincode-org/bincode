#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(old_orphan_check)]

extern crate "rustc-serialize" as rustc_serialize;

use std::io::Buffer;
use std::io::MemWriter;
use std::io::MemReader;
use std::io::IoResult;
use rustc_serialize::Encodable;
use rustc_serialize::Decodable;

pub use writer::EncoderWriter;
pub use reader::{DecoderReader, DecodingResult, DecodingError};

mod writer;
mod reader;

pub fn encode<T: Encodable>(t: &T) -> IoResult<Vec<u8>> {
    let mut w = MemWriter::new();
    match encode_into(t, &mut w) {
        Ok(()) => Ok(w.into_inner()),
        Err(e) => Err(e)
    }
}

pub fn decode<T: Decodable>(b: Vec<u8>) -> DecodingResult<T> {
    decode_from(&mut MemReader::new(b))
}

pub fn encode_into<T: Encodable, W: Writer>(t: &T, w: &mut W) -> IoResult<()> {
    t.encode(&mut writer::EncoderWriter::new(w))
}

pub fn decode_from<R: Reader+Buffer, T: Decodable>(r: &mut R) -> DecodingResult<T> {
    Decodable::decode(&mut reader::DecoderReader::new(r))
}

#[cfg(test)]
mod test;
