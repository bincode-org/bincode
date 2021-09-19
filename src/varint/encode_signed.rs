use super::{varint_encode_u128, varint_encode_u16, varint_encode_u32, varint_encode_u64};
use crate::{config::Endian, enc::write::Writer, error::EncodeError};

pub fn varint_encode_i16<W: Writer>(
    writer: &mut W,
    endian: Endian,
    val: i16,
) -> Result<(), EncodeError> {
    varint_encode_u16(
        writer,
        endian,
        if val < 0 {
            // let's avoid the edge case of i16::min_value()
            // !n is equal to `-n - 1`, so this is:
            // !n * 2 + 1 = 2(-n - 1) + 1 = -2n - 2 + 1 = -2n - 1
            !(val as u16) * 2 + 1
        } else {
            (val as u16) * 2
        },
    )
}

pub fn varint_encode_i32<W: Writer>(
    writer: &mut W,
    endian: Endian,
    val: i32,
) -> Result<(), EncodeError> {
    varint_encode_u32(
        writer,
        endian,
        if val < 0 {
            // let's avoid the edge case of i32::min_value()
            // !n is equal to `-n - 1`, so this is:
            // !n * 2 + 1 = 2(-n - 1) + 1 = -2n - 2 + 1 = -2n - 1
            !(val as u32) * 2 + 1
        } else {
            (val as u32) * 2
        },
    )
}

pub fn varint_encode_i64<W: Writer>(
    writer: &mut W,
    endian: Endian,
    val: i64,
) -> Result<(), EncodeError> {
    varint_encode_u64(
        writer,
        endian,
        if val < 0 {
            // let's avoid the edge case of i64::min_value()
            // !n is equal to `-n - 1`, so this is:
            // !n * 2 + 1 = 2(-n - 1) + 1 = -2n - 2 + 1 = -2n - 1
            !(val as u64) * 2 + 1
        } else {
            (val as u64) * 2
        },
    )
}

pub fn varint_encode_i128<W: Writer>(
    writer: &mut W,
    endian: Endian,
    val: i128,
) -> Result<(), EncodeError> {
    varint_encode_u128(
        writer,
        endian,
        if val < 0 {
            // let's avoid the edge case of i128::min_value()
            // !n is equal to `-n - 1`, so this is:
            // !n * 2 + 1 = 2(-n - 1) + 1 = -2n - 2 + 1 = -2n - 1
            !(val as u128) * 2 + 1
        } else {
            (val as u128) * 2
        },
    )
}

pub fn varint_encode_isize<W: Writer>(
    writer: &mut W,
    endian: Endian,
    val: isize,
) -> Result<(), EncodeError> {
    // isize is being encoded as a i64
    varint_encode_i64(writer, endian, val as i64)
}

#[test]
fn test_encode_i16() {
    // TODO
}

#[test]
fn test_encode_i32() {
    // TODO
}

#[test]
fn test_encode_i64() {
    // TODO
}

#[test]
fn test_encode_i128() {
    // TODO
}
