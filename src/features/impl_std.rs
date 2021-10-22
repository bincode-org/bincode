use crate::{
    config::Config,
    de::{read::Reader, BorrowDecode, BorrowDecoder, Decode, Decoder, DecoderImpl},
    enc::{write::Writer, Encode, Encoder, EncoderImpl},
    error::{DecodeError, EncodeError},
};
use core::time::Duration;
use std::{
    ffi::{CStr, CString},
    io::Read,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    sync::{Mutex, RwLock},
    time::SystemTime,
};

/// Decode type `D` from the given reader with the given `Config`. The reader can be any type that implements `std::io::Read`, e.g. `std::fs::File`.
///
/// See the [config] module for more information about config options.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn decode_from_std_read<D: Decode, C: Config, R: std::io::Read>(
    src: &mut R,
    _config: C,
) -> Result<D, DecodeError> {
    let reader = IoReader { reader: src };
    let mut decoder = DecoderImpl::<_, C>::new(reader, _config);
    D::decode(&mut decoder)
}

struct IoReader<R> {
    reader: R,
}

impl<R> Reader for IoReader<R>
where
    R: std::io::Read,
{
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        match self.reader.read_exact(bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(DecodeError::UnexpectedEnd),
        }
    }
}

impl<R> Reader for std::io::BufReader<R>
where
    R: std::io::Read,
{
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        match self.read_exact(bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(DecodeError::UnexpectedEnd),
        }
    }

    #[inline]
    fn peek_read(&self, n: usize) -> Option<&[u8]> {
        self.buffer().get(..n)
    }

    #[inline]
    fn consume(&mut self, n: usize) {
        <Self as std::io::BufRead>::consume(self, n);
    }
}

/// Encode the given value into any type that implements `std::io::Write`, e.g. `std::fs::File`, with the given `Config`.
/// See the [config] module for more information.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn encode_into_std_write<E: Encode, C: Config, W: std::io::Write>(
    val: E,
    dst: &mut W,
    config: C,
) -> Result<usize, EncodeError> {
    let writer = IoWriter {
        writer: dst,
        bytes_written: 0,
    };
    let mut encoder = EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written)
}

struct IoWriter<'a, W: std::io::Write> {
    writer: &'a mut W,
    bytes_written: usize,
}

impl<'storage, W: std::io::Write> Writer for IoWriter<'storage, W> {
    #[inline(always)]
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

impl<'a> Encode for &'a CStr {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        self.to_bytes_with_nul().encode(encoder)
    }
}

impl<'de> BorrowDecode<'de> for &'de CStr {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: D) -> Result<Self, DecodeError> {
        let bytes = <&[u8]>::borrow_decode(decoder)?;
        CStr::from_bytes_with_nul(bytes).map_err(|e| DecodeError::CStrNulError { inner: e })
    }
}

impl Encode for CString {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        self.as_bytes_with_nul().encode(encoder)
    }
}

impl Decode for CString {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
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

impl<T> Encode for Mutex<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        let t = self.lock().map_err(|_| EncodeError::LockFailed {
            type_name: core::any::type_name::<Mutex<T>>(),
        })?;
        t.encode(encoder)
    }
}

impl<T> Decode for Mutex<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Mutex::new(t))
    }
}

impl<T> Encode for RwLock<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        let t = self.read().map_err(|_| EncodeError::LockFailed {
            type_name: core::any::type_name::<RwLock<T>>(),
        })?;
        t.encode(encoder)
    }
}

impl<T> Decode for RwLock<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(RwLock::new(t))
    }
}

impl Encode for SystemTime {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        let duration = self.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| {
            EncodeError::InvalidSystemTime {
                inner: e,
                time: *self,
            }
        })?;
        duration.encode(encoder)
    }
}

impl Decode for SystemTime {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let duration = Duration::decode(decoder)?;
        Ok(SystemTime::UNIX_EPOCH + duration)
    }
}

impl Encode for &'_ Path {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        match self.to_str() {
            Some(str) => str.encode(encoder),
            None => Err(EncodeError::InvalidPathCharacters),
        }
    }
}

impl<'de> BorrowDecode<'de> for &'de Path {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: D) -> Result<Self, DecodeError> {
        let str = <&'de str>::borrow_decode(decoder)?;
        Ok(Path::new(str))
    }
}

impl Encode for PathBuf {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        self.as_path().encode(encoder)
    }
}

impl Decode for PathBuf {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        let string = std::string::String::decode(decoder)?;
        Ok(string.into())
    }
}

impl Encode for IpAddr {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        match self {
            IpAddr::V4(v4) => {
                0u32.encode(&mut encoder)?;
                v4.encode(encoder)
            }
            IpAddr::V6(v6) => {
                1u32.encode(&mut encoder)?;
                v6.encode(encoder)
            }
        }
    }
}

impl Decode for IpAddr {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        match u32::decode(&mut decoder)? {
            0 => Ok(IpAddr::V4(Ipv4Addr::decode(decoder)?)),
            1 => Ok(IpAddr::V6(Ipv6Addr::decode(decoder)?)),
            found => Err(DecodeError::UnexpectedVariant {
                min: 0,
                max: 1,
                found,
                type_name: core::any::type_name::<IpAddr>(),
            }),
        }
    }
}

impl Encode for Ipv4Addr {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        self.octets().encode(encoder)
    }
}

impl Decode for Ipv4Addr {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        Ok(Self::from(<[u8; 4]>::decode(decoder)?))
    }
}

impl Encode for Ipv6Addr {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), EncodeError> {
        self.octets().encode(encoder)
    }
}

impl Decode for Ipv6Addr {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
        Ok(Self::from(<[u8; 16]>::decode(decoder)?))
    }
}

impl Encode for SocketAddr {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        match self {
            SocketAddr::V4(v4) => {
                0u32.encode(&mut encoder)?;
                v4.encode(encoder)
            }
            SocketAddr::V6(v6) => {
                1u32.encode(&mut encoder)?;
                v6.encode(encoder)
            }
        }
    }
}

impl Decode for SocketAddr {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        match u32::decode(&mut decoder)? {
            0 => Ok(SocketAddr::V4(SocketAddrV4::decode(decoder)?)),
            1 => Ok(SocketAddr::V6(SocketAddrV6::decode(decoder)?)),
            found => Err(DecodeError::UnexpectedVariant {
                min: 0,
                max: 1,
                found,
                type_name: core::any::type_name::<SocketAddr>(),
            }),
        }
    }
}

impl Encode for SocketAddrV4 {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.ip().encode(&mut encoder)?;
        self.port().encode(encoder)
    }
}

impl Decode for SocketAddrV4 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        let ip = Ipv4Addr::decode(&mut decoder)?;
        let port = u16::decode(decoder)?;
        Ok(Self::new(ip, port))
    }
}

impl Encode for SocketAddrV6 {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), EncodeError> {
        self.ip().encode(&mut encoder)?;
        self.port().encode(encoder)
    }
}

impl Decode for SocketAddrV6 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, DecodeError> {
        let ip = Ipv6Addr::decode(&mut decoder)?;
        let port = u16::decode(decoder)?;
        Ok(Self::new(ip, port, 0, 0))
    }
}
