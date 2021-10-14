use super::{
    read::{BorrowReader, Reader},
    BorrowDecode, Decode,
};
use crate::{
    config::{Config, Endian, IntEncoding},
    error::DecodeError,
};
use core::marker::PhantomData;

/// A Decoder that reads bytes from a given reader `R`.
///
/// This struct should rarely be used.
/// In most cases, prefer any of the `decode` functions.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to read integers out of the reader.
///
/// ```
/// # let slice: &[u8] = &[0, 0, 0, 0];
/// # let some_reader = bincode::de::read::SliceReader::new(slice);
/// use bincode::de::{Decoder, Decodable};
/// use bincode::config;
/// let mut decoder = Decoder::new(some_reader, config::Default);
/// // this u32 can be any Decodable
/// let value = u32::decode(&mut decoder).unwrap();
/// ```
pub struct Decoder<R, C: Config> {
    reader: R,
    config: PhantomData<C>,
}

impl<'de, R: Reader<'de>, C: Config> Decoder<R, C> {
    /// Construct a new Decoder
    pub fn new(reader: R, _config: C) -> Decoder<R, C> {
        Decoder {
            reader,
            config: PhantomData,
        }
    }

    /// Consume the decoder and return the inner reader
    pub fn into_reader(self) -> R {
        self.reader
    }
}

impl<'a, 'de, R: BorrowReader<'de>, C: Config> BorrowDecode<'de> for &'a mut Decoder<R, C> {
    fn decode_slice(&mut self, len: usize) -> Result<&'de [u8], DecodeError> {
        self.reader.take_bytes(len)
    }
}

impl<'a, 'de, R: Reader<'de>, C: Config> Decode for &'a mut Decoder<R, C> {
    fn decode_u8(&mut self) -> Result<u8, DecodeError> {
        let mut bytes = [0u8; 1];
        self.reader.read(&mut bytes)?;
        Ok(bytes[0])
    }

    fn decode_u16(&mut self) -> Result<u16, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => crate::varint::varint_decode_u16(&mut self.reader, C::ENDIAN),
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => u16::from_le_bytes(bytes),
                    Endian::Big => u16::from_be_bytes(bytes),
                })
            }
        }
    }

    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => crate::varint::varint_decode_u32(&mut self.reader, C::ENDIAN),
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => u32::from_le_bytes(bytes),
                    Endian::Big => u32::from_be_bytes(bytes),
                })
            }
        }
    }

    fn decode_u64(&mut self) -> Result<u64, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => crate::varint::varint_decode_u64(&mut self.reader, C::ENDIAN),
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => u64::from_le_bytes(bytes),
                    Endian::Big => u64::from_be_bytes(bytes),
                })
            }
        }
    }

    fn decode_u128(&mut self) -> Result<u128, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => crate::varint::varint_decode_u128(&mut self.reader, C::ENDIAN),
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => u128::from_le_bytes(bytes),
                    Endian::Big => u128::from_be_bytes(bytes),
                })
            }
        }
    }

    fn decode_usize(&mut self) -> Result<usize, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_usize(&mut self.reader, C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => u64::from_le_bytes(bytes),
                    Endian::Big => u64::from_be_bytes(bytes),
                } as usize)
            }
        }
    }

    fn decode_i8(&mut self) -> Result<i8, DecodeError> {
        let mut bytes = [0u8; 1];
        self.reader.read(&mut bytes)?;
        Ok(bytes[0] as i8)
    }

    fn decode_i16(&mut self) -> Result<i16, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => crate::varint::varint_decode_i16(&mut self.reader, C::ENDIAN),
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => i16::from_le_bytes(bytes),
                    Endian::Big => i16::from_be_bytes(bytes),
                })
            }
        }
    }

    fn decode_i32(&mut self) -> Result<i32, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => crate::varint::varint_decode_i32(&mut self.reader, C::ENDIAN),
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => i32::from_le_bytes(bytes),
                    Endian::Big => i32::from_be_bytes(bytes),
                })
            }
        }
    }

    fn decode_i64(&mut self) -> Result<i64, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => crate::varint::varint_decode_i64(&mut self.reader, C::ENDIAN),
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => i64::from_le_bytes(bytes),
                    Endian::Big => i64::from_be_bytes(bytes),
                })
            }
        }
    }

    fn decode_i128(&mut self) -> Result<i128, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => crate::varint::varint_decode_i128(&mut self.reader, C::ENDIAN),
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => i128::from_le_bytes(bytes),
                    Endian::Big => i128::from_be_bytes(bytes),
                })
            }
        }
    }

    fn decode_isize(&mut self) -> Result<isize, DecodeError> {
        match C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_isize(&mut self.reader, C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                self.reader.read(&mut bytes)?;
                Ok(match C::ENDIAN {
                    Endian::Little => i64::from_le_bytes(bytes),
                    Endian::Big => i64::from_be_bytes(bytes),
                } as isize)
            }
        }
    }

    fn decode_f32(&mut self) -> Result<f32, DecodeError> {
        let mut bytes = [0u8; 4];
        self.reader.read(&mut bytes)?;
        Ok(match C::ENDIAN {
            Endian::Little => f32::from_le_bytes(bytes),
            Endian::Big => f32::from_be_bytes(bytes),
        })
    }

    fn decode_f64(&mut self) -> Result<f64, DecodeError> {
        let mut bytes = [0u8; 8];
        self.reader.read(&mut bytes)?;
        Ok(match C::ENDIAN {
            Endian::Little => f64::from_le_bytes(bytes),
            Endian::Big => f64::from_be_bytes(bytes),
        })
    }

    fn decode_array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        let mut array = [0u8; N];
        self.reader.read(&mut array)?;
        Ok(array)
    }

    fn decode_char(&mut self) -> Result<char, DecodeError> {
        let mut array = [0u8; 4];

        // Look at the first byte to see how many bytes must be read
        self.reader.read(&mut array[..1])?;

        let width = utf8_char_width(array[0]);
        if width == 0 {
            return Err(DecodeError::InvalidCharEncoding(array));
        }
        if width == 1 {
            return Ok(array[0] as char);
        }

        // read the remaining pain
        self.reader.read(&mut array[1..width])?;
        let res = core::str::from_utf8(&array[..width])
            .ok()
            .and_then(|s| s.chars().next())
            .ok_or(DecodeError::InvalidCharEncoding(array))?;
        Ok(res)
    }
}

const UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x1F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x3F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x5F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x7F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0xBF
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];

// This function is a copy of core::str::utf8_char_width
const fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
