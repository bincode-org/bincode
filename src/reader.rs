use std::io::Read;
use std::io::Error as IoError;
use std::io::Result as IoResult;
use std::num::{cast, NumCast};
use std::error::{Error, FromError};
use std::fmt;

use rustc_serialize::Decoder;

use byteorder::{BigEndian, ReadBytesExt};
use byteorder::Error as ByteOrderError;

use unicode;

use super::SizeLimit;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct InvalidEncoding {
    desc: &'static str,
    detail: Option<String>,
}

impl fmt::Display for InvalidEncoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidEncoding { detail: None, desc } =>
                write!(fmt, "{}", desc),
            InvalidEncoding { detail: Some(ref detail), desc } =>
                write!(fmt, "{} ({})", desc, detail)
        }
    }
}

/// An error that can be produced during decoding.
///
/// If decoding from a Buffer, assume that the buffer has been left
/// in an invalid state.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DecodingError {
    /// If the error stems from the reader that is being used
    /// during decoding, that error will be stored and returned here.
    IoError(IoError),
    /// If the bytes in the reader are not decodable because of an invalid
    /// encoding, this error will be returned.  This error is only possible
    /// if a stream is corrupted.  A stream produced from `encode` or `encode_into`
    /// should **never** produce an InvalidEncoding error.
    InvalidEncoding(InvalidEncoding),
    /// If decoding a message takes more than the provided size limit, this
    /// error is returned.
    SizeLimit
}

impl fmt::Display for DecodingError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodingError::IoError(ref ioerr) =>
                write!(fmt, "IoError: {}", ioerr),
            DecodingError::InvalidEncoding(ref ib) =>
                write!(fmt, "InvalidEncoding: {}", ib),
            DecodingError::SizeLimit =>
                write!(fmt, "SizeLimit")
        }
    }
}

pub type DecodingResult<T> = Result<T, DecodingError>;

fn wrap_io(err: ByteOrderError) -> DecodingError {
    match err {
        ByteOrderError::Io(ioe) => DecodingError::IoError(ioe),
        ByteOrderError::UnexpectedEOF =>
            DecodingError::InvalidEncoding(InvalidEncoding {
                desc: "Unexpected EOF while reading a multi-byte number",
                detail: None
            })
    }
}

impl Error for DecodingError {
    fn description(&self) -> &str {
        match *self {
            DecodingError::IoError(ref err)     => err.description(),
            DecodingError::InvalidEncoding(ref ib) => ib.desc,
            DecodingError::SizeLimit => "the size limit for decoding has been reached"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DecodingError::IoError(ref err)     => err.cause(),
            DecodingError::InvalidEncoding(_) => None,
            DecodingError::SizeLimit => None
        }
    }
}

impl FromError<IoError> for DecodingError {
    fn from_error(err: IoError) -> DecodingError {
        DecodingError::IoError(err)
    }
}

/// A Decoder that reads bytes from a buffer.
///
/// This struct should rarely be used.
/// In most cases, prefer the `decode_from` function.
pub struct DecoderReader<'a, R: 'a> {
    reader: &'a mut R,
    size_limit: SizeLimit,
    read: u64
}

impl<'a, R: Read> DecoderReader<'a, R> {
    pub fn new(r: &'a mut R, size_limit: SizeLimit) -> DecoderReader<'a, R> {
        DecoderReader {
            reader: r,
            size_limit: size_limit,
            read: 0
        }
    }
}

impl <'a, A> DecoderReader<'a, A> {
    fn read_bytes<I>(&mut self, count: I) -> Result<(), DecodingError>
    where I: NumCast {
        self.read += cast(count).unwrap();
        match self.size_limit {
            SizeLimit::Infinite => Ok(()),
            SizeLimit::Bounded(x) if self.read <= x => Ok(()),
            SizeLimit::Bounded(_) => Err(DecodingError::SizeLimit)
        }
    }

    fn read_type<T>(&mut self) -> Result<(), DecodingError> {
        use std::mem::size_of;
        self.read_bytes(size_of::<T>())
    }
}

impl<'a, R: Read> Decoder for DecoderReader<'a, R> {
    type Error = DecodingError;

    fn read_nil(&mut self) -> DecodingResult<()> {
        Ok(())
    }
    fn read_usize(&mut self) -> DecodingResult<usize> {
        Ok(try!(self.read_u64().map(|x| x as usize)))
    }
    fn read_u64(&mut self) -> DecodingResult<u64> {
        try!(self.read_type::<u64>());
        self.reader.read_u64::<BigEndian>().map_err(wrap_io)
    }
    fn read_u32(&mut self) -> DecodingResult<u32> {
        try!(self.read_type::<u32>());
        self.reader.read_u32::<BigEndian>().map_err(wrap_io)
    }
    fn read_u16(&mut self) -> DecodingResult<u16> {
        try!(self.read_type::<u16>());
        self.reader.read_u16::<BigEndian>().map_err(wrap_io)
    }
    fn read_u8(&mut self) -> DecodingResult<u8> {
        try!(self.read_type::<u8>());
        self.reader.read_u8().map_err(wrap_io)
    }
    fn read_isize(&mut self) -> DecodingResult<isize> {
        self.read_i64().map(|x| x as isize)
    }
    fn read_i64(&mut self) -> DecodingResult<i64> {
        try!(self.read_type::<i64>());
        self.reader.read_i64::<BigEndian>().map_err(wrap_io)
    }
    fn read_i32(&mut self) -> DecodingResult<i32> {
        try!(self.read_type::<i32>());
        self.reader.read_i32::<BigEndian>().map_err(wrap_io)
    }
    fn read_i16(&mut self) -> DecodingResult<i16> {
        try!(self.read_type::<i16>());
        self.reader.read_i16::<BigEndian>().map_err(wrap_io)
    }
    fn read_i8(&mut self) -> DecodingResult<i8> {
        try!(self.read_type::<i8>());
        self.reader.read_i8().map_err(wrap_io)
    }
    fn read_bool(&mut self) -> DecodingResult<bool> {
        let x = try!(self.read_i8());
        match x {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(DecodingError::InvalidEncoding(InvalidEncoding{
                desc: "invalid u8 when decoding bool",
                detail: Some(format!("Expected 0 or 1, got {}", x))
            })),
        }
    }
    fn read_f64(&mut self) -> DecodingResult<f64> {
        try!(self.read_type::<f64>());
        self.reader.read_f64::<BigEndian>().map_err(wrap_io)
    }
    fn read_f32(&mut self) -> DecodingResult<f32> {
        try!(self.read_type::<f32>());
        self.reader.read_f32::<BigEndian>().map_err(wrap_io)
    }
    fn read_char(&mut self) -> DecodingResult<char> {
        use std::str;

        let error = DecodingError::InvalidEncoding(InvalidEncoding {
            desc: "Invalid char encoding",
            detail: None
        });

        let mut buf = [0];

        let _ = try!(self.reader.read(&mut buf[..]));
        let first_byte = buf[0];
        let width = unicode::str::utf8_char_width(first_byte);
        if width == 1 { return Ok(first_byte as char) }
        if width == 0 { return Err(error)}
        let mut buf = [first_byte, 0, 0, 0];
        {
            let mut start = 1;
            while start < width {
                match try!(self.reader.read(&mut buf[start .. width])) {
                    n if n == width - start => break,
                    n if n < width - start => { start += n; }
                    _ => return Err(error)
                }
            }
        }

        let res = try!(match str::from_utf8(&buf[..width]).ok() {
            Some(s) => Ok(s.char_at(0)),
            None => Err(error)
        });

        try!(self.read_bytes(res.len_utf8()));
        Ok(res)
    }

    fn read_str(&mut self) -> DecodingResult<String> {
        let len = try!(self.read_usize());
        try!(self.read_bytes(len));

        let vector = try!(read_exact(&mut self.reader, len));
        match String::from_utf8(vector) {
            Ok(s) => Ok(s),
            Err(err) => Err(DecodingError::InvalidEncoding(InvalidEncoding {
                desc: "error while decoding utf8 string",
                detail: Some(format!("Decoding error: {}", err))
            })),
        }
    }
    fn read_enum<T, F>(&mut self, _: &str, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_enum_variant<T, F>(&mut self, names: &[&str], mut f: F) -> DecodingResult<T> where
        F: FnMut(&mut DecoderReader<'a, R>, usize) -> DecodingResult<T> {
            let id = try!(self.read_u32());
            let id = id as usize;
            if id >= names.len() {
                Err(DecodingError::InvalidEncoding(InvalidEncoding {
                    desc: "out of bounds tag when reading enum variant",
                    detail: Some(format!("Expected tag < {}, got {}", names.len(), id))
                }))
            } else {
                f(self, id)
            }
        }
    fn read_enum_variant_arg<T, F>(&mut self, _: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> DecodingResult<T> where
        F: FnMut(&mut DecoderReader<'a, R>, usize) -> DecodingResult<T> {
            self.read_enum_variant(names, f)
        }
    fn read_enum_struct_variant_field<T, F>(&mut self,
                                            _: &str,
                                            f_idx: usize,
                                            f: F)
        -> DecodingResult<T> where
            F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
                self.read_enum_variant_arg(f_idx, f)
            }
    fn read_struct<T, F>(&mut self, _: &str, _: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_struct_field<T, F>(&mut self,
                               _: &str,
                               _: usize,
                               f: F)
        -> DecodingResult<T> where
            F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
                f(self)
            }
    fn read_tuple<T, F>(&mut self, _: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_tuple_arg<T, F>(&mut self, _: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_tuple_struct<T, F>(&mut self, _: &str, len: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            self.read_tuple(len, f)
        }
    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            self.read_tuple_arg(a_idx, f)
        }
    fn read_option<T, F>(&mut self, mut f: F) -> DecodingResult<T> where
        F: FnMut(&mut DecoderReader<'a, R>, bool) -> DecodingResult<T> {
            let x = try!(self.read_u8());
            match x {
                1 => f(self, true),
                0 => f(self, false),
                _ => Err(DecodingError::InvalidEncoding(InvalidEncoding {
                    desc: "invalid tag when decoding Option",
                    detail: Some(format!("Expected 0 or 1, got {}", x))
                })),
            }
        }
    fn read_seq<T, F>(&mut self, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>, usize) -> DecodingResult<T> {
            let len = try!(self.read_usize());
            f(self, len)
        }
    fn read_seq_elt<T, F>(&mut self, _: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_map<T, F>(&mut self, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>, usize) -> DecodingResult<T> {
            let len = try!(self.read_usize());
            f(self, len)
        }
    fn read_map_elt_key<T, F>(&mut self, _: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_map_elt_val<T, F>(&mut self, _: usize, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn error(&mut self, err: &str) -> DecodingError {
        DecodingError::InvalidEncoding(InvalidEncoding {
            desc: "user-induced error",
            detail: Some(err.to_string()),
        })
    }
}

fn read_at_least<R: Read>(reader: &mut R, min: usize, buf: &mut [u8]) -> IoResult<usize> {
    use std::io::ErrorKind;
    if min > buf.len() {
        return Err(IoError::new(
            ErrorKind::InvalidInput, "the buffer is too short", None));
    }

    let mut read = 0;
    while read < min {
        let mut zeroes = 0;
        loop {
            match reader.read(&mut buf[read..]) {
                Ok(0) => {
                    zeroes += 1;
                    if zeroes >= 1000 {
                        return Err(IoError::new(ErrorKind::Other,
                                                "no progress was made",
                                                None ));
                    }
                }
                Ok(n) => {
                    read += n;
                    break;
                }
                err@Err(_) => return err
            }
        }
    }
    Ok(read)
}

unsafe fn slice_vec_capacity<'a, T>(v: &'a mut Vec<T>, start: usize, end: usize) -> &'a mut [T] {
    use std::raw::Slice;
    use std::ptr::PtrExt;
    use std::mem::transmute;

    assert!(start <= end);
    assert!(end <= v.capacity());
    transmute(Slice {
        data: v.as_ptr().offset(start as isize),
        len: end - start
    })
}


fn push_at_least<R: Read>(reader: &mut R, min: usize, len: usize, buf: &mut Vec<u8>) -> IoResult<usize> {
    use std::io::ErrorKind;
    if min > len {
        return Err(IoError::new(ErrorKind::InvalidInput, "the buffer is too short", None));
    }

    let start_len = buf.len();
    buf.reserve(len);


    let mut read = 0;
    while read < min {
        read += {
            let s = unsafe { slice_vec_capacity(buf, start_len + read, start_len + len) };
            try!(read_at_least(reader, 1, s))
        };
        unsafe { buf.set_len(start_len + read) };
    }
    Ok(read)
}

fn read_exact<R: Read>(reader: &mut R, len: usize) -> IoResult<Vec<u8>> {
    let mut buf = Vec::with_capacity(len);
    match push_at_least(reader, len, len, &mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    }
}
