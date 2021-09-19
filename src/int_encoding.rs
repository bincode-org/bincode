use crate::{config::Endian, enc::write::Writer, error::Error};

pub trait IntEncoding {
    fn int_encode_u16<W: Writer>(writer: &mut W, endian: Endian, val: u16) -> Result<(), Error>;
    fn int_encode_u32<W: Writer>(writer: &mut W, endian: Endian, val: u32) -> Result<(), Error>;
    fn int_encode_u64<W: Writer>(writer: &mut W, endian: Endian, val: u64) -> Result<(), Error>;
    fn int_encode_u128<W: Writer>(writer: &mut W, endian: Endian, val: u128) -> Result<(), Error>;
    fn int_encode_usize<W: Writer>(writer: &mut W, endian: Endian, val: usize)
        -> Result<(), Error>;

    fn int_encode_i16<W: Writer>(writer: &mut W, endian: Endian, val: i16) -> Result<(), Error>;
    fn int_encode_i32<W: Writer>(writer: &mut W, endian: Endian, val: i32) -> Result<(), Error>;
    fn int_encode_i64<W: Writer>(writer: &mut W, endian: Endian, val: i64) -> Result<(), Error>;
    fn int_encode_i128<W: Writer>(writer: &mut W, endian: Endian, val: i128) -> Result<(), Error>;
    fn int_encode_isize<W: Writer>(writer: &mut W, endian: Endian, val: isize)
        -> Result<(), Error>;

    fn int_encode_f32<W: Writer>(writer: &mut W, endian: Endian, val: f32) -> Result<(), Error>;
    fn int_encode_f64<W: Writer>(writer: &mut W, endian: Endian, val: f64) -> Result<(), Error>;
}

#[derive(Copy, Clone)]
pub struct VarintEncoding;

#[derive(Copy, Clone)]
pub struct FixintEncoding;

impl IntEncoding for FixintEncoding {
    fn int_encode_u16<W: Writer>(writer: &mut W, endian: Endian, val: u16) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_u32<W: Writer>(writer: &mut W, endian: Endian, val: u32) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_u64<W: Writer>(writer: &mut W, endian: Endian, val: u64) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_u128<W: Writer>(writer: &mut W, endian: Endian, val: u128) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_usize<W: Writer>(
        writer: &mut W,
        endian: Endian,
        val: usize,
    ) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_i16<W: Writer>(writer: &mut W, endian: Endian, val: i16) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_i32<W: Writer>(writer: &mut W, endian: Endian, val: i32) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_i64<W: Writer>(writer: &mut W, endian: Endian, val: i64) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_i128<W: Writer>(writer: &mut W, endian: Endian, val: i128) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_isize<W: Writer>(
        writer: &mut W,
        endian: Endian,
        val: isize,
    ) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_f32<W: Writer>(writer: &mut W, endian: Endian, val: f32) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }

    fn int_encode_f64<W: Writer>(writer: &mut W, endian: Endian, val: f64) -> Result<(), Error> {
        match endian {
            Endian::Big => writer.write(&val.to_be_bytes()),
            Endian::Little => writer.write(&val.to_le_bytes()),
        }
    }
}
