#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(old_orphan_check)]

extern crate "rustc-serialize" as rustc_serialize;

use std::io::Buffer;
use std::io::MemWriter;
use std::io::MemReader;
use std::io::IoError;
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

pub fn encode<'a, T>(t: &T, size_limit: SizeLimit) -> IoResult<Vec<u8>>
where T: Encodable<EncoderWriter<'a, MemWriter>, IoError> {
    let mut w = MemWriter::new();
    match encode_into(t, &mut w, size_limit) {
        Ok(()) => Ok(w.into_inner()),
        Err(e) => Err(e)
    }
}

pub fn decode<'a, T>(b: Vec<u8>, size_limit: SizeLimit) -> IoResult<T>
where T: Decodable<DecoderReader<'a, MemReader>, IoError> {
    decode_from(&mut MemReader::new(b), size_limit)
}

// In order to be able to pass MemReaders/MemWriters by reference, I borrowed the method used in
// the current json encoder in the stdlib

// TODO: Make code safe https://github.com/rust-lang/rust/issues/14302
pub fn encode_into<'a, W, T>(t: &T, w: &mut W, size_limit: SizeLimit) -> IoResult<()>
where W: 'a + Writer, T: Encodable<EncoderWriter<'a, W>, IoError>{
    unsafe {
        t.encode(std::mem::transmute(&mut writer::EncoderWriter::new(w, size_limit)))
    }
}

// TODO: Make code safe https://github.com/rust-lang/rust/issues/14302
pub fn decode_from<'a, R, T>(r: &mut R, size_limit: SizeLimit) -> IoResult<T>
where R: 'a + Reader + Buffer, T: Decodable<DecoderReader<'a, R>, IoError>{
    unsafe {
        Decodable::decode(std::mem::transmute(&mut reader::DecoderReader::new(r, size_limit)))
    }
}

#[cfg(test)]
mod test;
