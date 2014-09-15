extern crate serialize;

use std::io::Writer;
use std::io::IoError;
use serialize::Encoder;

type EwResult = Result<(), IoError>;

struct EncoderWriter<W> {
    writer: W
}

impl <W> EncoderWriter<W> {
    fn new(w: W) -> EncoderWriter<W> {
        EncoderWriter{ writer: w }
    }

    fn unwrap(self) -> W {
        self.writer
    }
}

impl <W: Writer> Encoder<IoError> for EncoderWriter<W> {
    fn emit_nil(&mut self) -> EwResult { Ok(()) }
    fn emit_uint(&mut self, v: uint) -> EwResult {
        Ok(())
    }
    fn emit_u64(&mut self, v: u64) -> EwResult { Ok(()) }
    fn emit_u32(&mut self, v: u32) -> EwResult { Ok(()) }
    fn emit_u16(&mut self, v: u16) -> EwResult { Ok(()) }
    fn emit_u8(&mut self, v: u8) -> EwResult { Ok(()) }
    fn emit_int(&mut self, v: int) -> EwResult { Ok(()) }
    fn emit_i64(&mut self, v: i64) -> EwResult { Ok(()) }
    fn emit_i32(&mut self, v: i32) -> EwResult { Ok(()) }
    fn emit_i16(&mut self, v: i16) -> EwResult { Ok(()) }
    fn emit_i8(&mut self, v: i8) -> EwResult { Ok(()) }
    fn emit_bool(&mut self, v: bool) -> EwResult { Ok(()) }
    fn emit_f64(&mut self, v: f64) -> EwResult { Ok(()) }
    fn emit_f32(&mut self, v: f32) -> EwResult { Ok(()) }
    fn emit_char(&mut self, v: char) -> EwResult { Ok(()) }
    fn emit_str(&mut self, v: &str) -> EwResult { Ok(()) }
    fn emit_enum(&mut self, name: &str,
                 f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_enum_variant(&mut self, v_name: &str, v_id: uint, len: uint,
                         f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_enum_variant_arg(&mut self, a_idx: uint,
                             f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_enum_struct_variant(&mut self, v_name: &str, v_id: uint,
                                len: uint, f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_enum_struct_variant_field(&mut self, f_name: &str,
                                      f_idx: uint, f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_struct(&mut self, name: &str, len: uint,
                   f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_struct_field(&mut self, f_name: &str, f_idx: uint,
                         f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_tuple(&mut self, len: uint,
                  f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_tuple_arg(&mut self, idx: uint,
                      f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_tuple_struct(&mut self, name: &str, len: uint,
                         f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_tuple_struct_arg(&mut self, f_idx: uint,
                             f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_option(&mut self, f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_option_none(&mut self) -> EwResult { Ok(()) }
    fn emit_option_some(&mut self, f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_seq(&mut self, len: uint, f: |this: &mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_seq_elt(&mut self, idx: uint, f: |this: &mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_map(&mut self, len: uint, f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_map_elt_key(&mut self, idx: uint, f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
    fn emit_map_elt_val(&mut self, idx: uint, f: |&mut EncoderWriter<W>| -> EwResult) -> EwResult { Ok(()) }
}
