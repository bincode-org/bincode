use std::io::{Buffer, Reader, IoError};
use std::num::{cast, NumCast};
use std::error::{Error, FromError};

use rustc_serialize::Decoder;

use super::SizeLimit;

#[derive(PartialEq, Clone, Show)]
pub struct InvalidBytes {
    desc: &'static str,
    detail: Option<String>,
}

#[derive(PartialEq, Clone, Show)]
pub enum DecodingError {
    IoError(IoError),
    InvalidBytes(InvalidBytes),
    SizeLimit
}

pub type DecodingResult<T> = Result<T, DecodingError>;

fn wrap_io(err: IoError) -> DecodingError {
    DecodingError::IoError(err)
}

impl Error for DecodingError {
    fn description(&self) -> &str {
        match *self {
            DecodingError::IoError(ref err)     => err.description(),
            DecodingError::InvalidBytes(ref ib) => ib.desc,
            DecodingError::SizeLimit => "the size limit for decoding has been reached"
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            DecodingError::IoError(ref err)     => err.detail(),
            DecodingError::InvalidBytes(ref ib) => ib.detail.clone(),
            DecodingError::SizeLimit => None
        }
    }
}

impl FromError<IoError> for DecodingError {
    fn from_error(err: IoError) -> DecodingError {
        DecodingError::IoError(err)
    }
}

pub struct DecoderReader<'a, R: 'a> {
    reader: &'a mut R,
    size_limit: SizeLimit,
    read: u64
}

impl<'a, R: Reader+Buffer> DecoderReader<'a, R> {
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
            SizeLimit::UpperBound(x) if self.read <= x => Ok(()),
            SizeLimit::UpperBound(_) => Err(DecodingError::SizeLimit)
        }
    }

    fn read_type<T>(&mut self) -> Result<(), DecodingError> {
        use std::mem::size_of;
        self.read_bytes(size_of::<T>())
    }
}

impl<'a, R: Reader+Buffer> Decoder for DecoderReader<'a, R> {
    type Error = DecodingError;

    fn read_nil(&mut self) -> DecodingResult<()> {
        Ok(())
    }
    fn read_uint(&mut self) -> DecodingResult<uint> {
        Ok(try!(self.read_u64().map(|x| x as uint)))
    }
    fn read_u64(&mut self) -> DecodingResult<u64> {
        try!(self.read_type::<u64>());
        self.reader.read_be_u64().map_err(wrap_io)
    }
    fn read_u32(&mut self) -> DecodingResult<u32> {
        try!(self.read_type::<u32>());
        self.reader.read_be_u32().map_err(wrap_io)
    }
    fn read_u16(&mut self) -> DecodingResult<u16> {
        try!(self.read_type::<u16>());
        self.reader.read_be_u16().map_err(wrap_io)
    }
    fn read_u8(&mut self) -> DecodingResult<u8> {
        try!(self.read_type::<u8>());
        self.reader.read_u8().map_err(wrap_io)
    }
    fn read_int(&mut self) -> DecodingResult<int> {
        self.read_i64().map(|x| x as int)
    }
    fn read_i64(&mut self) -> DecodingResult<i64> {
        try!(self.read_type::<i64>());
        self.reader.read_be_i64().map_err(wrap_io)
    }
    fn read_i32(&mut self) -> DecodingResult<i32> {
        try!(self.read_type::<i32>());
        self.reader.read_be_i32().map_err(wrap_io)
    }
    fn read_i16(&mut self) -> DecodingResult<i16> {
        try!(self.read_type::<i16>());
        self.reader.read_be_i16().map_err(wrap_io)
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
            _ => Err(DecodingError::InvalidBytes(InvalidBytes{
                desc: "invalid u8 when decoding bool",
                detail: Some(format!("Expected 0 or 1, got {}", x))
            })),
        }
    }
    fn read_f64(&mut self) -> DecodingResult<f64> {
        try!(self.read_type::<f64>());
        self.reader.read_be_f64().map_err(wrap_io)
    }
    fn read_f32(&mut self) -> DecodingResult<f32> {
        try!(self.read_type::<f32>());
        self.reader.read_be_f32().map_err(wrap_io)
    }
    fn read_char(&mut self) -> DecodingResult<char> {
        let c = try!(self.reader.read_char().map_err(wrap_io));
        try!(self.read_bytes(c.len_utf8()));
        Ok(c)

    }
    fn read_str(&mut self) -> DecodingResult<String> {
        let len = try!(self.read_uint());

        try!(self.read_bytes(len));
        let vector = try!(self.reader.read_exact(len));
        match String::from_utf8(vector) {
            Ok(s) => Ok(s),
            Err(err) => Err(DecodingError::InvalidBytes(InvalidBytes {
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
        F: FnMut(&mut DecoderReader<'a, R>, uint) -> DecodingResult<T> {
            let id = try!(self.read_u32());
            let id = id as uint;
            if id >= names.len() {
                Err(DecodingError::InvalidBytes(InvalidBytes {
                    desc: "out of bounds tag when reading enum variant",
                    detail: Some(format!("Expected tag < {}, got {}", names.len(), id))
                }))
            } else {
                f(self, id)
            }
        }
    fn read_enum_variant_arg<T, F>(&mut self, _: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> DecodingResult<T> where
        F: FnMut(&mut DecoderReader<'a, R>, uint) -> DecodingResult<T> {
            self.read_enum_variant(names, f)
        }
    fn read_enum_struct_variant_field<T, F>(&mut self,
                                            _: &str,
                                            f_idx: uint,
                                            f: F)
        -> DecodingResult<T> where
            F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
                self.read_enum_variant_arg(f_idx, f)
            }
    fn read_struct<T, F>(&mut self, _: &str, _: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_struct_field<T, F>(&mut self,
                               _: &str,
                               _: uint,
                               f: F)
        -> DecodingResult<T> where
            F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
                f(self)
            }
    fn read_tuple<T, F>(&mut self, _: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_tuple_arg<T, F>(&mut self, _: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_tuple_struct<T, F>(&mut self, _: &str, len: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            self.read_tuple(len, f)
        }
    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            self.read_tuple_arg(a_idx, f)
        }
    fn read_option<T, F>(&mut self, mut f: F) -> DecodingResult<T> where
        F: FnMut(&mut DecoderReader<'a, R>, bool) -> DecodingResult<T> {
            let x = try!(self.read_u8());
            match x {
                1 => f(self, true),
                0 => f(self, false),
                _ => Err(DecodingError::InvalidBytes(InvalidBytes {
                    desc: "invalid tag when decoding Option",
                    detail: Some(format!("Expected 0 or 1, got {}", x))
                })),
            }
        }
    fn read_seq<T, F>(&mut self, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>, uint) -> DecodingResult<T> {
            let len = try!(self.read_uint());
            f(self, len)
        }
    fn read_seq_elt<T, F>(&mut self, _: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_map<T, F>(&mut self, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>, uint) -> DecodingResult<T> {
            let len = try!(self.read_uint());
            f(self, len)
        }
    fn read_map_elt_key<T, F>(&mut self, _: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn read_map_elt_val<T, F>(&mut self, _: uint, f: F) -> DecodingResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> DecodingResult<T> {
            f(self)
        }
    fn error(&mut self, err: &str) -> DecodingError {
        DecodingError::InvalidBytes(InvalidBytes {
            desc: "user-induced error",
            detail: Some(err.to_string()),
        })
    }
}
