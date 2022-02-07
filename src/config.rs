//! The config module is used to change the behavior of bincode's encoding and decoding logic.
//!
//! *Important* make sure you use the same config for encoding and decoding, or else bincode will not work properly.
//!
//! To use a config, first create a type of [Configuration]. This type will implement trait [Config] for use with bincode.
//!
//! ```
//! let config = bincode::config::standard()
//!     // pick one of:
//!     .with_big_endian()
//!     .with_little_endian()
//!     // pick one of:
//!     .with_variable_int_encoding()
//!     .with_fixed_int_encoding()
//!     // pick one of:
//!     .skip_fixed_array_length()
//!     .write_fixed_array_length();
//! ```
//!
//! See [Configuration] for more information on the configuration options.

pub(crate) use self::internal::*;
use core::marker::PhantomData;

/// The Configuration struct is used to build bincode configurations. The [Config] trait is implemented
/// by this struct when a valid configuration has been constructed.
///
/// The following methods are mutually exclusive and will overwrite each other. The last call to one of these methods determines the behavior of the configuration:
///
/// - [with_little_endian] and [with_big_endian]
/// - [with_fixed_int_encoding] and [with_variable_int_encoding]
/// - [skip_fixed_array_length] and [write_fixed_array_length]
///
///
/// [with_little_endian]: #method.with_little_endian
/// [with_big_endian]: #method.with_big_endian
/// [with_fixed_int_encoding]: #method.with_fixed_int_encoding
/// [with_variable_int_encoding]: #method.with_variable_int_encoding
/// [skip_fixed_array_length]: #method.skip_fixed_array_length
/// [write_fixed_array_length]: #method.write_fixed_array_length
#[derive(Copy, Clone)]
pub struct Configuration<E = LittleEndian, I = Varint, A = SkipFixedArrayLength, L = NoLimit> {
    _e: PhantomData<E>,
    _i: PhantomData<I>,
    _a: PhantomData<A>,
    _l: PhantomData<L>,
}

/// The default config for bincode 2.0. By default this will be:
/// - Little endian
/// - Variable int encoding
/// - Skip fixed array length
pub const fn standard() -> Configuration {
    generate()
}

/// Creates the "legacy" default config. This is the default config that was present in bincode 1.0
/// - Little endian
/// - Fixed int length encoding
/// - Write array lengths
pub const fn legacy() -> Configuration<LittleEndian, Fixint, WriteFixedArrayLength, NoLimit> {
    generate()
}

const fn generate<_E, _I, _A, _L>() -> Configuration<_E, _I, _A, _L> {
    Configuration {
        _e: PhantomData,
        _i: PhantomData,
        _a: PhantomData,
        _l: PhantomData,
    }
}

// When adding more features to configuration, follow these steps:
// - Create 2 or more structs that can be used as a type (e.g. Limit and NoLimit)
// - Add an `Internal...Config` to the `internal` module
// - Make sure `Config` and `impl<T> Config for T` extend from this new trait
// - Add a generic to `Configuration`
// - Add this generic to `const fn generate<...>()`
// - Add this generic to _every_ function in `Configuration`
// - Add your new methods

impl<E, I, A, L> Configuration<E, I, A, L> {
    /// Makes bincode encode all integer types in big endian.
    pub const fn with_big_endian(self) -> Configuration<BigEndian, I, A, L> {
        generate()
    }

    /// Makes bincode encode all integer types in little endian.
    pub const fn with_little_endian(self) -> Configuration<LittleEndian, I, A, L> {
        generate()
    }

    /// Makes bincode encode all integer types with a variable integer encoding.
    ///
    /// Encoding an unsigned integer v (of any type excepting u8) works as follows:
    ///
    /// 1. If `u < 251`, encode it as a single byte with that value.
    /// 2. If `251 <= u < 2**16`, encode it as a literal byte 251, followed by a u16 with value `u`.
    /// 3. If `2**16 <= u < 2**32`, encode it as a literal byte 252, followed by a u32 with value `u`.
    /// 4. If `2**32 <= u < 2**64`, encode it as a literal byte 253, followed by a u64 with value `u`.
    /// 5. If `2**64 <= u < 2**128`, encode it as a literal byte 254, followed by a
    ///   u128 with value `u`.
    ///
    /// Then, for signed integers, we first convert to unsigned using the zigzag algorithm,
    /// and then encode them as we do for unsigned integers generally. The reason we use this
    /// algorithm is that it encodes those values which are close to zero in less bytes; the
    /// obvious algorithm, where we encode the cast values, gives a very large encoding for all
    /// negative values.
    ///
    /// The zigzag algorithm is defined as follows:
    ///
    /// ```rust
    /// # type Signed = i32;
    /// # type Unsigned = u32;
    /// fn zigzag(v: Signed) -> Unsigned {
    ///     match v {
    ///         0 => 0,
    ///         // To avoid the edge case of Signed::min_value()
    ///         // !n is equal to `-n - 1`, so this is:
    ///         // !n * 2 + 1 = 2(-n - 1) + 1 = -2n - 2 + 1 = -2n - 1
    ///         v if v < 0 => !(v as Unsigned) * 2 - 1,
    ///         v if v > 0 => (v as Unsigned) * 2,
    /// #       _ => unreachable!()
    ///     }
    /// }
    /// ```
    ///
    /// And works such that:
    ///
    /// ```rust
    /// # let zigzag = |n: i64| -> u64 {
    /// #     match n {
    /// #         0 => 0,
    /// #         v if v < 0 => !(v as u64) * 2 + 1,
    /// #         v if v > 0 => (v as u64) * 2,
    /// #         _ => unreachable!(),
    /// #     }
    /// # };
    /// assert_eq!(zigzag(0), 0);
    /// assert_eq!(zigzag(-1), 1);
    /// assert_eq!(zigzag(1), 2);
    /// assert_eq!(zigzag(-2), 3);
    /// assert_eq!(zigzag(2), 4);
    /// // etc
    /// assert_eq!(zigzag(i64::min_value()), u64::max_value());
    /// ```
    ///
    /// Note that u256 and the like are unsupported by this format; if and when they are added to the
    /// language, they may be supported via the extension point given by the 255 byte.
    pub const fn with_variable_int_encoding(self) -> Configuration<E, Varint, A, L> {
        generate()
    }

    /// Fixed-size integer encoding.
    ///
    /// * Fixed size integers are encoded directly
    /// * Enum discriminants are encoded as u32
    /// * Lengths and usize are encoded as u64
    pub const fn with_fixed_int_encoding(self) -> Configuration<E, Fixint, A, L> {
        generate()
    }

    /// Skip writing the length of fixed size arrays (`[u8; N]`) before writing the array
    ///
    /// **NOTE:** This is not supported if you're using the `bincode::serde::*` functions, the `#[bincode(with_serde)]` attribute, or the `Compat` struct.
    pub const fn skip_fixed_array_length(self) -> Configuration<E, I, SkipFixedArrayLength, L> {
        generate()
    }

    /// Write the length of fixed size arrays (`[u8; N]`) before writing the array
    pub const fn write_fixed_array_length(self) -> Configuration<E, I, WriteFixedArrayLength, L> {
        generate()
    }

    /// Sets the byte limit to `limit`.
    pub const fn with_limit<const N: usize>(self) -> Configuration<E, I, A, Limit<N>> {
        generate()
    }

    /// Clear the byte limit.
    pub const fn with_no_limit(self) -> Configuration<E, I, A, NoLimit> {
        generate()
    }
}

/// Indicates a type is valid for controlling the bincode configuration
pub trait Config:
    InternalEndianConfig
    + InternalArrayLengthConfig
    + InternalIntEncodingConfig
    + InternalLimitConfig
    + Copy
    + Clone
{
}

impl<T> Config for T where
    T: InternalEndianConfig
        + InternalArrayLengthConfig
        + InternalIntEncodingConfig
        + InternalLimitConfig
        + Copy
        + Clone
{
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct BigEndian {}

impl InternalEndianConfig for BigEndian {
    const ENDIAN: Endian = Endian::Big;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct LittleEndian {}

impl InternalEndianConfig for LittleEndian {
    const ENDIAN: Endian = Endian::Little;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct Fixint {}

impl InternalIntEncodingConfig for Fixint {
    const INT_ENCODING: IntEncoding = IntEncoding::Fixed;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct Varint {}

impl InternalIntEncodingConfig for Varint {
    const INT_ENCODING: IntEncoding = IntEncoding::Variable;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct SkipFixedArrayLength {}

impl InternalArrayLengthConfig for SkipFixedArrayLength {
    const SKIP_FIXED_ARRAY_LENGTH: bool = true;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct WriteFixedArrayLength {}

impl InternalArrayLengthConfig for WriteFixedArrayLength {
    const SKIP_FIXED_ARRAY_LENGTH: bool = false;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct NoLimit {}
impl InternalLimitConfig for NoLimit {
    const LIMIT: Option<usize> = None;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct Limit<const N: usize> {}
impl<const N: usize> InternalLimitConfig for Limit<N> {
    const LIMIT: Option<usize> = Some(N);
}

mod internal {
    use super::Configuration;

    pub trait InternalEndianConfig {
        const ENDIAN: Endian;
    }

    impl<E: InternalEndianConfig, I, A, L> InternalEndianConfig for Configuration<E, I, A, L> {
        const ENDIAN: Endian = E::ENDIAN;
    }

    #[derive(PartialEq, Eq)]
    pub enum Endian {
        Little,
        Big,
    }

    pub trait InternalIntEncodingConfig {
        const INT_ENCODING: IntEncoding;
    }

    impl<E, I: InternalIntEncodingConfig, A, L> InternalIntEncodingConfig
        for Configuration<E, I, A, L>
    {
        const INT_ENCODING: IntEncoding = I::INT_ENCODING;
    }

    #[derive(PartialEq, Eq)]
    pub enum IntEncoding {
        Fixed,
        Variable,
    }

    pub trait InternalArrayLengthConfig {
        const SKIP_FIXED_ARRAY_LENGTH: bool;
    }

    impl<E, I, A: InternalArrayLengthConfig, L> InternalArrayLengthConfig
        for Configuration<E, I, A, L>
    {
        const SKIP_FIXED_ARRAY_LENGTH: bool = A::SKIP_FIXED_ARRAY_LENGTH;
    }

    pub trait InternalLimitConfig {
        const LIMIT: Option<usize>;
    }

    impl<E, I, A, L: InternalLimitConfig> InternalLimitConfig for Configuration<E, I, A, L> {
        const LIMIT: Option<usize> = L::LIMIT;
    }
}
