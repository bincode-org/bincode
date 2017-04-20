use std::io::{Read as IoRead, Result as IoResult};
use std::cmp::min;

pub trait BincodeRead<'storage>: IoRead {
    fn read_into<'a, 'b>(&'a mut self, out: &'b mut [u8]) -> IoResult<usize>;
    fn advance_internal_buffer<'a: 'storage>(&'a mut self, length: usize) -> IoResult<&'a [u8]>;
}

pub struct SliceReader<'storage> {
    slice: &'storage[u8]
}

pub struct IoReadReader<R> {
    reader: R,
    temp_buffer: Vec<u8>,
}

impl <'storage> SliceReader<'storage> {
    pub fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader {
            slice: bytes,
        }
    }
}

impl <R> IoReadReader<R> {
    pub fn new(r: R) -> IoReadReader<R> {
        IoReadReader {
            reader: r,
            temp_buffer: vec![],
        }
    }
}

impl <'storage> IoRead for SliceReader<'storage> {
    fn read(&mut self, out: & mut [u8]) -> IoResult<usize> {
        self.read_into(out)
    }
}

impl <R: IoRead> IoRead for IoReadReader<R> {
    fn read(&mut self, out: & mut [u8]) -> IoResult<usize> {
        self.read_into(out)
    }
}

impl <'storage> BincodeRead<'storage> for SliceReader<'storage> {
    fn read_into<'a, 'b>(&'a mut self, out: &'b mut [u8]) -> IoResult<usize> {
        let write_length = min(self.slice.len(), out.len());
        (&mut out[..write_length]).copy_from_slice(&self.slice[..write_length]);
        self.slice = &self.slice[write_length ..];

        Ok(write_length)
    }

    fn advance_internal_buffer<'a: 'storage>(&'a mut self, length: usize) -> IoResult<&'a [u8]> {
        let split_point = min(self.slice.len(), length);
        let (before, after) = self.slice.split_at(split_point);
        self.slice = after;
        Ok(before)
    }
}

impl <'storage, R> BincodeRead<'storage> for IoReadReader<R> where R: IoRead {
    fn read_into<'a, 'b>(&'a mut self, out: &'b mut [u8]) -> IoResult<usize> {
        self.reader.read(out)
    }

    fn advance_internal_buffer<'a: 'storage>(&'a mut self, length: usize) -> IoResult<&'a [u8]> {
        let current_length = self.temp_buffer.len();
        if length > current_length{
            self.temp_buffer.reserve_exact(length - current_length);
        }

        self.reader.read_exact(&mut self.temp_buffer[..length])?;
        Ok(&self.temp_buffer[..length])
    }
}
