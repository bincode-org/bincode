//! Errors that can be encounting by Encoding and Decoding.

/// Errors that can be encountered by encoding a type
#[non_exhaustive]
#[derive(Debug)]
pub enum EncodeError {
    /// The writer ran out of storage.
    UnexpectedEnd,

    /// The RefCell<T> is already borrowed
    RefCellAlreadyBorrowed {
        /// The inner borrow error
        inner: core::cell::BorrowError,
        /// the type name of the RefCell being encoded that is currently borrowed.
        type_name: &'static str,
    },

    /// An uncommon error occured, see the inner text for more information
    Other(&'static str),

    /// A `std::path::Path` was being encoded but did not contain a valid `&str` representation
    #[cfg(feature = "std")]
    InvalidPathCharacters,

    /// The targetted writer encountered an `std::io::Error`
    #[cfg(feature = "std")]
    Io {
        /// The encountered error
        error: std::io::Error,
        /// The amount of bytes that were written before the error occured
        index: usize,
    },

    /// The encoder tried to encode a `Mutex` or `RwLock`, but the locking failed
    #[cfg(feature = "std")]
    LockFailed {
        /// The type name of the mutex for debugging purposes
        type_name: &'static str,
    },

    /// The encoder tried to encode a `SystemTime`, but it was before `SystemTime::UNIX_EPOCH`
    #[cfg(feature = "std")]
    InvalidSystemTime {
        /// The error that was thrown by the SystemTime
        inner: std::time::SystemTimeError,
        /// The SystemTime that caused the error
        time: std::time::SystemTime,
    },
}

/// Errors that can be encounted by decoding a type
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    /// The reader reached its end but more bytes were expected.
    UnexpectedEnd,

    /// Invalid type was found. The decoder tried to read type `expected`, but found type `found` instead.
    InvalidIntegerType {
        /// The type that was being read from the reader
        expected: IntegerType,
        /// The type that was encoded in the data
        found: IntegerType,
    },

    /// The decoder tried to decode any of the `NonZero*` types but the value is zero
    NonZeroTypeIsZero {
        /// The type that was being read from the reader
        non_zero_type: IntegerType,
    },

    /// Invalid enum variant was found. The decoder tried to decode variant index `found`, but the variant index should be between `min` and `max`.
    UnexpectedVariant {
        /// The type name that was being decoded.
        type_name: &'static str,

        /// The variants that are allowed
        allowed: AllowedEnumVariants,

        /// The index of the enum that the decoder encountered
        found: u32,
    },

    /// The decoder tried to decode a `str`, but an utf8 error was encountered.
    Utf8(core::str::Utf8Error),

    /// The decoder tried to decode a `char` and failed. The given buffer contains the bytes that are read at the moment of failure.
    InvalidCharEncoding([u8; 4]),

    /// The decoder tried to decode a `bool` and failed. The given value is what is actually read.
    InvalidBooleanValue(u8),

    /// The decoder tried to decode an array of length `required`, but the binary data contained an array of length `found`.
    ArrayLengthMismatch {
        /// The length of the array required by the rust type.
        required: usize,
        /// The length of the array found in the binary format.
        found: usize,
    },

    /// The decoder tried to decode a `CStr` or `CString`, but the incoming data contained a 0 byte
    #[cfg(feature = "std")]
    CStrNulError {
        /// The inner exception
        inner: std::ffi::FromBytesWithNulError,
    },
}

impl DecodeError {
    /// If the current error is `InvalidIntegerType`, change the `expected` and
    /// `found` values from `Ux` to `Ix`. This is needed to have correct error
    /// reporting in src/varint/decode_signed.rs since this calls
    /// src/varint/decode_unsigned.rs and needs to correct the `expected` and
    /// `found` types.
    pub(crate) fn change_integer_type_to_signed(self) -> DecodeError {
        match self {
            Self::InvalidIntegerType { expected, found } => Self::InvalidIntegerType {
                expected: expected.into_signed(),
                found: found.into_signed(),
            },
            other => other,
        }
    }
}

/// Indicates which enum variants are allowed
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum AllowedEnumVariants {
    /// All values between `min` and `max` (inclusive) are allowed
    #[allow(missing_docs)]
    Range { min: u32, max: u32 },
    /// Each one of these values is allowed
    Allowed(&'static [u32]),
}

/// Integer types. Used by [DecodeError]. These types have no purpose other than being shown in errors.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,

    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,

    Reserved,
}

impl IntegerType {
    /// Change the `Ux` value to the associated `Ix` value.
    /// Returns the old value if `self` is already `Ix`.
    pub(crate) fn into_signed(self) -> Self {
        match self {
            Self::U8 => Self::I8,
            Self::U16 => Self::I16,
            Self::U32 => Self::I32,
            Self::U64 => Self::I64,
            Self::U128 => Self::I128,
            Self::Usize => Self::Isize,

            other => other,
        }
    }
}
