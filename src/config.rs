//! The config module is used to change the behavior of bincode's encoding and decoding logic.
//!
//! *Important* make sure you use the same config for encoding and decoding, or else bincode will not work properly.
//!
//! To use a config, first create a type of [struct@Default]. This type will implement trait [Config] for further configuration.
//!
//! ```
//! use bincode::config::{Config, Default};
//! let config = Default
//!     // pick one of:
//!     .with_big_endian()
//!     .with_little_endian()
//!     // pick one of:
//!     .with_variable_int_encoding()
//!     .with_fixed_int_encoding();
//! ```
//!
//! See [Config] for more information on the configuration options.

pub(crate) use self::internal::*;
use core::marker::PhantomData;

/// The config trait that is implemented by all types returned by this function, as well as [struct@Default].
///
/// The following methods are mutually exclusive and will overwrite each other. The last call to one of these methods determines the behavior of the configuration:
///
/// - [with_little_endian] and [with_big_endian]
/// - [with_fixed_int_encoding] and [with_variable_int_encoding]
///
///
/// [with_little_endian]: #method.with_little_endian
/// [with_big_endian]: #method.with_big_endian
/// [with_fixed_int_encoding]: #method.with_fixed_int_encoding
/// [with_variable_int_encoding]: #method.with_variable_int_encoding
pub trait Config: InternalConfig + Copy + Clone + Sized {
    /// Makes bincode encode all integer types in big endian.
    fn with_big_endian(self) -> BigEndian<Self> {
        BigEndian { _pd: PhantomData }
    }

    /// Makes bincode encode all integer types in little endian.
    fn with_little_endian(self) -> LittleEndian<Self> {
        LittleEndian { _pd: PhantomData }
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
    /// ```ignore
    /// fn zigzag(v: Signed) -> Unsigned {
    ///     match v {
    ///         0 => 0,
    ///         v if v < 0 => |v| * 2 - 1
    ///         v if v > 0 => v * 2
    ///     }
    /// }
    /// ```
    ///
    /// And works such that:
    ///
    /// ```ignore
    /// assert_eq!(zigzag(0), 0);
    /// assert_eq!(zigzag(-1), 1);
    /// assert_eq!(zigzag(1), 2);
    /// assert_eq!(zigzag(-2), 3);
    /// assert_eq!(zigzag(2), 4);
    /// assert_eq!(zigzag(i64::min_value()), u64::max_value());
    /// ```
    ///
    /// Note that u256 and the like are unsupported by this format; if and when they are added to the
    /// language, they may be supported via the extension point given by the 255 byte.
    fn with_variable_int_encoding(self) -> Varint<Self> {
        Varint { _pd: PhantomData }
    }

    /// Fixed-size integer encoding.
    ///
    /// * Fixed size integers are encoded directly
    /// * Enum discriminants are encoded as u32
    /// * Lengths and usize are encoded as u64
    fn with_fixed_int_encoding(self) -> Fixint<Self> {
        Fixint { _pd: PhantomData }
    }
}

impl<T: InternalConfig> Config for T {}

#[derive(Copy, Clone)]
pub struct Default;

impl InternalConfig for Default {
    const ENDIAN: Endian = Endian::Little;
    const INT_ENCODING: IntEncoding = IntEncoding::Variable;
    const LIMIT: Option<u64> = None;
    const ALLOW_TRAILING: bool = true;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct BigEndian<C: Config> {
    _pd: PhantomData<C>,
}

impl<C: InternalConfig> InternalConfig for BigEndian<C> {
    const ENDIAN: Endian = Endian::Big;
    const INT_ENCODING: IntEncoding = C::INT_ENCODING;
    const LIMIT: Option<u64> = C::LIMIT;
    const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct LittleEndian<C: Config> {
    _pd: PhantomData<C>,
}

impl<C: InternalConfig> InternalConfig for LittleEndian<C> {
    const ENDIAN: Endian = Endian::Little;
    const INT_ENCODING: IntEncoding = C::INT_ENCODING;
    const LIMIT: Option<u64> = C::LIMIT;
    const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct Fixint<C: Config> {
    _pd: PhantomData<C>,
}

impl<C: InternalConfig> InternalConfig for Fixint<C> {
    const ENDIAN: Endian = C::ENDIAN;
    const INT_ENCODING: IntEncoding = IntEncoding::Fixed;
    const LIMIT: Option<u64> = C::LIMIT;
    const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct Varint<C: Config> {
    _pd: PhantomData<C>,
}

impl<C: InternalConfig> InternalConfig for Varint<C> {
    const ENDIAN: Endian = C::ENDIAN;
    const INT_ENCODING: IntEncoding = IntEncoding::Variable;
    const LIMIT: Option<u64> = C::LIMIT;
    const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
}

mod internal {
    pub trait InternalConfig: Copy + Clone {
        const ENDIAN: Endian;
        const INT_ENCODING: IntEncoding;
        const LIMIT: Option<u64>;
        const ALLOW_TRAILING: bool;
    }

    #[derive(PartialEq, Eq)]
    pub enum Endian {
        Little,
        Big,
    }

    #[derive(PartialEq, Eq)]
    pub enum IntEncoding {
        Fixed,
        Variable,
    }

    impl<'a, C: InternalConfig> InternalConfig for &'a mut C
    where
        &'a mut C: Copy + Clone,
    {
        const ENDIAN: Endian = C::ENDIAN;
        const INT_ENCODING: IntEncoding = C::INT_ENCODING;
        const LIMIT: Option<u64> = C::LIMIT;
        const ALLOW_TRAILING: bool = C::ALLOW_TRAILING;
    }
}
