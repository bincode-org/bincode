#![feature(struct_variant)]

extern crate serialize;

pub use writer::EncoderWriter;
pub use reader::DecoderReader;

mod writer;
mod reader;

#[cfg(test)]
mod test;
