// This file has the purpose of shimming out std structs if we're running in no_std or alloc mode
// The following types should be exported:
// - Box<T>
// - StdError (renamed from std::error::Error)
// - String/ToString (if available)
// - Vec<T> (if available)
// - Either std::mem or core::mem
// - Either std::marker or core::marker
// - Either std::str or core::str
// - Either std::u32 or core::u32
// - Either std::fmt or core::fmt
// - Either std::result or core::result
// - vec![] macro (if available)

pub mod io;

#[cfg(feature = "std")]
mod inner {
    // std support, just re-export everything
    pub use std::boxed::Box;
    pub use std::error::Error as StdError;
    pub use std::string::{String, ToString};
    pub use std::vec;
    pub use std::vec::Vec;
    pub use std::{fmt, marker, mem, result, str, u32};

    impl<R: std::io::Read> super::io::Read for R {
        fn read(&mut self, out: &mut [u8]) -> Result<usize, super::io::Error> {
            std::io::Read::read(self, out)
        }
        fn read_exact(&mut self, out: &mut [u8]) -> Result<(), super::io::Error> {
            std::io::Read::read_exact(self, out)
        }
    }
    impl<W: std::io::Write> super::io::Write for W {
        fn write(&mut self, buf: &[u8]) -> Result<usize, super::io::Error> {
            std::io::Write::write(self, buf)
        }
    }

    pub fn box_new<T>(t: T) -> Box<T> {
        Box::new(t)
    }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod inner {
    // alloc support, use the alloc crate
    extern crate alloc;
    pub use alloc::boxed::Box;
    pub use alloc::string::{String, ToString};
    pub use alloc::vec;
    pub use alloc::vec::Vec;
    pub use core::{fmt, marker, mem, result, str, u32};

    pub trait StdError {}

    pub fn box_new<T>(t: T) -> Box<T> {
        Box::new(t)
    }

    impl<'a> super::io::Write for &'a mut Vec<u8> {
        fn write(&mut self, buf: &[u8]) -> Result<usize, super::io::Error> {
            self.extend_from_slice(buf);
            Ok(buf.len())
        }
    }
    impl super::io::Write for Vec<u8> {
        fn write(&mut self, buf: &[u8]) -> Result<usize, super::io::Error> {
            self.extend_from_slice(buf);
            Ok(buf.len())
        }
    }
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
mod inner {
    // No alloc and no std, use only the bare minimum

    pub use core::{fmt, marker, mem, result, str, u32};

    // Box is just a newtype for T
    pub type Box<T> = T;

    pub fn box_new<T>(t: T) -> Box<T> {
        t
    }

    // TODO: Vec<T>
    // pub type Vec<T> = &'static [T];

    // TODO: Read/Write traits
    pub trait StdError {}
}

pub use self::inner::*;
