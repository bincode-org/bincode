#[non_exhaustive]
#[derive(Debug)]
pub enum EncodeError {
    InvalidIntEncoding,
    UnexpectedEnd,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum DecodeError {
    UnexpectedEnd,
    /// Invalid type was found. The decoder tried to read type `expected`, but found type `found` instead.
    InvalidIntegerType {
        /// The type that was being read from the reader
        expected: IntegerType,
        /// The type that was encoded in the data
        found: IntegerType,
    },
    UnexpectedVariant {
        min: u32,
        max: u32,
        found: u32,
    },
}

#[non_exhaustive]
#[derive(Debug)]
pub enum IntegerType {
    U16,
    U32,
    U64,
    U128,
    USize,
}
