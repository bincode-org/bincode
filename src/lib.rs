#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(old_orphan_check)]
#![feature(associated_types)]

extern crate "rustc-serialize" as rustc_serialize;

use std::io::Buffer;
use std::io::MemWriter;
use std::io::MemReader;
use std::io::IoResult;
use rustc_serialize::Encodable;
use rustc_serialize::Decodable;

pub use writer::EncoderWriter;
pub use reader::DecoderReader;

mod writer;
mod reader;

#[derive(Clone, Copy)]
pub enum SizeLimit {
    Infinite,
    UpperBound(u64)
}

pub fn encode<T: Encodable>(t: &T, size_limit: SizeLimit) -> IoResult<Vec<u8>> {
    let mut w = MemWriter::new();
    match encode_into(t, &mut w, size_limit) {
        Ok(()) => Ok(w.into_inner()),
        Err(e) => Err(e)
    }
}

pub fn decode<T: Decodable>(b: Vec<u8>, size_limit: SizeLimit) -> IoResult<T> {
    decode_from(&mut MemReader::new(b), size_limit)
}

pub fn encode_into<T: Encodable, W: Writer>(t: &T, w: &mut W, size_limit: SizeLimit) -> IoResult<()> {
    t.encode(&mut writer::EncoderWriter::new(w, size_limit))
}

pub fn decode_from<R: Reader+Buffer, T: Decodable>(r: &mut R, size_limit: SizeLimit) -> IoResult<T> {
    Decodable::decode(&mut reader::DecoderReader::new(r, size_limit))
}

#[cfg(test)]
mod test;
