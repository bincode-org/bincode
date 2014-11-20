#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate serialize;

use std::io::Buffer;
use std::io::MemWriter;
use std::io::MemReader;
use std::io::IoError;
use std::io::IoResult;
use serialize::Encodable;
use serialize::Decodable;

pub use writer::EncoderWriter;
pub use reader::DecoderReader;

mod writer;
mod reader;

pub fn encode<'a, T: Encodable<EncoderWriter<'a, MemWriter>, IoError>>(t: &T) ->IoResult<Vec<u8>> {
    let mut w = MemWriter::new();
    match encode_into(t, &mut w) {
        Ok(()) => Ok(w.unwrap()),
        Err(e) => Err(e)
    }
}

pub fn decode<'a, T: Decodable<DecoderReader<'a, MemReader>, IoError>>(b: Vec<u8>) -> IoResult<T> {
    decode_from(&mut MemReader::new(b))
}

// In order to be able to pass MemReaders/MemWriters by reference, I borrowed the method used in
// the current json encoder in the stdlib

// TODO: Make code safe https://github.com/rust-lang/rust/issues/14302
pub fn encode_into<'a, W: 'a+Writer, T: Encodable<EncoderWriter<'a, W>, IoError>>(t: &T, w: &mut W) -> IoResult<()> {
    unsafe {
        t.encode(std::mem::transmute(&mut writer::EncoderWriter::new(w)))
    }
}

// TODO: Make code safe https://github.com/rust-lang/rust/issues/14302
pub fn decode_from<'a, R: 'a+Reader+Buffer, T: Decodable<DecoderReader<'a, R>, IoError>>(r: &mut R) -> IoResult<T> {
    unsafe {
        Decodable::decode(std::mem::transmute(&mut reader::DecoderReader::new(r)))
    }
}

#[cfg(test)]
mod test;
