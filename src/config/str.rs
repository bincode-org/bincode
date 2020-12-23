use std::io::Write;

use crate::ser::SizeChecker;

use super::IntEncoding;
use super::Options;
use error::Result;
use serde::Serializer;

pub trait StrEncoding {
    fn serialize_str<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        v: &str,
    ) -> Result<()>;

    fn get_len<O: Options>(s: &str) -> u64;
}

/// Encode strings the same way as vectors,
/// as the length followed by the data.
pub struct LenStrEncoding;

/// Encode strings c-style, with the contents
/// followed by a null byte.
pub struct NullTerminatedStrEncoding;

impl StrEncoding for LenStrEncoding {
    fn serialize_str<W: Write, O: Options>(
        ser: &mut crate::Serializer<W, O>,
        v: &str,
    ) -> Result<()> {
        ser.serialize_bytes(v.as_bytes()).map_err(Into::into)
    }

    fn get_len<O: Options>(s: &str) -> u64 {
        O::IntEncoding::len_size(s.len()) + s.len() as u64
    }
}

impl StrEncoding for NullTerminatedStrEncoding {
    fn serialize_str<W: Write, O: Options>(
        ser: &mut crate::Serializer<W, O>,
        v: &str,
    ) -> Result<()> {
        ser.serialize_raw(v.as_bytes())
            .and_then(|_| ser.serialize_byte(0x0))
            .map_err(Into::into)
    }

    fn get_len<O: Options>(s: &str) -> u64 {
        s.len() as u64 + "\0".len() as u64
    }
}
