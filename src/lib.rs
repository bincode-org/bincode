#![feature(struct_variant)]

extern crate serialize;

use std::io::MemWriter;
use std::io::MemReader;
use std::io::IoError;
use serialize::Encodable;
use serialize::Decodable;

pub use writer::EncoderWriter;
pub use reader::DecoderReader;

mod writer;
mod reader;

pub fn encode<T: Encodable<EncoderWriter<MemWriter>, IoError>>(t: &T) ->
Result<Vec<u8>, IoError> {
    match encode_into(t, MemWriter::new()) {
        Ok(w) => Ok(w.unwrap()),
        Err((_, e)) => Err(e)
    }
}

pub fn decode<T: Decodable<DecoderReader<MemReader>, IoError>>(b: Vec<u8>) ->
Result<T, (IoError, Vec<u8>)> {
    match decode_from(MemReader::new(b)) {
        Ok((t, _)) => Ok(t),
        Err((e, r)) => Err((e, r.unwrap()))
    }
}

pub fn encode_into<W: Writer, T: Encodable<EncoderWriter<W>, IoError>>
(t: &T, w: W)
-> Result<W, (W, IoError)> {
    let mut writer = writer::EncoderWriter::new(w);
    match t.encode(&mut writer) {
        Ok(()) => Ok(writer.unwrap()),
        Err(e) => Err((writer.unwrap(), e))
    }
}

pub fn decode_from<R: Reader, T: Decodable<DecoderReader<R>, IoError>>(r: R) ->
Result<(T, R), (IoError, R)> {
    let mut reader = reader::DecoderReader::new(r);
    let x: Result<T, IoError> = Decodable::decode(&mut reader);
    let mem = reader.unwrap();

    match x {
        Ok(t) => Ok((t, mem)),
        Err(e) => Err((e, mem))
    }
}

#[cfg(test)]
mod test;
