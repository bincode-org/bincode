//! Contains

use super::{write::Writer, Encode};
use crate::{
    config::{Config, Endian, IntEncoding},
    error::EncodeError,
};
use core::marker::PhantomData;

/// An Encoder that writes bytes into a given writer `W`.
///
/// This struct should rarely be used.
/// In most cases, prefer any of the `encode` functions.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to write integers to the writer.
///
/// ```
/// # use bincode::enc::{write::SliceWriter, Encoder, Encodeable};
/// # use bincode::config::{self, Config};
/// # let config = config::Default.with_fixed_int_encoding().with_big_endian();
/// let slice: &mut [u8] = &mut [0, 0, 0, 0];
/// let mut encoder = Encoder::new(SliceWriter::new(slice), config);
/// // this u32 can be any Encodable
/// 5u32.encode(&mut encoder).unwrap();
/// assert_eq!(encoder.into_writer().bytes_written(), 4);
/// assert_eq!(slice, [0, 0, 0, 5]);
/// ```
pub struct Encoder<W: Writer, C: Config> {
    writer: W,
    config: PhantomData<C>,
}

impl<W: Writer, C: Config> Encoder<W, C> {
    /// Create a new Encoder
    pub fn new(writer: W, _config: C) -> Encoder<W, C> {
        Encoder {
            writer,
            config: PhantomData,
        }
    }

    /// Return the underlying writer
    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<'a, W: Writer, C: Config> Encode for &'a mut Encoder<W, C> {
    fn encode_u8(&mut self, val: u8) -> Result<(), EncodeError> {
        self.writer.write(&[val])
    }

    fn encode_u16(&mut self, val: u16) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u16(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_u32(&mut self, val: u32) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u32(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_u64(&mut self, val: u64) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u64(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_u128(&mut self, val: u128) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u128(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_usize(&mut self, val: usize) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_usize(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_i8(&mut self, val: i8) -> Result<(), EncodeError> {
        self.writer.write(&[val as u8])
    }

    fn encode_i16(&mut self, val: i16) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i16(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_i32(&mut self, val: i32) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i32(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_i64(&mut self, val: i64) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i64(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_i128(&mut self, val: i128) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i128(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_isize(&mut self, val: isize) -> Result<(), EncodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_isize(&mut self.writer, C::ENDIAN, val)
            }
            IntEncoding::Fixed => match C::ENDIAN {
                Endian::Big => self.writer.write(&val.to_be_bytes()),
                Endian::Little => self.writer.write(&val.to_le_bytes()),
            },
        }
    }

    fn encode_f32(&mut self, val: f32) -> Result<(), EncodeError> {
        match C::ENDIAN {
            Endian::Big => self.writer.write(&val.to_be_bytes()),
            Endian::Little => self.writer.write(&val.to_le_bytes()),
        }
    }

    fn encode_f64(&mut self, val: f64) -> Result<(), EncodeError> {
        match C::ENDIAN {
            Endian::Big => self.writer.write(&val.to_be_bytes()),
            Endian::Little => self.writer.write(&val.to_le_bytes()),
        }
    }

    fn encode_slice(&mut self, val: &[u8]) -> Result<(), EncodeError> {
        self.encode_usize(val.len())?;
        self.writer.write(val)
    }

    fn encode_array<const N: usize>(&mut self, val: [u8; N]) -> Result<(), EncodeError> {
        self.writer.write(&val)
    }

    fn encode_char(&mut self, val: char) -> Result<(), EncodeError> {
        encode_utf8(&mut self.writer, val)
    }
}

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;

fn encode_utf8(writer: &mut impl Writer, c: char) -> Result<(), EncodeError> {
    let code = c as u32;

    if code < MAX_ONE_B {
        writer.write(&[c as u8])
    } else if code < MAX_TWO_B {
        let mut buf = [0u8; 2];
        buf[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
        buf[1] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    } else if code < MAX_THREE_B {
        let mut buf = [0u8; 3];
        buf[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
        buf[1] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    } else {
        let mut buf = [0u8; 4];
        buf[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
        buf[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    }
}
