use std::io::{Writer, IoError, IoResult};
use std::num::Int;

use rustc_serialize::Encoder;

use super::SizeLimit;

type EwResult = IoResult<()>;

pub struct EncoderWriter<'a, W: 'a> {
    writer: &'a mut W,
    _size_limit: SizeLimit
}

impl <'a, W: Writer> EncoderWriter<'a, W> {
    pub fn new(w: &'a mut W, size_limit: SizeLimit) -> EncoderWriter<'a, W> {
        EncoderWriter {
            writer: w,
            _size_limit: size_limit
        }
    }
}

impl<'a, W: Writer> Encoder for EncoderWriter<'a, W> {
    type Error = IoError;

    fn emit_nil(&mut self) -> EwResult { Ok(()) }
    fn emit_uint(&mut self, v: uint) -> EwResult {
        self.emit_u64(v as u64)
    }
    fn emit_u64(&mut self, v: u64) -> EwResult {
        self.writer.write_be_u64(v)
    }
    fn emit_u32(&mut self, v: u32) -> EwResult {
        self.writer.write_be_u32(v)
    }
    fn emit_u16(&mut self, v: u16) -> EwResult {
        self.writer.write_be_u16(v)
    }
    fn emit_u8(&mut self, v: u8) -> EwResult {
        self.writer.write_u8(v)
    }
    fn emit_int(&mut self, v: int) -> EwResult {
        self.emit_i64(v as i64)
    }
    fn emit_i64(&mut self, v: i64) -> EwResult {
        self.writer.write_be_i64(v)
    }
    fn emit_i32(&mut self, v: i32) -> EwResult {
        self.writer.write_be_i32(v)
    }
    fn emit_i16(&mut self, v: i16) -> EwResult {
        self.writer.write_be_i16(v)
    }
    fn emit_i8(&mut self, v: i8) -> EwResult {
        self.writer.write_i8(v)
    }
    fn emit_bool(&mut self, v: bool) -> EwResult {
        self.writer.write_u8(if v {1} else {0})
    }
    fn emit_f64(&mut self, v: f64) -> EwResult {
        self.writer.write_be_f64(v)
    }
    fn emit_f32(&mut self, v: f32) -> EwResult {
        self.writer.write_be_f32(v)
    }
    fn emit_char(&mut self, v: char) -> EwResult {
        self.writer.write_char(v)
    }
    fn emit_str(&mut self, v: &str) -> EwResult {
        try!(self.emit_uint(v.len()));
        self.writer.write_str(v)
    }
    fn emit_enum<F>(&mut self, __: &str, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_enum_variant<F>(&mut self, _: &str,
                            v_id: uint,
                            _: uint,
                            f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            let max_u32: u32 = Int::max_value();
            if v_id > (max_u32 as uint) {
                panic!("Variant tag doesn't fit in a u32")
            }
            try!(self.emit_u32(v_id as u32));
            f(self)
        }
    fn emit_enum_variant_arg<F>(&mut self, _: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_enum_struct_variant<F>(&mut self, _: &str,
                                   _: uint,
                                   _: uint,
                                   f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_enum_struct_variant_field<F>(&mut self,
                                         _: &str,
                                         _: uint,
                                         f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_struct<F>(&mut self, _: &str, _: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_struct_field<F>(&mut self, _: &str, _: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_tuple<F>(&mut self, _: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_tuple_arg<F>(&mut self, _: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_tuple_struct<F>(&mut self, _: &str, len: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            self.emit_tuple(len, f)
        }
    fn emit_tuple_struct_arg<F>(&mut self, f_idx: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            self.emit_tuple_arg(f_idx, f)
        }
    fn emit_option<F>(&mut self, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_option_none(&mut self) -> EwResult {
        self.writer.write_u8(0)
    }
    fn emit_option_some<F>(&mut self, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            try!(self.writer.write_u8(1));
            f(self)
        }
    fn emit_seq<F>(&mut self, len: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            try!(self.emit_uint(len));
            f(self)
        }
    fn emit_seq_elt<F>(&mut self, _: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_map<F>(&mut self, len: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            try!(self.emit_uint(len));
            f(self)
        }
    fn emit_map_elt_key<F>(&mut self, _: uint, mut f: F) -> EwResult where
        F: FnMut(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
    fn emit_map_elt_val<F>(&mut self, _: uint, f: F) -> EwResult where
        F: FnOnce(&mut EncoderWriter<'a, W>) -> EwResult {
            f(self)
        }
}
