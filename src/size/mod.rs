//! Compile-time information about the encoded size of structs and enums.

mod impl_core;


/// Specifying types that have constant size. Mostly relevant for using `Fixint` configuration
pub trait ConstantSize {
    /// The encoded constant size in bytes
    const ENCODED_SIZE: usize;
}

/// Specifying types that have bounded maximum size.
pub trait MaxSize {
    /// The encoded maximum size in bytes
    const ENCODED_MAX_SIZE: usize;
}



