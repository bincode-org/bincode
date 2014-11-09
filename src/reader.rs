use std::io::Buffer;
use std::io::Reader;
use std::io::IoError;
use std::io::IoResult;
use std::io::OtherIoError;
use serialize::Decoder;

pub struct DecoderReader<'a, R: 'a> {
    reader: &'a mut R
}

impl<'a, R: Reader+Buffer> DecoderReader<'a, R> {
    pub fn new(r: &'a mut R) -> DecoderReader<'a, R> {
        DecoderReader {
            reader: r
        }
    }
}

impl<'a, R: Reader+Buffer> Decoder<IoError> for DecoderReader<'a, R> {
    fn read_nil(&mut self) -> IoResult<()> {
        Ok(())
    }
    fn read_uint(&mut self) -> IoResult<uint> {
        match self.reader.read_be_u64() {
            Ok(x) => Ok(x as uint),
            Err(e) => Err(e)
        }
    }
    fn read_u64(&mut self) -> IoResult<u64> {
        self.reader.read_be_u64()
    }
    fn read_u32(&mut self) -> IoResult<u32> {
        self.reader.read_be_u32()
    }
    fn read_u16(&mut self) -> IoResult<u16> {
        self.reader.read_be_u16()
    }
    fn read_u8(&mut self) -> IoResult<u8> {
        self.reader.read_u8()
    }
    fn read_int(&mut self) -> IoResult<int> {
        self.reader.read_be_int()
    }
    fn read_i64(&mut self) -> IoResult<i64> {
        self.reader.read_be_i64()
    }
    fn read_i32(&mut self) -> IoResult<i32> {
        self.reader.read_be_i32()
    }
    fn read_i16(&mut self) -> IoResult<i16> {
        self.reader.read_be_i16()
    }
    fn read_i8(&mut self) -> IoResult<i8> {
        self.reader.read_i8()
    }
    fn read_bool(&mut self) -> IoResult<bool> {
        match try!(self.reader.read_i8()) {
            1 => Ok(true),
            _ => Ok(false)
        }
    }
    fn read_f64(&mut self) -> IoResult<f64> {
        self.reader.read_be_f64()
    }
    fn read_f32(&mut self) -> IoResult<f32> {
        self.reader.read_be_f32()
    }
    fn read_char(&mut self) -> IoResult<char> {
        self.reader.read_char()
    }
    fn read_str(&mut self) -> IoResult<String> {
        let len = try!(self.read_uint());
        let mut vector = Vec::with_capacity(len as uint);
        for _ in range(0, len) {
            vector.push(try!(self.reader.read_u8()));
        }
        Ok(String::from_utf8(vector).unwrap())
    }
    fn read_enum<T>(&mut self, _: &str,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        f(self)
    }
    fn read_enum_variant<T>(&mut self, _: &[&str],
    f: |&mut DecoderReader<'a, R>, uint| -> IoResult<T>) -> IoResult<T> {
        let id = try!(self.read_uint());
        f(self, id)
    }
    fn read_enum_variant_arg<T>(&mut self, _: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        f(self)
    }
    fn read_enum_struct_variant<T>(&mut self, names: &[&str],
    f: |&mut DecoderReader<'a, R>, uint| -> IoResult<T>) -> IoResult<T> {
        self.read_enum_variant(names, f)
    }
    fn read_enum_struct_variant_field<T>(&mut self, _: &str, f_idx: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        self.read_enum_variant_arg(f_idx, f)
    }
    fn read_struct<T>(&mut self, _: &str, _: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        f(self)
    }
    fn read_struct_field<T>(&mut self, _: &str, _: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        f(self)
    }
    fn read_tuple<T>(&mut self, _: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) ->
    IoResult<T> {
        f(self)
    }
    fn read_tuple_arg<T>(&mut self, _: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        f(self)
    }
    fn read_tuple_struct<T>(&mut self, _: &str, len: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) ->
    IoResult<T> {
        self.read_tuple(len, f)
    }
    fn read_tuple_struct_arg<T>(&mut self, a_idx: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        self.read_tuple_arg(a_idx, f)
    }
    fn read_option<T>(&mut self,
    f: |&mut DecoderReader<'a, R>, bool| -> IoResult<T>) ->
    IoResult<T> {
        match try!(self.reader.read_u8()) {
            1 => f(self, true),
            _ => f(self, false)
        }
    }
    fn read_seq<T>(&mut self,
    f: |&mut DecoderReader<'a, R>, uint| -> IoResult<T>) ->
    IoResult<T> {
        let len = try!(self.read_uint());
        f(self, len)
    }
    fn read_seq_elt<T>(&mut self, _: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        f(self)
    }
    fn read_map<T>(&mut self,
    f: |&mut DecoderReader<'a, R>, uint| -> IoResult<T>) ->
    IoResult<T> {
        let len = try!(self.read_uint());
        f(self, len)
    }
    fn read_map_elt_key<T>(&mut self, _: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
        f(self)
    }
    fn read_map_elt_val<T>(&mut self, _: uint,
    f: |&mut DecoderReader<'a, R>| -> IoResult<T>) -> IoResult<T> {
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
