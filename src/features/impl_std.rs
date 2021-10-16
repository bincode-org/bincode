use crate::{
    config::{self, Config},
    de::{read::Reader, BorrowDecodable, BorrowDecode, Decodable, Decode, Decoder},
    enc::{write::Writer, Encode, Encodeable, Encoder},
    error::{DecodeError, EncodeError},
};
use std::{
    ffi::{CStr, CString},
    sync::{Mutex, RwLock},
};

/// Decode type `D` from the given reader. The reader can be any type that implements `std::io::Read`, e.g. `std::fs::File`.
pub fn decode_from<D: Decodable, R: std::io::Read>(src: &mut R) -> Result<D, DecodeError> {
    decode_from_with_config(src, config::Default)
}

/// Decode type `D` from the given reader with the given `Config`. The reader can be any type that implements `std::io::Read`, e.g. `std::fs::File`.
///
/// See the [config] module for more information about config options.
pub fn decode_from_with_config<D: Decodable, C: Config, R: std::io::Read>(
    src: &mut R,
    _config: C,
) -> Result<D, DecodeError> {
    let mut decoder = Decoder::<_, C>::new(src, _config);
    D::decode(&mut decoder)
}

impl<'storage, R: std::io::Read> Reader<'storage> for R {
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        match self.read_exact(bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(DecodeError::UnexpectedEnd),
        }
    }
}

/// Encode the given value into any type that implements `std::io::Write`, e.g. `std::fs::File`.
pub fn encode_into_write<E: Encodeable, W: std::io::Write>(
    val: E,
    dst: &mut W,
) -> Result<usize, EncodeError> {
    encode_into_write_with_config(val, dst, config::Default)
}

/// Encode the given value into any type that implements `std::io::Write`, e.g. `std::fs::File`, with the given `Config`. See the [config] module for more information.
pub fn encode_into_write_with_config<E: Encodeable, C: Config, W: std::io::Write>(
    val: E,
    dst: &mut W,
    config: C,
) -> Result<usize, EncodeError> {
    let writer = IoWriter {
        writer: dst,
        bytes_written: 0,
    };
    let mut encoder = Encoder::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written)
}

struct IoWriter<'a, W: std::io::Write> {
    writer: &'a mut W,
    bytes_written: usize,
}

impl<'storage, W: std::io::Write> Writer for IoWriter<'storage, W> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.writer
            .write_all(bytes)
            .map_err(|error| EncodeError::Io {
                error,
                index: self.bytes_written,
            })?;
        self.bytes_written += bytes.len();
        Ok(())
    }
}

impl<'a> Encodeable for &'a CStr {
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        self.to_bytes_with_nul().encode(encoder)
    }
}

impl<'de> BorrowDecodable<'de> for &'de CStr {
    fn borrow_decode<D: BorrowDecode<'de>>(decoder: D) -> Result<Self, DecodeError> {
        let bytes = <&[u8]>::borrow_decode(decoder)?;
        CStr::from_bytes_with_nul(bytes).map_err(|e| DecodeError::CStrNulError { inner: e })
    }
}

impl Encodeable for CString {
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        self.as_bytes_with_nul().encode(encoder)
    }
}

impl Decodable for CString {
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        // BlockedTODO: https://github.com/rust-lang/rust/issues/73179
        // use `from_vec_with_nul` instead, combined with:
        // let bytes = std::vec::Vec::<u8>::decode(decoder)?;

        // now we have to allocate twice unfortunately
        let vec: std::vec::Vec<u8> = std::vec::Vec::decode(decoder)?;
        let cstr =
            CStr::from_bytes_with_nul(&vec).map_err(|e| DecodeError::CStrNulError { inner: e })?;
        Ok(cstr.into())
    }
}

impl<T> Encodeable for Mutex<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        let t = self.lock().map_err(|_| EncodeError::LockFailed {
            type_name: core::any::type_name::<Mutex<T>>(),
        })?;
        t.encode(encoder)
    }
}

impl<T> Decodable for Mutex<T>
where
    T: Decodable,
{
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Mutex::new(t))
    }
}

impl<T> Encodeable for RwLock<T>
where
    T: Encodeable,
{
    fn encode<E: Encode>(&self, encoder: E) -> Result<(), EncodeError> {
        let t = self.read().map_err(|_| EncodeError::LockFailed {
            type_name: core::any::type_name::<Mutex<T>>(),
        })?;
        t.encode(encoder)
    }
}

impl<T> Decodable for RwLock<T>
where
    T: Decodable,
{
    fn decode<D: Decode>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(RwLock::new(t))
    }
}
