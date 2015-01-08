use std::io::{Buffer, Reader, IoError, IoResult, OtherIoError};
use std::num::{cast, NumCast};
use std::error::Error;

use rustc_serialize::Decoder;

use super::SizeLimit;

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
    fn read_bytes<I>(&mut self, count: I) -> Result<(), IoError>
    where I: NumCast {
        self.read += cast(count).unwrap();
        match self.size_limit {
            SizeLimit::Infinite => Ok(()),
            SizeLimit::UpperBound(x) if self.read <= x => Ok(()),
            SizeLimit::UpperBound(_) => Err(IoError{
                kind: OtherIoError,
                desc: "The max number of bytes has been read from this reader.",
                detail: None
            })
        }
    }
    fn read_type<T>(&mut self) -> Result<(), IoError> {
        use std::intrinsics::size_of;
        unsafe{ self.read_bytes(size_of::<T>()) }
    }
}

impl<'a, R: Reader+Buffer> Decoder for DecoderReader<'a, R> {
    type Error = IoError;

    fn read_nil(&mut self) -> IoResult<()> {
        Ok(())
    }
    fn read_uint(&mut self) -> IoResult<uint> {
        self.read_u64().map(|x| x as uint)
    }
    fn read_u64(&mut self) -> IoResult<u64> {
        try!(self.read_type::<u64>());
        self.reader.read_be_u64()
    }
    fn read_u32(&mut self) -> IoResult<u32> {
        try!(self.read_type::<u32>());
        self.reader.read_be_u32()
    }
    fn read_u16(&mut self) -> IoResult<u16> {
        try!(self.read_type::<u16>());
        self.reader.read_be_u16()
    }
    fn read_u8(&mut self) -> IoResult<u8> {
        try!(self.read_type::<u8>());
        self.reader.read_u8()
    }
    fn read_int(&mut self) -> IoResult<int> {
        self.read_i64().map(|x| x as int)
    }
    fn read_i64(&mut self) -> IoResult<i64> {
        try!(self.read_type::<i64>());
        self.reader.read_be_i64()
    }
    fn read_i32(&mut self) -> IoResult<i32> {
        try!(self.read_type::<i32>());
        self.reader.read_be_i32()
    }
    fn read_i16(&mut self) -> IoResult<i16> {
        try!(self.read_type::<i16>());
        self.reader.read_be_i16()
    }
    fn read_i8(&mut self) -> IoResult<i8> {
        try!(self.read_type::<i8>());
        self.reader.read_i8()
    }
    fn read_bool(&mut self) -> IoResult<bool> {
        match try!(self.read_i8()) {
            1 => Ok(true),
            _ => Ok(false)
        }
    }
    fn read_f64(&mut self) -> IoResult<f64> {
        try!(self.read_type::<f64>());
        self.reader.read_be_f64()
    }
    fn read_f32(&mut self) -> IoResult<f32> {
        try!(self.read_type::<f32>());
        self.reader.read_be_f32()
    }
    fn read_char(&mut self) -> IoResult<char> {
        try!(self.read_type::<char>());
        self.reader.read_char()
    }
    fn read_str(&mut self) -> IoResult<String> {
        let len = try!(self.read_uint());

        try!(self.read_bytes(len));
        let vector = try!(self.reader.read_exact(len));
        String::from_utf8(vector).map_err(|e| IoError {
            kind: OtherIoError,
            desc: "invalid utf-8",
            detail: e.detail()
        })
    }
    fn read_enum<T, F>(&mut self, _: &str, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            f(self)
        }
    fn read_enum_variant<T, F>(&mut self, names: &[&str], mut f: F) -> IoResult<T> where
        F: FnMut(&mut DecoderReader<'a, R>, uint) -> IoResult<T> {
            let id = try!(self.read_u32());
            let id = id as uint;
            if id >= names.len() {
                Err(IoError {
                    kind: OtherIoError,
                    desc: "out of bounds tag when reading enum variant",
                    detail: Some(format!("Expected tag < {}, got {}", names.len(), id))
                })
            } else {
                f(self, id)
            }
        }
    fn read_enum_variant_arg<T, F>(&mut self, _: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            f(self)
        }
    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> IoResult<T> where
        F: FnMut(&mut DecoderReader<'a, R>, uint) -> IoResult<T> {
            self.read_enum_variant(names, f)
        }
    fn read_enum_struct_variant_field<T, F>(&mut self,
                                            _: &str,
                                            f_idx: uint,
                                            f: F)
        -> IoResult<T> where
            F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
                self.read_enum_variant_arg(f_idx, f)
            }
    fn read_struct<T, F>(&mut self, _: &str, _: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            f(self)
        }
    fn read_struct_field<T, F>(&mut self,
                               _: &str,
                               _: uint,
                               f: F)
        -> IoResult<T> where
            F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
                f(self)
            }
    fn read_tuple<T, F>(&mut self, _: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            f(self)
        }
    fn read_tuple_arg<T, F>(&mut self, _: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            f(self)
        }
    fn read_tuple_struct<T, F>(&mut self, _: &str, len: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            self.read_tuple(len, f)
        }
    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            self.read_tuple_arg(a_idx, f)
        }
    fn read_option<T, F>(&mut self, mut f: F) -> IoResult<T> where
        F: FnMut(&mut DecoderReader<'a, R>, bool) -> IoResult<T> {
            match try!(self.read_u8()) {
                1 => f(self, true),
                _ => f(self, false)
            }
        }
    fn read_seq<T, F>(&mut self, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>, uint) -> IoResult<T> {
            let len = try!(self.read_uint());
            f(self, len)
        }
    fn read_seq_elt<T, F>(&mut self, _: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            f(self)
        }
    fn read_map<T, F>(&mut self, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>, uint) -> IoResult<T> {
            let len = try!(self.read_uint());
            f(self, len)
        }
    fn read_map_elt_key<T, F>(&mut self, _: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            f(self)
        }
    fn read_map_elt_val<T, F>(&mut self, _: uint, f: F) -> IoResult<T> where
        F: FnOnce(&mut DecoderReader<'a, R>) -> IoResult<T> {
            f(self)
        }
    fn error(&mut self, err: &str) -> IoError {
        IoError {
            kind: OtherIoError,
            desc: "failure decoding or something, I don't know",
            detail: Some(err.to_string())
        }
    }
}
