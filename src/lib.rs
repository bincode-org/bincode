#![deny(missing_docs)]

//! `bincode` is a crate for encoding and decoding using a tiny binary
//! serialization strategy.
//!
//! There are simple functions for encoding to `Vec<u8>` and decoding from
//! `&[u8]`, but the meat of the library is the `encode_into` and `decode_from`
//! functions which respectively allow encoding into a `std::io::Writer`
//! and decoding from a `std::io::Buffer`.
//!
//! ### Using Basic Functions
//!
//! ```rust
//! extern crate bincode;
//! use bincode::{serialize, deserialize, DEFAULT_CONFIG};
//! fn main() {
//!     // The object that we will serialize.
//!     let target = Some("hello world".to_string());
//!
//!     let encoded: Vec<u8>        = serialize(&target, DEFAULT_CONFIG).unwrap();
//!     let decoded: Option<String> = deserialize(&encoded[..], DEFAULT_CONFIG).unwrap();
//!     assert_eq!(target, decoded);
//! }
//! ```

#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate byteorder;
extern crate num_traits;
extern crate serde as serde_crate;

mod ser;
mod de;
mod internal;
mod config;

pub use internal::*;
use config::*;

pub use config::DEFAULT_CONFIG;

pub mod read {
    //! The types that the deserializer uses for optimizations
    pub use ::de::read::{SliceReader, BincodeRead, IoReadReader};
}
