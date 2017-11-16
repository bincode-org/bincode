#![deny(missing_docs)]

//! Bincode is a crate for encoding and decoding using a tiny binary
//! serialization strategy.
//!
//! There are simple functions for encoding to `Vec<u8>` and decoding from
//! `&[u8]`, but the meat of the library is the `serialize_into` and `deserialize_from`
//! functions which respectively allow encoding into any `std::io::Write`
//! or decode from any `std::io::Read`.
//!
//! ## Modules
//! Until "default type parameters" lands, we have an extra module called `endian_choice`
//! that duplicates all of the core Bincode functionality but with the option to choose
//! which endianness the integers are encoded using.
//!
//! The default endianness is little.
//!
//! ### Using Basic Functions
//!
//! ```rust
//! extern crate bincode;
//! use bincode::{serialize, deserialize};
//! fn main() {
//!     // The object that we will serialize.
//!     let target = Some("hello world".to_string());
//!
//!     let encoded: Vec<u8>        = serialize(&target).unwrap();
//!     let decoded: Option<String> = deserialize(&encoded[..]).unwrap();
//!     assert_eq!(target, decoded);
//! }
//! ```

#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate byteorder;
extern crate serde;

mod config;
mod ser;
mod error;
mod de;
mod internal;

pub use error::{Error, ErrorKind, Result};
pub use config::Config;

/// TODO: Document
pub fn config() -> Config {
    Config::new()
}

/// Serializes an object directly into a `Writer` using the default configuration.
///
/// If the serialization would take more bytes than allowed by `size_limit`, an error
/// is returned and *no bytes* will be written into the `Writer`.
///
/// If this returns an `Error` (other than SizeLimit), assume that the
/// writer is in an invalid state, as writing could bail out in the middle of
/// serializing.
pub fn serialize_into<W, T: ?Sized, O>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: serde::Serialize,
{
    config().serialize_into(writer, value)
}

/// Serializes a serializable object into a `Vec` of bytes using the default configuration.
///
/// If the serialization would take more bytes than allowed by `size_limit`,
/// an error is returned.
pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: serde::Serialize,
{
    config().serialize(value)
}

/// Deserializes an object directly from a `Read`er using the default configuration.
///
/// If the provided `SizeLimit` is reached, the deserialization will bail immediately.
/// A SizeLimit can help prevent an attacker from flooding your server with
/// a neverending stream of values that runs your server out of memory.
///
/// If this returns an `Error`, assume that the buffer that you passed
/// in is in an invalid state, as the error could be returned during any point
/// in the reading.
pub fn deserialize_from<R, T>(reader: R) -> Result<T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    config().deserialize_from(reader)
}

/// Deserializes a slice of bytes into an object.
///
/// This method does not have a size-limit because if you already have the bytes
/// in memory, then you don't gain anything by having a limiter.
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    config().deserialize(bytes)
}

/// Returns the size that an object would be if serialized using Bincode.
///
/// This is used internally as part of the check for encode_into, but it can
/// be useful for preallocating buffers if thats your style.
pub fn serialized_size<T: ?Sized>(value: &T) -> Result<u64>
where
    T: serde::Serialize,
{
    config().serialized_size(value)
}
