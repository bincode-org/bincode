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
//! use bincode::{serialize, deserialize, Bounded};
//! fn main() {
//!     // The object that we will serialize.
//!     let target = Some("hello world".to_string());
//!     // The maximum size of the encoded message.
//!     let limit = Bounded(20);
//!
//!     let encoded: Vec<u8>        = serialize(&target, limit).unwrap();
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

pub use error::{Error, ErrorKind};
use error::Result;

/// A limit on the amount of bytes that can be read or written.
///
/// Size limits are an incredibly important part of both encoding and decoding.
///
/// In order to prevent DOS attacks on a decoder, it is important to limit the
/// amount of bytes that a single encoded message can be; otherwise, if you
/// are decoding bytes right off of a TCP stream for example, it would be
/// possible for an attacker to flood your server with a 3TB vec, causing the
/// decoder to run out of memory and crash your application!
/// Because of this, you can provide a maximum-number-of-bytes that can be read
/// during decoding, and the decoder will explicitly fail if it has to read
/// any more than that.
///
/// On the other side, you want to make sure that you aren't encoding a message
/// that is larger than your decoder expects.  By supplying a size limit to an
/// encoding function, the encoder will verify that the structure can be encoded
/// within that limit.  This verification occurs before any bytes are written to
/// the Writer, so recovering from an error is easy.
pub(crate) trait SizeLimit {
    /// Tells the SizeLimit that a certain number of bytes has been
    /// read or written.  Returns Err if the limit has been exceeded.
    fn add(&mut self, n: u64) -> Result<()>;
    /// Returns the hard limit (if one exists)
    fn limit(&self) -> Option<u64>;
}

/// A SizeLimit that restricts serialized or deserialized messages from
/// exceeding a certain byte length.
#[derive(Copy, Clone)]
pub struct Bounded(pub u64);

/// A SizeLimit without a limit!
/// Use this if you don't care about the size of encoded or decoded messages.
#[derive(Copy, Clone)]
pub struct Infinite;

impl SizeLimit for Bounded {
    #[inline(always)]
    fn add(&mut self, n: u64) -> Result<()> {
        if self.0 >= n {
            self.0 -= n;
            Ok(())
        } else {
            Err(Box::new(ErrorKind::SizeLimit))
        }
    }

    #[inline(always)]
    fn limit(&self) -> Option<u64> {
        Some(self.0)
    }
}

impl SizeLimit for Infinite {
    #[inline(always)]
    fn add(&mut self, _: u64) -> Result<()> {
        Ok(())
    }

    #[inline(always)]
    fn limit(&self) -> Option<u64> {
        None
    }
}
