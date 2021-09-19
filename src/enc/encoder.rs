use super::write::Writer;
use super::Encode;
use crate::{
    config::{Config, Endian, IntEncoding},
    error::Error,
};
use core::marker::PhantomData;

pub struct Encoder<W: Writer, C: Config> {
    writer: W,
    config: PhantomData<C>,
}

impl<W: Writer, C: Config> Encoder<W, C> {
    pub fn new(writer: W) -> Encoder<W, C> {
        Encoder {
            writer,
            config: PhantomData,
        }
    }

    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<'a, W: Writer, C: Config> Encode for &'a mut Encoder<W, C> {
    fn encode_u8(&mut self, val: u8) -> Result<(), Error> {
        self.writer.write(&[val])
    }

    fn encode_u16(&mut self, val: u16) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_u16(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_u32(&mut self, val: u32) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_u32(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_u64(&mut self, val: u64) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_u64(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_u128(&mut self, val: u128) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_u128(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_usize(&mut self, val: usize) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_usize(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_i8(&mut self, val: i8) -> Result<(), Error> {
        self.writer.write(&[val as u8])
    }

    fn encode_i16(&mut self, val: i16) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_i16(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_i32(&mut self, val: i32) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_i32(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_i64(&mut self, val: i64) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_i64(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_i128(&mut self, val: i128) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_i128(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_isize(&mut self, val: isize) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint_encoding::varint_encode_isize(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_f32(&mut self, val: f32) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => unimplemented!(), // crate::int_encoding::varint_encode_f32(&mut self.writer, C::ENDIAN, val),
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_f64(&mut self, val: f64) -> Result<(), Error> {
        match C::INT_ENCODING {
            IntEncoding::Variable => unimplemented!(), // crate::int_encoding::varint_encode_f64(&mut self.writer, C::ENDIAN, val),
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_slice(&mut self, val: &[u8]) -> Result<(), Error> {
        // TODO: Should this be swapped if we're big or little endian?
        self.writer.write(val)
    }
}
