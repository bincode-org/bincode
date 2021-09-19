use super::{U128_BYTE, U16_BYTE, U32_BYTE, U64_BYTE};
use crate::{
    config::Endian,
    de::read::Reader,
    error::{DecodeError, IntegerType},
};

#[allow(dead_code)]
pub fn varint_decode_u16<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<u16, DecodeError> {
    let mut byte = [0u8; 1];
    read.read(&mut byte)?;
    match byte[0] {
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes),
                Endian::Little => u16::from_le_bytes(bytes),
            })
        }
        U32_BYTE => Err(DecodeError::InvalidIntegerType {
            expected: IntegerType::U16,
            found: IntegerType::U32,
        }),
        U64_BYTE => Err(DecodeError::InvalidIntegerType {
            expected: IntegerType::U16,
            found: IntegerType::U64,
        }),
        U128_BYTE => Err(DecodeError::InvalidIntegerType {
            expected: IntegerType::U16,
            found: IntegerType::U128,
        }),
        x => Ok(x as u16),
    }
}

#[allow(dead_code)]
pub fn varint_decode_u32<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<u32, DecodeError> {
    let mut byte = [0u8; 1];
    read.read(&mut byte)?;
    match byte[0] {
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes) as u32,
                Endian::Little => u16::from_le_bytes(bytes) as u32,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u32::from_be_bytes(bytes),
                Endian::Little => u32::from_le_bytes(bytes),
            })
        }
        U64_BYTE => Err(DecodeError::InvalidIntegerType {
            expected: IntegerType::U32,
            found: IntegerType::U64,
        }),
        U128_BYTE => Err(DecodeError::InvalidIntegerType {
            expected: IntegerType::U32,
            found: IntegerType::U128,
        }),
        x => Ok(x as u32),
    }
}

#[allow(dead_code)]
pub fn varint_decode_u64<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<u64, DecodeError> {
    let mut byte = [0u8; 1];
    read.read(&mut byte)?;
    match byte[0] {
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes) as u64,
                Endian::Little => u16::from_le_bytes(bytes) as u64,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u32::from_be_bytes(bytes) as u64,
                Endian::Little => u32::from_le_bytes(bytes) as u64,
            })
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u64::from_be_bytes(bytes),
                Endian::Little => u64::from_le_bytes(bytes),
            })
        }
        U128_BYTE => Err(DecodeError::InvalidIntegerType {
            expected: IntegerType::U64,
            found: IntegerType::U128,
        }),
        x => Ok(x as u64),
    }
}

#[allow(dead_code)]
pub fn varint_decode_usize<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<usize, DecodeError> {
    let mut byte = [0u8; 1];
    read.read(&mut byte)?;
    match byte[0] {
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes) as usize,
                Endian::Little => u16::from_le_bytes(bytes) as usize,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u32::from_be_bytes(bytes) as usize,
                Endian::Little => u32::from_le_bytes(bytes) as usize,
            })
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u64::from_be_bytes(bytes) as usize,
                Endian::Little => u64::from_le_bytes(bytes) as usize,
            })
        }
        U128_BYTE => Err(DecodeError::InvalidIntegerType {
            expected: IntegerType::USize,
            found: IntegerType::U128,
        }),
        x => Ok(x as usize),
    }
}

#[allow(dead_code)]
pub fn varint_decode_u128<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<u128, DecodeError> {
    let mut byte = [0u8; 1];
    read.read(&mut byte)?;
    match byte[0] {
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes) as u128,
                Endian::Little => u16::from_le_bytes(bytes) as u128,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u32::from_be_bytes(bytes) as u128,
                Endian::Little => u32::from_le_bytes(bytes) as u128,
            })
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u64::from_be_bytes(bytes) as u128,
                Endian::Little => u64::from_le_bytes(bytes) as u128,
            })
        }
        U128_BYTE => {
            let mut bytes = [0u8; 16];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u128::from_be_bytes(bytes),
                Endian::Little => u128::from_le_bytes(bytes),
            })
        }
        x => Ok(x as u128),
    }
}

#[test]
fn test_decode_u16() {
    let cases: &[(&[u8], u16, u16)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);

        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }

    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U32_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U32,
            },
        ),
        (
            &[U64_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U64,
            },
        ),
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U128,
            },
        ),
        (&[U16_BYTE], DecodeError::UnexpectedEnd),
        (&[U16_BYTE, 0], DecodeError::UnexpectedEnd),
    ];

    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}

#[test]
fn test_decode_u32() {
    let cases: &[(&[u8], u32, u32)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);

        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }

    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U64_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U32,
                found: IntegerType::U64,
            },
        ),
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U32,
                found: IntegerType::U128,
            },
        ),
        (&[U16_BYTE], DecodeError::UnexpectedEnd),
        (&[U16_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0, 0, 0], DecodeError::UnexpectedEnd),
    ];

    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}

#[test]
fn test_decode_u64() {
    let cases: &[(&[u8], u64, u64)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0, 0, 10],
            72_057_594_037_9279_360,
            10,
        ),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);

        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }

    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U64,
                found: IntegerType::U128,
            },
        ),
        (&[U16_BYTE], DecodeError::UnexpectedEnd),
        (&[U16_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
    ];

    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}

#[test]
fn test_decode_u128() {
    let cases: &[(&[u8], u128, u128)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0, 0, 10],
            72_057_594_037_9279_360,
            10,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10],
            13_292_279_957_849_158_729_038_070_602_803_445_760,
            10,
        ),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);

        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }

    let errors: &[(&[u8], DecodeError)] = &[
        (&[U16_BYTE], DecodeError::UnexpectedEnd),
        (&[U16_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0, 0], DecodeError::UnexpectedEnd),
        (&[U32_BYTE, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U64_BYTE, 0, 0, 0, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U128_BYTE], DecodeError::UnexpectedEnd),
        (&[U128_BYTE, 0], DecodeError::UnexpectedEnd),
        (&[U128_BYTE, 0, 0], DecodeError::UnexpectedEnd),
        (&[U128_BYTE, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U128_BYTE, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U128_BYTE, 0, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (&[U128_BYTE, 0, 0, 0, 0, 0, 0], DecodeError::UnexpectedEnd),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd,
        ),
    ];

    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Little).unwrap_err();
        std::dbg!(slice);
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}
