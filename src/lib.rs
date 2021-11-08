#![no_std]
#![warn(missing_docs, unused_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Bincode is a crate for encoding and decoding using a tiny binary
//! serialization strategy.  Using it, you can easily go from having
//! an object in memory, quickly serialize it to bytes, and then
//! deserialize it back just as fast!
//!
//! # Serde
//!
//! Starting from bincode 2, serde is now an optional dependency. If you want to use serde, please enable the `serde` feature. See [Features](#features) for more information.
//!
//! # Features
//!
//! |Name  |Default?|Supported types for Encode/Decode|Enabled methods                                                  |Other|
//! |------|--------|-----------------------------------------|-----------------------------------------------------------------|-----|
//! |std   | Yes    ||`decode_from_reader` and `encode_into_writer`|
//! |alloc | Yes    |All common containers in alloc, like `Vec`, `String`, `Box`|`encode_to_vec`|
//! |atomic| Yes    |All `Atomic*` integer types, e.g. `AtomicUsize`, and `AtomicBool`||
//! |derive| Yes    |||Enables the `BorrowDecode`, `Decode` and `Encode` derive macros|
//! |serde | No     |TODO|TODO|TODO|
//!
//! # Example
//!
//! ```rust
//! use bincode::config::Configuration;
//!
//! let mut slice = [0u8; 100];
//!
//!     // You can encode any type that implements `enc::Encode`.
//!     // You can automatically implement this trait on custom types with the `derive` feature.
//! let input = (
//!     0u8,
//!     10u32,
//!     10000i128,
//!     'a',
//!     [0u8, 1u8, 2u8, 3u8]
//! );
//!
//! let length = bincode::encode_into_slice(
//!     input,
//!     &mut slice,
//!     Configuration::standard()
//! ).unwrap();
//!
//! let slice = &slice[..length];
//! println!("Bytes written: {:?}", slice);
//!
//! // Decoding works the same as encoding.
//! // The trait used is `de::Decode`, and can also be automatically implemented with the `derive` feature.
//! let decoded: (u8, u32, i128, char, [u8; 4]) = bincode::decode_from_slice(slice, Configuration::standard()).unwrap();
//!
//! assert_eq!(decoded, input);
//! ```

#![doc(html_root_url = "https://docs.rs/bincode/2.0.0-alpha.0")]
#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

mod features;
pub(crate) mod utils;
pub(crate) mod varint;

use de::read::Reader;
use enc::write::Writer;
pub use features::*;

pub mod config;
pub mod de;
pub mod enc;
pub mod error;

pub use de::{BorrowDecode, Decode};
pub use enc::Encode;

use config::Config;

/// Encode the given value into the given slice. Returns the amount of bytes that have been written.
///
/// See the [config] module for more information on configurations.
///
/// [config]: config/index.html
pub fn encode_into_slice<E: enc::Encode, C: Config>(
    val: E,
    dst: &mut [u8],
    config: C,
) -> Result<usize, error::EncodeError> {
    let writer = enc::write::SliceWriter::new(dst);
    let mut encoder = enc::EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Encode the given value into a custom [Writer].
///
/// See the [config] module for more information on configurations.
///
/// [config]: config/index.html
pub fn encode_into_writer<E: enc::Encode, W: Writer, C: Config>(
    val: E,
    writer: W,
    config: C,
) -> Result<(), error::EncodeError> {
    let mut encoder = enc::EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(())
}

/// Attempt to decode a given type `D` from the given slice.
///
/// See the [config] module for more information on configurations.
///
/// [config]: config/index.html
pub fn decode_from_slice<'a, D: de::BorrowDecode<'a>, C: Config>(
    src: &'a [u8],
    config: C,
) -> Result<D, error::DecodeError> {
    let reader = de::read::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::<_, C>::new(reader, config);
    D::borrow_decode(&mut decoder)
}

/// Attempt to decode a given type `D` from the given [Reader].
///
/// See the [config] module for more information on configurations.
///
/// [config]: config/index.html
pub fn decode_from_reader<D: de::Decode, R: Reader, C: Config>(
    reader: R,
    _config: C,
) -> Result<D, error::DecodeError> {
    let mut decoder = de::DecoderImpl::<_, C>::new(reader, _config);
    D::decode(&mut decoder)
}

// TODO: Currently our doctests fail when trying to include the specs because the specs depend on `derive` and `alloc`.
// But we want to have the specs in the docs always
#[cfg(all(feature = "alloc", feature = "derive"))]
pub mod spec {
    #![doc = include_str!("../docs/spec.md")]
}

// Test the examples in readme.md
#[cfg(all(feature = "alloc", feature = "derive", doctest))]
mod readme {
    #![doc = include_str!("../readme.md")]
}
