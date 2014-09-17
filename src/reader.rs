use std::io::Reader;
use std::io::BufferedReader;
use std::io::IoError;
use std::io::OtherIoError;
use serialize::Decoder;

type EwResult = Result<(), IoError>;

pub struct DecoderReader<R> {
    reader: BufferedReader<R>
}

impl <R: Reader> DecoderReader<R> {
    pub fn new(r: R) -> DecoderReader<R> {
        DecoderReader {
            reader: BufferedReader::new(r)
        }
    }
    pub fn unwrap(self) -> R {
        self.reader.unwrap()
    }
}

impl <R: Reader> Decoder<IoError> for DecoderReader<R> {
    fn read_nil(&mut self) -> Result<(), IoError> {
        Ok(())
    }
    fn read_uint(&mut self) -> Result<uint, IoError> {
        self.reader.read_be_uint()
    }
    fn read_u64(&mut self) -> Result<u64, IoError> {
        self.reader.read_be_u64()
    }
    fn read_u32(&mut self) -> Result<u32, IoError> {
        self.reader.read_be_u32()
    }
    fn read_u16(&mut self) -> Result<u16, IoError> {
        self.reader.read_be_u16()
    }
    fn read_u8(&mut self) -> Result<u8, IoError> {
        self.reader.read_u8()
    }
    fn read_int(&mut self) -> Result<int, IoError> {
        self.reader.read_be_int()
    }
    fn read_i64(&mut self) -> Result<i64, IoError> {
        self.reader.read_be_i64()
    }
    fn read_i32(&mut self) -> Result<i32, IoError> {
        self.reader.read_be_i32()
    }
    fn read_i16(&mut self) -> Result<i16, IoError> {
        self.reader.read_be_i16()
    }
    fn read_i8(&mut self) -> Result<i8, IoError> {
        self.reader.read_i8()
    }
    fn read_bool(&mut self) -> Result<bool, IoError> {
        match try!(self.reader.read_i8()) {
            1 => Ok(true),
            _ => Ok(false)
        }
    }
    fn read_f64(&mut self) -> Result<f64, IoError> {
        self.reader.read_be_f64()
    }
    fn read_f32(&mut self) -> Result<f32, IoError> {
        self.reader.read_be_f32()
    }
    fn read_char(&mut self) -> Result<char, IoError> {
        self.reader.read_char()
    }
    fn read_str(&mut self) -> Result<String, IoError> {
        let len = try!(self.reader.read_be_uint());
        let mut string = String::new();
        for _ in range(0, len) {
            string.push_char(try!(self.reader.read_char()));
        }
        Ok(string)
    }
    fn read_enum<T>(&mut self, _: &str,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        f(self)
    }
    fn read_enum_variant<T>(&mut self, _: &[&str],
    f: |&mut DecoderReader<R>, uint| -> Result<T, IoError>) -> Result<T, IoError> {
        let id = try!(self.reader.read_be_uint());
        f(self, id)
    }
    fn read_enum_variant_arg<T>(&mut self, _: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        f(self)
    }
    fn read_enum_struct_variant<T>(&mut self, names: &[&str],
    f: |&mut DecoderReader<R>, uint| -> Result<T, IoError>) -> Result<T, IoError> {
        self.read_enum_variant(names, f)
    }
    fn read_enum_struct_variant_field<T>(&mut self, _: &str, f_idx: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        self.read_enum_variant_arg(f_idx, f)
    }
    fn read_struct<T>(&mut self, _: &str, _: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        f(self)
    }
    fn read_struct_field<T>(&mut self, _: &str, _: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        f(self)
    }
    fn read_tuple<T>(&mut self,
    f: |&mut DecoderReader<R>, uint| -> Result<T, IoError>) ->
    Result<T, IoError> {
        let len = try!(self.reader.read_be_uint());
        f(self, len)
    }
    fn read_tuple_arg<T>(&mut self, _: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        f(self)
    }
    fn read_tuple_struct<T>(&mut self, _: &str,
    f: |&mut DecoderReader<R>, uint| -> Result<T, IoError>) ->
    Result<T, IoError> {
        self.read_tuple(f)
    }
    fn read_tuple_struct_arg<T>(&mut self, a_idx: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        self.read_tuple_arg(a_idx, f)
    }
    fn read_option<T>(&mut self,
    f: |&mut DecoderReader<R>, bool| -> Result<T, IoError>) ->
    Result<T, IoError> {
        match try!(self.reader.read_u8()) {
            1 => f(self, true),
            _ => f(self, false)
        }
    }
    fn read_seq<T>(&mut self,
    f: |&mut DecoderReader<R>, uint| -> Result<T, IoError>) ->
    Result<T, IoError> {
        let len = try!(self.reader.read_be_uint());
        f(self, len)
    }
    fn read_seq_elt<T>(&mut self, _: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        f(self)
    }
    fn read_map<T>(&mut self,
    f: |&mut DecoderReader<R>, uint| -> Result<T, IoError>) ->
    Result<T, IoError> {
        let len = try!(self.reader.read_be_uint());
        f(self, len)
    }
    fn read_map_elt_key<T>(&mut self, _: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
        f(self)
    }
    fn read_map_elt_val<T>(&mut self, _: uint,
    f: |&mut DecoderReader<R>| -> Result<T, IoError>) -> Result<T, IoError> {
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
