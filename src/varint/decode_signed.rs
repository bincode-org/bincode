#![allow(unused_variables)]

use crate::{config::Endian, de::read::Reader, error::DecodeError};

#[allow(dead_code)]
pub fn varint_decode_i16<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<i16, DecodeError> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn varint_decode_i32<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<i32, DecodeError> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn varint_decode_i64<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<i64, DecodeError> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn varint_decode_i128<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<i128, DecodeError> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn varint_decode_isize<'a, R: Reader<'a>>(
    read: &mut R,
    endian: Endian,
) -> Result<isize, DecodeError> {
    unimplemented!()
}
