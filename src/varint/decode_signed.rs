#![allow(unused_variables)]

use crate::{config::Endian, de::read::Reader, error::DecodeError};

#[allow(dead_code)]
pub fn varint_decode_i16<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<i16, DecodeError> {
    let n = super::varint_decode_u16(read, endian)?;
    Ok(if n % 2 == 0 {
        // positive number
        (n / 2) as _
    } else {
        // negative number
        // !m * 2 + 1 = n
        // !m * 2 = n - 1
        // !m = (n - 1) / 2
        // m = !((n - 1) / 2)
        // since we have n is odd, we have floor(n / 2) = floor((n - 1) / 2)
        !(n / 2) as _
    })
}

#[allow(dead_code)]
pub fn varint_decode_i32<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<i32, DecodeError> {
    let n = super::varint_decode_u32(read, endian)?;
    Ok(if n % 2 == 0 {
        // positive number
        (n / 2) as _
    } else {
        // negative number
        // !m * 2 + 1 = n
        // !m * 2 = n - 1
        // !m = (n - 1) / 2
        // m = !((n - 1) / 2)
        // since we have n is odd, we have floor(n / 2) = floor((n - 1) / 2)
        !(n / 2) as _
    })
}

#[allow(dead_code)]
pub fn varint_decode_i64<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<i64, DecodeError> {
    let n = super::varint_decode_u64(read, endian)?;
    Ok(if n % 2 == 0 {
        // positive number
        (n / 2) as _
    } else {
        // negative number
        // !m * 2 + 1 = n
        // !m * 2 = n - 1
        // !m = (n - 1) / 2
        // m = !((n - 1) / 2)
        // since we have n is odd, we have floor(n / 2) = floor((n - 1) / 2)
        !(n / 2) as _
    })
}

#[allow(dead_code)]
pub fn varint_decode_i128<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<i128, DecodeError> {
    let n = super::varint_decode_u128(read, endian)?;
    Ok(if n % 2 == 0 {
        // positive number
        (n / 2) as _
    } else {
        // negative number
        // !m * 2 + 1 = n
        // !m * 2 = n - 1
        // !m = (n - 1) / 2
        // m = !((n - 1) / 2)
        // since we have n is odd, we have floor(n / 2) = floor((n - 1) / 2)
        !(n / 2) as _
    })
}

#[allow(dead_code)]
pub fn varint_decode_isize<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<isize, DecodeError> {
    varint_decode_i64(read, endian).map(|v| v as isize)
}
