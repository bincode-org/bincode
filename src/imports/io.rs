// Bincode's write trait. In std this is automatically implemented for all types that implement std::io::Write
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        let len = self.write(buf)?;

        if len != buf.len() {
            Err(ErrorKind::UnexpectedEof.into())
        } else {
            Ok(())
        }
    }
}

// Bincode's read trait. In std this is automatically implemented for all types that implement std::io::Read
pub trait Read {
    fn read(&mut self, out: &mut [u8]) -> Result<usize, Error>;
    fn read_exact(&mut self, out: &mut [u8]) -> Result<(), Error>;
}

#[cfg(not(feature = "std"))]
impl<'a> Read for &'a [u8] {
    fn read(&mut self, out: &mut [u8]) -> Result<usize, Error> {
        if out.len() < self.len() {
            out.copy_from_slice(&self[..out.len()]);
            *self = &self[out.len()..];
            Ok(out.len())
        } else {
            let len = self.len();
            out[..len].copy_from_slice(self);
            *self = &[];
            Ok(len)
        }
    }
    fn read_exact(&mut self, out: &mut [u8]) -> Result<(), Error> {
        if out.len() < self.len() {
            out.copy_from_slice(&self[..out.len()]);
            *self = &self[out.len()..];
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedEof.into())
        }
    }
}

#[cfg(feature = "std")]
pub use std::io::{Error, ErrorKind};

#[cfg(not(feature = "std"))]
pub use self::error::*;
#[cfg(not(feature = "std"))]
mod error {
    #[derive(Debug)]
    pub struct Error {
        kind: ErrorKind,
        message: Option<&'static str>,
    }

    impl Error {
        pub fn description(&self) -> &str {
            if let Some(message) = &self.message {
                message
            } else {
                self.kind.description()
            }
        }
    }

    impl core::fmt::Display for Error {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(fmt, "{}", self.description())
        }
    }

    impl Error {
        pub fn kind(&self) -> ErrorKind {
            self.kind
        }
    }

    impl From<ErrorKind> for Error {
        fn from(kind: ErrorKind) -> Error {
            Error {
                kind,
                message: None,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ErrorKind {
        UnexpectedEof,
    }

    impl ErrorKind {
        pub fn description(&self) -> &str {
            match self {
                ErrorKind::UnexpectedEof => "Unexpected EOF",
            }
        }
    }
}
