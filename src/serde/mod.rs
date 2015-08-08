use std::io::{Write, Read};
use ::SizeLimit;

pub use self::reader::{
    Deserializer,
    DeserializeResult,
    DeserializeError,
};

pub use self::writer::{
    Serializer,
    SerializeResult,
    SerializeError,
};

use self::writer::SizeChecker;

use serde_crate as serde;

mod reader;
mod writer;

pub fn serialize_into<W, T>(writer: &mut W,
                       value: &T,
                       size_limit: SizeLimit) -> SerializeResult<()>
    where W: Write,
          T: serde::Serialize,
{
    match size_limit {
        SizeLimit::Infinite => { }
        SizeLimit::Bounded(x) => {
            let mut size_checker = SizeChecker::new(x);
            try!(value.serialize(&mut size_checker))
        }
    }

    let mut serializer = Serializer::new(writer);
    serde::Serialize::serialize(value, &mut serializer)
}

pub fn serialize<T>(value: &T, size_limit: SizeLimit) -> SerializeResult<Vec<u8>>
    where T: serde::Serialize,
{
    // Since we are putting values directly into a vector, we can do size
    // computation out here and pre-allocate a buffer of *exactly*
    // the right size.
    let mut writer = match size_limit {
        SizeLimit::Bounded(size_limit) => {
            let actual_size = match serialized_size_bounded(value, size_limit) {
                Some(actual_size) => actual_size,
                None => { return Err(SerializeError::SizeLimit); }
            };
            Vec::with_capacity(actual_size as usize)
        }
        SizeLimit::Infinite => Vec::new()
    };

    try!(serialize_into(&mut writer, value, SizeLimit::Infinite));
    Ok(writer)
}

/// Returns the size that an object would be if serialized using bincode.
///
/// This is used internally as part of the check for encode_into, but it can
/// be useful for preallocating buffers if thats your style.
pub fn serialized_size<T: serde::Serialize>(value: &T) -> u64 {
    use std::u64::MAX;
    let mut size_checker = SizeChecker::new(MAX);
    value.serialize(&mut size_checker).ok();
    size_checker.written
}

/// Given a maximum size limit, check how large an object would be if it
/// were to be serialized.
///
/// If it can be serialized in `max` or fewer bytes, that number will be returned
/// inside `Some`.  If it goes over bounds, then None is returned.
pub fn serialized_size_bounded<T: serde::Serialize>(value: &T, max: u64) -> Option<u64> {
    let mut size_checker = SizeChecker::new(max);
    value.serialize(&mut size_checker).ok().map(|_| size_checker.written)
}

pub fn deserialize_from<R, T>(reader: &mut R, size_limit: SizeLimit) -> DeserializeResult<T>
    where R: Read,
          T: serde::Deserialize,
{
    let mut deserializer = Deserializer::new(reader, size_limit);
    serde::Deserialize::deserialize(&mut deserializer)
}

pub fn deserialize<T>(bytes: &[u8]) -> DeserializeResult<T>
    where T: serde::Deserialize,
{
    let mut reader = bytes;
    deserialize_from(&mut reader, SizeLimit::Infinite)
}

