use std::io::Write;

use super::{BincodeRead, IntEncoding, Options};
use crate::error::{ErrorKind, Result};
use serde::Serializer;

pub trait StrEncoding {
    fn serialize_str<W: Write, O: Options>(
        ser: &mut crate::ser::Serializer<W, O>,
        v: &str,
    ) -> Result<()>;

    fn get_len<O: Options>(s: &str) -> u64;

    fn deserialize_str<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut crate::de::Deserializer<R, O>,
    ) -> Result<String>;
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

    fn deserialize_str<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut crate::Deserializer<R, O>,
    ) -> Result<String> {
        let vec = de.read_vec()?;
        String::from_utf8(vec).map_err(|e| ErrorKind::InvalidUtf8Encoding(e.utf8_error()).into())
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

    fn deserialize_str<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut crate::Deserializer<R, O>,
    ) -> Result<String> {
        let vec = std::iter::repeat(0)
            .map(|_| de.deserialize_byte())
            .take_while(|r| match r {
                Ok(r) => *r != 0x0,
                Err(_) => false,
            })
            .collect::<Result<Vec<_>>>()?;
        String::from_utf8(vec).map_err(|e| ErrorKind::InvalidUtf8Encoding(e.utf8_error()).into())
    }
}
