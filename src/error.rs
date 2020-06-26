use crate::imports::fmt;
use crate::imports::str::Utf8Error;
use crate::imports::{io, Box};
use serde;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::imports::String;

/// The result of a serialization or deserialization operation.
pub type Result<T> = crate::imports::result::Result<T, Error>;

/// An error that can be produced during (de)serializing.
pub type Error = Box<ErrorKind>;

/// The kind of error that can be produced during a serialization or deserialization.
#[derive(Debug)]
pub enum ErrorKind {
    /// If the error stems from the reader/writer that is being used
    /// during (de)serialization, that error will be stored and returned here.
    Io(io::Error),
    /// Returned if the deserializer attempts to deserialize a string that is not valid utf8
    InvalidUtf8Encoding(Utf8Error),
    /// Returned if the deserializer attempts to deserialize a bool that was
    /// not encoded as either a 1 or a 0
    InvalidBoolEncoding(u8),
    /// Returned if the deserializer attempts to deserialize a char that is not in the correct format.
    InvalidCharEncoding,
    /// Returned if the deserializer attempts to deserialize the tag of an enum that is
    /// not in the expected ranges
    InvalidTagEncoding(usize),
    /// Serde has a deserialize_any method that lets the format hint to the
    /// object which route to take in deserializing.
    DeserializeAnyNotSupported,
    /// If (de)serializing a message takes more than the provided size limit, this
    /// error is returned.
    SizeLimit,
    /// Bincode can not encode sequences of unknown length (like iterators).
    SequenceMustHaveLength,

    /// Tried to cast from type `from_type` to `to_type`, but the cast failed. Often this means that the types in the data is different than the type you're trying to deserialze into.
    ///
    /// For example, bincode might try to convert an `u64` to an `u16`, but the value of the `u64` does not fit in the `u16`.
    InvalidCast {
        from_type: &'static str,
        to_type: &'static str,
    },

    /// A custom error message
    #[cfg(any(feature = "std", feature = "alloc"))]
    Custom(String),
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    Custom(&'static str),
}

impl crate::imports::StdError for ErrorKind {}

#[cfg(not(feature = "std"))]
impl serde::de::StdError for ErrorKind {}

impl ErrorKind {
    fn description(&self) -> &str {
        match *self {
            #[cfg(feature = "std")]
            ErrorKind::Io(ref err) => std::error::Error::description(err),
            #[cfg(not(feature = "std"))]
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::InvalidUtf8Encoding(_) => "string is not valid utf8",
            ErrorKind::InvalidBoolEncoding(_) => "invalid u8 while decoding bool",
            ErrorKind::InvalidCharEncoding => "char is not valid",
            ErrorKind::InvalidTagEncoding(_) => "tag for enum is not valid",
            ErrorKind::SequenceMustHaveLength => {
                "Bincode can only encode sequences and maps that have a knowable size ahead of time"
            }
            ErrorKind::DeserializeAnyNotSupported => {
                "Bincode doesn't support serde::Deserializer::deserialize_any"
            }
            ErrorKind::SizeLimit => "the size limit has been reached",
            ErrorKind::InvalidCast { .. } => "Encountered an invalid cast",
            ErrorKind::Custom(ref msg) => msg,
        }
    }
}
/*
impl StdError for ErrorKind {
    fn description(&self) -> &str {
        match *self {
            ErrorKind::Io(ref err) => error::Error::description(err),
            ErrorKind::InvalidUtf8Encoding(_) => "string is not valid utf8",
            ErrorKind::InvalidBoolEncoding(_) => "invalid u8 while decoding bool",
            ErrorKind::InvalidCharEncoding => "char is not valid",
            ErrorKind::InvalidTagEncoding(_) => "tag for enum is not valid",
            ErrorKind::SequenceMustHaveLength => {
                "Bincode can only encode sequences and maps that have a knowable size ahead of time"
            }
            ErrorKind::DeserializeAnyNotSupported => {
                "Bincode doesn't support serde::Deserializer::deserialize_any"
            }
            ErrorKind::SizeLimit => "the size limit has been reached",
            ErrorKind::Custom(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::InvalidUtf8Encoding(_) => None,
            ErrorKind::InvalidBoolEncoding(_) => None,
            ErrorKind::InvalidCharEncoding => None,
            ErrorKind::InvalidTagEncoding(_) => None,
            ErrorKind::SequenceMustHaveLength => None,
            ErrorKind::DeserializeAnyNotSupported => None,
            ErrorKind::SizeLimit => None,
            ErrorKind::Custom(_) => None,
        }
    }
}
*/

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Io(ref ioerr) => write!(fmt, "io error: {}", ioerr),
            ErrorKind::InvalidUtf8Encoding(ref e) => write!(fmt, "{}: {}", self.description(), e),
            ErrorKind::InvalidBoolEncoding(b) => {
                write!(fmt, "{}, expected 0 or 1, found {}", self.description(), b)
            }
            ErrorKind::InvalidCharEncoding => write!(fmt, "{}", self.description()),
            ErrorKind::InvalidTagEncoding(tag) => {
                write!(fmt, "{}, found {}", self.description(), tag)
            }
            ErrorKind::SequenceMustHaveLength => write!(fmt, "{}", self.description()),
            ErrorKind::SizeLimit => write!(fmt, "{}", self.description()),
            ErrorKind::DeserializeAnyNotSupported => write!(
                fmt,
                "Bincode does not support the serde::Deserializer::deserialize_any method"
            ),
            ErrorKind::InvalidCast { from_type, to_type } => write!(
                fmt,
                "Could not cast from {:?} to {:?} because the value would overflow",
                from_type, to_type
            ),
            ErrorKind::Custom(ref s) => s.fmt(fmt),
        }
    }
}

impl serde::de::Error for Error {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn custom<T: fmt::Display>(desc: T) -> Error {
        ErrorKind::Custom(crate::imports::ToString::to_string(&desc)).into()
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn custom<T: fmt::Display>(desc: T) -> Error {
        panic!("{}", desc)
    }
}

impl serde::ser::Error for Error {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ErrorKind::Custom(crate::imports::ToString::to_string(&msg)).into()
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn custom<T: fmt::Display>(desc: T) -> Error {
        panic!("{}", desc)
    }
}
