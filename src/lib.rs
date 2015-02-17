#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![feature(io, hash, core)]

#![doc(html_logo_url = "./icon.png")]

extern crate "rustc-serialize" as rustc_serialize;

use std::old_io::{Buffer, MemWriter};
use rustc_serialize::{Encodable, Decodable};

pub use refbox::RefBox;
pub use writer::{EncoderWriter, EncodingResult, EncodingError};
pub use reader::{DecoderReader, DecodingResult, DecodingError};
use writer::SizeChecker;

mod writer;
mod reader;
mod refbox;
#[cfg(test)] mod test;

///! `bincode` is a crate for encoding and decoding using a tiny binary
///! serialization strategy.
///!
///! There are simple functions for encoding to `Vec<u8>` and decoding from
///! `&[u8]`, but the meat of the library is the `encode_into` and `decode_from`
///! functions which respectively allow encoding into a `std::io::Writer`
///! and decoding from a `std::io::Buffer`.
///!
///! ### Using Basic Functions
///!
///! ```rust
///! #![allow(unstable)]
///! extern crate bincode;
///! fn main() {
///!     // The object that we will serialize.
///!     let target = Some("hello world".to_string());
///!     // The maximum size of the encoded message.
///!     let limit = bincode::SizeLimit::Bounded(20);
///!
///!     let encoded: Vec<u8>        = bincode::encode(&target, limit).unwrap();
///!     let decoded: Option<String> = bincode::decode(&encoded[]).unwrap();
///!     assert_eq!(target, decoded);
///! }
///! ```
///!
///! ### Using Into/From Functions
///!
///! ```rust
///! #![allow(unstable)]
///! extern crate bincode;
///! use std::old_io::pipe::PipeStream;
///! use std::old_io::BufferedReader;
///! fn main() {
///!     // The pipes that we will be using to send values across.
///!     let streams = PipeStream::pair().unwrap();
///!     let (mut reader, mut writer) = (BufferedReader::new(streams.reader),
///!                                     streams.writer);
///!     // The object that we will send across.
///!     let target = Some(5u32);
///!     // The max-size of the encoded bytes.
///!     let limit = bincode::SizeLimit::Bounded(10);
///!
///!     // Do the actual encoding and decoding.
///!     bincode::encode_into(&target, &mut writer, limit).ok();
///!     let out: Option<u32> = bincode::decode_from(&mut reader, limit).unwrap();
///!     assert_eq!(target, out);
///! }
///! ```
///!

/// A limit on the size of bytes to be read or written.
///
/// Size limits are an incredibly important part of both encoding and decoding.
///
/// In order to prevent DOS attacks on a decoder, it is important to limit the
/// amount of bytes that a single encoded message can be; otherwise, if you
/// are decoding bytes right off of a TCP stream for example, it would be
/// possible for an attacker to flood your server with a 3TB vec, causing the
/// decoder to run out of memory and crash your application!
/// Because of this, you can provide a maximum-number-of-bytes that can be read
/// during decoding, and the decoder will explicitly fail if it has to read
/// any more than that.
///
/// On the other side, you want to make sure that you aren't encoding a message
/// that is larger than your decoder expects.  By supplying a size limit to an
/// encoding function, the encoder will verify that the structure can be encoded
/// within that limit.  This verification occurs before any bytes are written to
/// the Writer, so recovering from an the error is possible.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum SizeLimit {
    Infinite,
    Bounded(u64)
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
///
/// This method does not have a size-limit because if you already have the bytes
/// in memory, then you don't gain anything by having a limiter.
pub fn decode<T: Decodable>(b: &[u8]) -> DecodingResult<T> {
    let mut b = b;
    decode_from(&mut b, SizeLimit::Infinite)
}

/// Encodes an object directly into a `Writer`.
///
/// If the encoding would take more bytes than allowed by `size_limit`, an error
/// is returned and *no bytes* will be written into the `Writer`.
///
/// If this returns an `EncodingError` (other than SizeLimit), assume that the
/// writer is in an invalid state, as writing could bail out in the middle of
/// encoding.
pub fn encode_into<T: Encodable, W: Writer>(t: &T, w: &mut W, size_limit: SizeLimit) -> EncodingResult<()> {
    try!(match size_limit {
        SizeLimit::Infinite => Ok(()),
        SizeLimit::Bounded(x) => {
            let mut size_checker = SizeChecker::new(x);
            t.encode(&mut size_checker)
        }
    });

    t.encode(&mut writer::EncoderWriter::new(w, size_limit))
}

/// Decoes an object directly from a `Buffer`ed Reader.
///
/// If the provided `SizeLimit` is reached, the decode will bail immediately.
/// A SizeLimit can help prevent an attacker from flooding your server with
/// a neverending stream of values that runs your server out of memory.
///
/// If this returns an `DecodingError`, assume that the buffer that you passed
/// in is in an invalid state, as the error could be returned during any point
/// in the reading.
pub fn decode_from<R: Buffer, T: Decodable>(r: &mut R, size_limit: SizeLimit) ->
DecodingResult<T> {
    Decodable::decode(&mut reader::DecoderReader::new(r, size_limit))
}


/// Returns the size that an object would be if encoded using bincode.
///
/// This is used internally as part of the check for encode_into, but it can
/// be useful for preallocating buffers if thats your style.
pub fn encoded_size<T: Encodable>(t: &T) -> u64 {
    use std::u64::MAX;
    let mut size_checker = SizeChecker::new(MAX);
    t.encode(&mut size_checker).ok();
    size_checker.written
}
