use super::{
    read::{BorrowReader, Reader},
    BorrowDecode, Decode,
};
use crate::{
    config::{Config, Endian, IntEncoding},
    error::DecodeError,
};
use std::marker::PhantomData;

pub struct Decoder<R, C: Config> {
    reader: R,
    config: PhantomData<C>,
}

impl<'de, R: Reader<'de>, C: Config> Decoder<R, C> {
    pub fn new(reader: R, _config: C) -> Decoder<R, C> {
        Decoder {
            reader,
            config: PhantomData,
        }
    }

    pub fn into_reader(self) -> R {
        self.reader
    }
}

impl<'a, 'de, R: BorrowReader<'de>, C: Config> BorrowDecode<'de> for &'a mut Decoder<R, C> {
    fn decode_slice(&mut self, len: usize) -> Result<&'de [u8], DecodeError> {
        self.reader.take_bytes(len)
    }
}

impl<'a, 'de, R: Reader<'de>, C: Config> Decode<'de> for &'a mut Decoder<R, C> {
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
}
