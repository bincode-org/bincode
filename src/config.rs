use byteorder;
use de::read::BincodeRead;
use error::{ErrorKind, Result};
use serde;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::mem::size_of;

pub(crate) use self::internal::*;

use self::EndianOption::*;
use self::LimitOption::*;

/// The default options for bincode serialization/deserialization.
///
/// ### Defaults
/// By default bincode will use little-endian encoding for multi-byte integers, and will not
/// limit the number of serialized/deserialized bytes.
#[derive(Copy, Clone)]
pub struct DefaultOptions(Infinite);

impl DefaultOptions {
    /// Get a default configuration object.
    ///
    /// ### Default Configuration:
    ///
    /// | Byte limit | Endianness |
    /// |------------|------------|
    /// | Unlimited  | Little     |
    pub fn new() -> DefaultOptions {
        DefaultOptions(Infinite)
    }
}

impl Default for DefaultOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalOptions for DefaultOptions {
    type Limit = Infinite;
    type Endian = LittleEndian;
    type IntEncoding = FixintEncoding;

    #[inline(always)]
    fn limit(&mut self) -> &mut Infinite {
        &mut self.0
    }
}

/// A configuration builder trait whose options Bincode will use
/// while serializing and deserializing.
///
/// ### Options
/// Endianness: The endianness with which multi-byte integers will be read/written.  *default: little endian*
///
/// Limit: The maximum number of bytes that will be read/written in a bincode serialize/deserialize. *default: unlimited*
///
/// ### Byte Limit Details
/// The purpose of byte-limiting is to prevent Denial-Of-Service attacks whereby malicious attackers get bincode
/// deserialization to crash your process by allocating too much memory or keeping a connection open for too long.
///
/// When a byte limit is set, bincode will return `Err` on any deserialization that goes over the limit, or any
/// serialization that goes over the limit.
/// Sets the byte limit to be unlimited.
/// This is the default.
pub trait Options: InternalOptions + Sized {
    /// Sets the byte limit to be unlimited.
    /// This is the default.
    fn with_no_limit(self) -> WithOtherLimit<Self, Infinite> {
        WithOtherLimit::new(self, Infinite)
    }

    /// Sets the byte limit to `limit`.
    fn with_limit(self, limit: u64) -> WithOtherLimit<Self, Bounded> {
        WithOtherLimit::new(self, Bounded(limit))
    }

    /// Sets the endianness to little-endian
    /// This is the default.
    fn with_little_endian(self) -> WithOtherEndian<Self, LittleEndian> {
        WithOtherEndian::new(self)
    }

    /// Sets the endianness to big-endian
    fn with_big_endian(self) -> WithOtherEndian<Self, BigEndian> {
        WithOtherEndian::new(self)
    }

    /// Sets the endianness to the the machine-native endianness
    fn with_native_endian(self) -> WithOtherEndian<Self, NativeEndian> {
        WithOtherEndian::new(self)
    }

    /// Sets the length encoding to varint
    fn with_varint_encoding(self) -> WithOtherIntEncoding<Self, VarintEncoding> {
        WithOtherIntEncoding::new(self)
    }

    /// Sets the length encoding to be fixed
    fn with_fixint_encoding(self) -> WithOtherIntEncoding<Self, FixintEncoding> {
        WithOtherIntEncoding::new(self)
    }

    /// Serializes a serializable object into a `Vec` of bytes using this configuration
    #[inline(always)]
    fn serialize<S: ?Sized + serde::Serialize>(self, t: &S) -> Result<Vec<u8>> {
        ::internal::serialize(t, self)
    }

    /// Returns the size that an object would be if serialized using Bincode with this configuration
    #[inline(always)]
    fn serialized_size<T: ?Sized + serde::Serialize>(self, t: &T) -> Result<u64> {
        ::internal::serialized_size(t, self)
    }

    /// Serializes an object directly into a `Writer` using this configuration
    ///
    /// If the serialization would take more bytes than allowed by the size limit, an error
    /// is returned and *no bytes* will be written into the `Writer`
    #[inline(always)]
    fn serialize_into<W: Write, T: ?Sized + serde::Serialize>(self, w: W, t: &T) -> Result<()> {
        ::internal::serialize_into(w, t, self)
    }

    /// Deserializes a slice of bytes into an instance of `T` using this configuration
    #[inline(always)]
    fn deserialize<'a, T: serde::Deserialize<'a>>(self, bytes: &'a [u8]) -> Result<T> {
        ::internal::deserialize(bytes, self)
    }

    /// TODO: document
    #[doc(hidden)]
    #[inline(always)]
    fn deserialize_in_place<'a, R, T>(self, reader: R, place: &mut T) -> Result<()>
    where
        R: BincodeRead<'a>,
        T: serde::de::Deserialize<'a>,
    {
        ::internal::deserialize_in_place(reader, self, place)
    }

    /// Deserializes a slice of bytes with state `seed` using this configuration.
    #[inline(always)]
    fn deserialize_seed<'a, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        bytes: &'a [u8],
    ) -> Result<T::Value> {
        ::internal::deserialize_seed(seed, bytes, self)
    }

    /// Deserializes an object directly from a `Read`er using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from<R: Read, T: serde::de::DeserializeOwned>(self, reader: R) -> Result<T> {
        ::internal::deserialize_from(reader, self)
    }

    /// Deserializes an object directly from a `Read`er with state `seed` using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_seed<'a, R: Read, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        ::internal::deserialize_from_seed(seed, reader, self)
    }

    /// Deserializes an object from a custom `BincodeRead`er using the default configuration.
    /// It is highly recommended to use `deserialize_from` unless you need to implement
    /// `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_custom<'a, R: BincodeRead<'a>, T: serde::de::DeserializeOwned>(
        self,
        reader: R,
    ) -> Result<T> {
        ::internal::deserialize_from_custom(reader, self)
    }

    /// Deserializes an object from a custom `BincodeRead`er with state `seed` using the default
    /// configuration. It is highly recommended to use `deserialize_from` unless you need to
    /// implement `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_custom_seed<'a, R: BincodeRead<'a>, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        ::internal::deserialize_from_custom_seed(seed, reader, self)
    }
}

impl<T: InternalOptions> Options for T {}

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

/// Little-endian byte ordering.
#[derive(Copy, Clone)]
pub struct LittleEndian;

/// Big-endian byte ordering.
#[derive(Copy, Clone)]
pub struct BigEndian;

/// The native byte ordering of the current system.
#[derive(Copy, Clone)]
pub struct NativeEndian;

impl BincodeByteOrder for LittleEndian {
    type Endian = byteorder::LittleEndian;
}

impl BincodeByteOrder for BigEndian {
    type Endian = byteorder::BigEndian;
}

impl BincodeByteOrder for NativeEndian {
    type Endian = byteorder::NativeEndian;
}

/// Fixed-size integer encoding.
///
/// * Fixed size integers are encoded directly
/// * Enum discriminants are encoded as u32
/// * Lengths and usize are encoded as u64
#[derive(Copy, Clone)]
pub struct FixintEncoding;

/// Variable-size integer encoding (excepting [ui]8).
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
#[derive(Copy, Clone)]
pub struct VarintEncoding;

const SINGLE_BYTE_MAX: u8 = 250;
const U16_BYTE: u8 = 251;
const U32_BYTE: u8 = 252;
const U64_BYTE: u8 = 253;
const U128_BYTE: u8 = 254;
const DESERIALIZE_EXTENSION_POINT_ERR: &str = r#"
Byte 255 is treated as an extension point; it should not be encoding anything.
Do you have a mismatched bincode version or configuration?
"#;

impl VarintEncoding {
    fn varint_size(n: u64) -> u64 {
        if n <= SINGLE_BYTE_MAX as u64 {
            1
        } else if n <= u16::max_value() as u64 {
            (1 + size_of::<u16>()) as u64
        } else if n <= u32::max_value() as u64 {
            (1 + size_of::<u32>()) as u64
        } else {
            (1 + size_of::<u64>()) as u64
        }
    }

    #[inline(always)]
    fn zigzag_encode(n: i64) -> u64 {
        if n < 0 {
            // let's avoid the edge case of i64::min_value()
            // !n is equal to `-n - 1`, so this is:
            // !n * 2 + 1 = 2(-n - 1) + 1 = -2n - 2 + 1 = -2n - 1
            !(n as u64) * 2 + 1
        } else {
            (n as u64) * 2
        }
    }

    #[inline(always)]
    fn zigzag_decode(n: u64) -> i64 {
        if n % 2 == 0 {
            // positive number
            (n / 2) as i64
        } else {
            // negative number
            // !m * 2 + 1 = n
            // !m * 2 = n - 1
            // !m = (n - 1) / 2
            // m = !((n - 1) / 2)
            // since we have n is odd, we have floor(n / 2) = floor((n - 1) / 2)
            !(n / 2) as i64
        }
    }

    fn serialize_varint<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        n: u64,
    ) -> Result<()> {
        if n <= SINGLE_BYTE_MAX as u64 {
            ser.serialize_byte(n as u8)
        } else if n <= u16::max_value() as u64 {
            ser.serialize_byte(U16_BYTE)?;
            ser.serialize_literal_u16(n as u16)
        } else if n <= u32::max_value() as u64 {
            ser.serialize_byte(U32_BYTE)?;
            ser.serialize_literal_u32(n as u32)
        } else {
            ser.serialize_byte(U64_BYTE)?;
            ser.serialize_literal_u64(n as u64)
        }
    }

    fn deserialize_varint<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<u64> {
        #[allow(ellipsis_inclusive_range_patterns)]
        match de.deserialize_byte()? {
            byte @ 0...SINGLE_BYTE_MAX => Ok(byte as u64),
            U16_BYTE => Ok(de.deserialize_literal_u16()? as u64),
            U32_BYTE => Ok(de.deserialize_literal_u32()? as u64),
            U64_BYTE => de.deserialize_literal_u64(),
            U128_BYTE => Err(Box::new(ErrorKind::Custom(
                "Invalid value (u128 range): you may have a version or configuration disagreement?"
                    .to_string(),
            ))),
            _ => Err(Box::new(ErrorKind::Custom(
                DESERIALIZE_EXTENSION_POINT_ERR.to_string(),
            ))),
        }
    }

    serde_if_integer128! {
        // see zigzag_encode and zigzag_decode for implementation comments
        #[inline(always)]
        fn zigzag128_encode(n: i128) -> u128 {
            if n < 0 {
                !(n as u128) * 2 + 1
            } else {
                (n as u128) * 2
            }
        }
        #[inline(always)]
        fn zigzag128_decode(n: u128) -> i128 {
            if n % 2 == 0 {
                (n / 2) as i128
            } else {
                !(n / 2) as i128
            }
        }

        fn varint128_size(n: u128) -> u64 {
            if n <= SINGLE_BYTE_MAX as u128 {
                1
            } else if n <= u16::max_value() as u128 {
                (1 + size_of::<u16>()) as u64
            } else if n <= u32::max_value() as u128 {
                (1 + size_of::<u32>()) as u64
            } else if n <= u64::max_value() as u128 {
                (1 + size_of::<u64>()) as u64
            } else {
                (1 + size_of::<u128>()) as u64
            }
        }

        fn serialize_varint128<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            n: u128,
        ) -> Result<()> {
            if n <= SINGLE_BYTE_MAX as u128 {
                ser.serialize_byte(n as u8)
            } else if n <= u16::max_value() as u128 {
                ser.serialize_byte(U16_BYTE)?;
                ser.serialize_literal_u16(n as u16)
            } else if n <= u32::max_value() as u128 {
                ser.serialize_byte(U32_BYTE)?;
                ser.serialize_literal_u32(n as u32)
            } else if n <= u64::max_value() as u128 {
                ser.serialize_byte(U64_BYTE)?;
                ser.serialize_literal_u64(n as u64)
            } else {
                ser.serialize_byte(U128_BYTE)?;
                ser.serialize_literal_u128(n)
            }
        }

        fn deserialize_varint128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<u128> {
            #[allow(ellipsis_inclusive_range_patterns)]
            match de.deserialize_byte()? {
                byte @ 0...SINGLE_BYTE_MAX => Ok(byte as u128),
                U16_BYTE => Ok(de.deserialize_literal_u16()? as u128),
                U32_BYTE => Ok(de.deserialize_literal_u32()? as u128),
                U64_BYTE => Ok(de.deserialize_literal_u64()? as u128),
                U128_BYTE => de.deserialize_literal_u128(),
                _ => Err(Box::new(ErrorKind::Custom(DESERIALIZE_EXTENSION_POINT_ERR.to_string()))),
            }
        }
    }
}

fn cast_u64_to_usize(n: u64) -> Result<usize> {
    if n <= usize::max_value() as u64 {
        Ok(n as usize)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid size {}: sizes must fit in a usize (0 to {})",
            n,
            usize::max_value()
        ))))
    }
}
fn cast_u64_to_u32(n: u64) -> Result<u32> {
    if n <= u32::max_value() as u64 {
        Ok(n as u32)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid u32 {}: you may have a version disagreement?",
            n,
        ))))
    }
}
fn cast_u64_to_u16(n: u64) -> Result<u16> {
    if n <= u16::max_value() as u64 {
        Ok(n as u16)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid u16 {}: you may have a version disagreement?",
            n,
        ))))
    }
}

fn cast_i64_to_i32(n: i64) -> Result<i32> {
    if n <= i32::max_value() as i64 && n >= i32::min_value() as i64 {
        Ok(n as i32)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid i32 {}: you may have a version disagreement?",
            n,
        ))))
    }
}
fn cast_i64_to_i16(n: i64) -> Result<i16> {
    if n <= i16::max_value() as i64 && n >= i16::min_value() as i64 {
        Ok(n as i16)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid i16 {}: you may have a version disagreement?",
            n,
        ))))
    }
}

impl IntEncoding for FixintEncoding {
    #[inline(always)]
    fn u16_size(_: u16) -> u64 {
        size_of::<u16>() as u64
    }
    #[inline(always)]
    fn u32_size(_: u32) -> u64 {
        size_of::<u32>() as u64
    }
    #[inline(always)]
    fn u64_size(_: u64) -> u64 {
        size_of::<u64>() as u64
    }

    #[inline(always)]
    fn i16_size(_: i16) -> u64 {
        size_of::<i16>() as u64
    }
    #[inline(always)]
    fn i32_size(_: i32) -> u64 {
        size_of::<i32>() as u64
    }
    #[inline(always)]
    fn i64_size(_: i64) -> u64 {
        size_of::<i64>() as u64
    }

    #[inline(always)]
    fn serialize_u16<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u16) -> Result<()> {
        ser.serialize_literal_u16(val)
    }
    #[inline(always)]
    fn serialize_u32<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u32) -> Result<()> {
        ser.serialize_literal_u32(val)
    }
    #[inline(always)]
    fn serialize_u64<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u64) -> Result<()> {
        ser.serialize_literal_u64(val)
    }

    #[inline(always)]
    fn serialize_i16<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i16) -> Result<()> {
        ser.serialize_literal_u16(val as u16)
    }
    #[inline(always)]
    fn serialize_i32<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i32) -> Result<()> {
        ser.serialize_literal_u32(val as u32)
    }
    #[inline(always)]
    fn serialize_i64<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i64) -> Result<()> {
        ser.serialize_literal_u64(val as u64)
    }

    #[inline(always)]
    fn deserialize_u16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u16> {
        de.deserialize_literal_u16()
    }
    #[inline(always)]
    fn deserialize_u32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u32> {
        de.deserialize_literal_u32()
    }
    #[inline(always)]
    fn deserialize_u64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u64> {
        de.deserialize_literal_u64()
    }

    #[inline(always)]
    fn deserialize_i16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i16> {
        Ok(de.deserialize_literal_u16()? as i16)
    }
    #[inline(always)]
    fn deserialize_i32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i32> {
        Ok(de.deserialize_literal_u32()? as i32)
    }
    #[inline(always)]
    fn deserialize_i64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i64> {
        Ok(de.deserialize_literal_u64()? as i64)
    }

    serde_if_integer128! {
        #[inline(always)]
        fn u128_size(_: u128) -> u64{
            size_of::<u128>() as u64
        }
        #[inline(always)]
        fn i128_size(_: i128) -> u64{
            size_of::<i128>() as u64
        }

        #[inline(always)]
        fn serialize_u128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: u128,
        ) -> Result<()> {
            ser.serialize_literal_u128(val)
        }
        #[inline(always)]
        fn serialize_i128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: i128,
        ) -> Result<()> {
            ser.serialize_literal_u128(val as u128)
        }
        #[inline(always)]
        fn deserialize_u128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<u128> {
            de.deserialize_literal_u128()
        }
        #[inline(always)]
        fn deserialize_i128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<i128> {
            Ok(de.deserialize_literal_u128()? as i128)
        }
    }
}

impl IntEncoding for VarintEncoding {
    #[inline(always)]
    fn u16_size(n: u16) -> u64 {
        Self::varint_size(n as u64)
    }
    #[inline(always)]
    fn u32_size(n: u32) -> u64 {
        Self::varint_size(n as u64)
    }
    #[inline(always)]
    fn u64_size(n: u64) -> u64 {
        Self::varint_size(n)
    }

    #[inline(always)]
    fn i16_size(n: i16) -> u64 {
        Self::varint_size(Self::zigzag_encode(n as i64))
    }
    #[inline(always)]
    fn i32_size(n: i32) -> u64 {
        Self::varint_size(Self::zigzag_encode(n as i64))
    }
    #[inline(always)]
    fn i64_size(n: i64) -> u64 {
        Self::varint_size(Self::zigzag_encode(n))
    }

    #[inline(always)]
    fn serialize_u16<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u16) -> Result<()> {
        Self::serialize_varint(ser, val as u64)
    }
    #[inline(always)]
    fn serialize_u32<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u32) -> Result<()> {
        Self::serialize_varint(ser, val as u64)
    }
    #[inline(always)]
    fn serialize_u64<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u64) -> Result<()> {
        Self::serialize_varint(ser, val)
    }

    #[inline(always)]
    fn serialize_i16<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i16) -> Result<()> {
        Self::serialize_varint(ser, Self::zigzag_encode(val as i64))
    }
    #[inline(always)]
    fn serialize_i32<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i32) -> Result<()> {
        Self::serialize_varint(ser, Self::zigzag_encode(val as i64))
    }
    #[inline(always)]
    fn serialize_i64<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i64) -> Result<()> {
        Self::serialize_varint(ser, Self::zigzag_encode(val))
    }

    #[inline(always)]
    fn deserialize_u16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u16> {
        Self::deserialize_varint(de).and_then(cast_u64_to_u16)
    }
    #[inline(always)]
    fn deserialize_u32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u32> {
        Self::deserialize_varint(de).and_then(cast_u64_to_u32)
    }
    #[inline(always)]
    fn deserialize_u64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u64> {
        Self::deserialize_varint(de)
    }

    #[inline(always)]
    fn deserialize_i16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i16> {
        Self::deserialize_varint(de)
            .map(Self::zigzag_decode)
            .and_then(cast_i64_to_i16)
    }
    #[inline(always)]
    fn deserialize_i32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i32> {
        Self::deserialize_varint(de)
            .map(Self::zigzag_decode)
            .and_then(cast_i64_to_i32)
    }
    #[inline(always)]
    fn deserialize_i64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i64> {
        Self::deserialize_varint(de).map(Self::zigzag_decode)
    }

    serde_if_integer128! {
        #[inline(always)]
        fn u128_size(n: u128) -> u64 {
            Self::varint128_size(n)
        }
        #[inline(always)]
        fn i128_size(n: i128) -> u64 {
            Self::varint128_size(Self::zigzag128_encode(n))
        }
        #[inline(always)]
        fn serialize_u128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: u128,
        ) -> Result<()> {
            Self::serialize_varint128(ser, val)
        }
        #[inline(always)]
        fn serialize_i128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: i128,
        ) -> Result<()> {
            Self::serialize_varint128(ser, Self::zigzag128_encode(val))
        }
        #[inline(always)]
        fn deserialize_u128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<u128> {
            Self::deserialize_varint128(de)
        }
        #[inline(always)]
        fn deserialize_i128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<i128> {
            Self::deserialize_varint128(de).map(Self::zigzag128_decode)
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum LimitOption {
    Unlimited,
    Limited(u64),
}

#[derive(Clone, Copy, Debug)]
enum EndianOption {
    Big,
    Little,
    Native,
}

/// A configuration builder whose options Bincode will use
/// while serializing and deserializing.
///
/// ### Options
/// Endianness: The endianness with which multi-byte integers will be read/written.  *default: little endian*
/// Limit: The maximum number of bytes that will be read/written in a bincode serialize/deserialize. *default: unlimited*
///
/// ### Byte Limit Details
/// The purpose of byte-limiting is to prevent Denial-Of-Service attacks whereby malicious attackers get bincode
/// deserialization to crash your process by allocating too much memory or keeping a connection open for too long.
///
/// When a byte limit is set, bincode will return `Err` on any deserialization that goes over the limit, or any
/// serialization that goes over the limit.
#[derive(Clone, Debug)]
#[deprecated(
    since = "1.3.0",
    note = "please use the `DefaultOptions`/`OptionsExt` system instead"
)]
pub struct Config {
    limit: LimitOption,
    endian: EndianOption,
}

/// A configuration struct with a user-specified byte limit
#[derive(Clone, Copy)]
pub struct WithOtherLimit<O: Options, L: SizeLimit> {
    _options: O,
    pub(crate) new_limit: L,
}

/// A configuration struct with a user-specified endian order
#[derive(Clone, Copy)]
pub struct WithOtherEndian<O: Options, E: BincodeByteOrder> {
    options: O,
    _endian: PhantomData<E>,
}

/// A configuration struct with a user-specified length encoding
pub struct WithOtherIntEncoding<O: Options, I: IntEncoding> {
    options: O,
    _length: PhantomData<I>,
}

impl<O: Options, L: SizeLimit> WithOtherLimit<O, L> {
    #[inline(always)]
    pub(crate) fn new(options: O, limit: L) -> WithOtherLimit<O, L> {
        WithOtherLimit {
            _options: options,
            new_limit: limit,
        }
    }
}

impl<O: Options, E: BincodeByteOrder> WithOtherEndian<O, E> {
    #[inline(always)]
    pub(crate) fn new(options: O) -> WithOtherEndian<O, E> {
        WithOtherEndian {
            options,
            _endian: PhantomData,
        }
    }
}

impl<O: Options, I: IntEncoding> WithOtherIntEncoding<O, I> {
    #[inline(always)]
    pub(crate) fn new(options: O) -> WithOtherIntEncoding<O, I> {
        WithOtherIntEncoding {
            options,
            _length: PhantomData,
        }
    }
}

impl<O: Options, E: BincodeByteOrder + 'static> InternalOptions for WithOtherEndian<O, E> {
    type Limit = O::Limit;
    type Endian = E;
    type IntEncoding = O::IntEncoding;

    #[inline(always)]
    fn limit(&mut self) -> &mut O::Limit {
        self.options.limit()
    }
}

impl<O: Options, L: SizeLimit + 'static> InternalOptions for WithOtherLimit<O, L> {
    type Limit = L;
    type Endian = O::Endian;
    type IntEncoding = O::IntEncoding;

    fn limit(&mut self) -> &mut L {
        &mut self.new_limit
    }
}

impl<O: Options, I: IntEncoding + 'static> InternalOptions for WithOtherIntEncoding<O, I> {
    type Limit = O::Limit;
    type Endian = O::Endian;
    type IntEncoding = I;

    fn limit(&mut self) -> &mut O::Limit {
        self.options.limit()
    }
}

macro_rules! config_map {
    ($self:expr, $opts:ident => $call:expr) => {
        match ($self.limit, $self.endian) {
            (Unlimited, Little) => {
                let $opts = DefaultOptions::new().with_no_limit().with_little_endian();
                $call
            }
            (Unlimited, Big) => {
                let $opts = DefaultOptions::new().with_no_limit().with_big_endian();
                $call
            }
            (Unlimited, Native) => {
                let $opts = DefaultOptions::new().with_no_limit().with_native_endian();
                $call
            }

            (Limited(l), Little) => {
                let $opts = DefaultOptions::new().with_limit(l).with_little_endian();
                $call
            }
            (Limited(l), Big) => {
                let $opts = DefaultOptions::new().with_limit(l).with_big_endian();
                $call
            }
            (Limited(l), Native) => {
                let $opts = DefaultOptions::new().with_limit(l).with_native_endian();
                $call
            }
        }
    };
}

impl Config {
    #[inline(always)]
    pub(crate) fn new() -> Config {
        Config {
            limit: LimitOption::Unlimited,
            endian: EndianOption::Little,
        }
    }

    /// Sets the byte limit to be unlimited.
    /// This is the default.
    #[inline(always)]
    pub fn no_limit(&mut self) -> &mut Self {
        self.limit = LimitOption::Unlimited;
        self
    }

    /// Sets the byte limit to `limit`.
    #[inline(always)]
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = LimitOption::Limited(limit);
        self
    }

    /// Sets the endianness to little-endian
    /// This is the default.
    #[inline(always)]
    pub fn little_endian(&mut self) -> &mut Self {
        self.endian = EndianOption::Little;
        self
    }

    /// Sets the endianness to big-endian
    #[inline(always)]
    pub fn big_endian(&mut self) -> &mut Self {
        self.endian = EndianOption::Big;
        self
    }

    /// Sets the endianness to the the machine-native endianness
    #[inline(always)]
    pub fn native_endian(&mut self) -> &mut Self {
        self.endian = EndianOption::Native;
        self
    }

    /// Serializes a serializable object into a `Vec` of bytes using this configuration
    #[inline(always)]
    pub fn serialize<T: ?Sized + serde::Serialize>(&self, t: &T) -> Result<Vec<u8>> {
        config_map!(self, opts => ::internal::serialize(t, opts))
    }

    /// Returns the size that an object would be if serialized using Bincode with this configuration
    #[inline(always)]
    pub fn serialized_size<T: ?Sized + serde::Serialize>(&self, t: &T) -> Result<u64> {
        config_map!(self, opts => ::internal::serialized_size(t, opts))
    }

    /// Serializes an object directly into a `Writer` using this configuration
    ///
    /// If the serialization would take more bytes than allowed by the size limit, an error
    /// is returned and *no bytes* will be written into the `Writer`
    #[inline(always)]
    pub fn serialize_into<W: Write, T: ?Sized + serde::Serialize>(
        &self,
        w: W,
        t: &T,
    ) -> Result<()> {
        config_map!(self, opts => ::internal::serialize_into(w, t, opts))
    }

    /// Deserializes a slice of bytes into an instance of `T` using this configuration
    #[inline(always)]
    pub fn deserialize<'a, T: serde::Deserialize<'a>>(&self, bytes: &'a [u8]) -> Result<T> {
        config_map!(self, opts => ::internal::deserialize(bytes, opts))
    }

    /// TODO: document
    #[doc(hidden)]
    #[inline(always)]
    pub fn deserialize_in_place<'a, R, T>(&self, reader: R, place: &mut T) -> Result<()>
    where
        R: BincodeRead<'a>,
        T: serde::de::Deserialize<'a>,
    {
        config_map!(self, opts => ::internal::deserialize_in_place(reader, opts, place))
    }

    /// Deserializes a slice of bytes with state `seed` using this configuration.
    #[inline(always)]
    pub fn deserialize_seed<'a, T: serde::de::DeserializeSeed<'a>>(
        &self,
        seed: T,
        bytes: &'a [u8],
    ) -> Result<T::Value> {
        config_map!(self, opts => ::internal::deserialize_seed(seed, bytes, opts))
    }

    /// Deserializes an object directly from a `Read`er using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    pub fn deserialize_from<R: Read, T: serde::de::DeserializeOwned>(
        &self,
        reader: R,
    ) -> Result<T> {
        config_map!(self, opts => ::internal::deserialize_from(reader, opts))
    }

    /// Deserializes an object directly from a `Read`er with state `seed` using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    pub fn deserialize_from_seed<'a, R: Read, T: serde::de::DeserializeSeed<'a>>(
        &self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        config_map!(self, opts => ::internal::deserialize_from_seed(seed, reader, opts))
    }

    /// Deserializes an object from a custom `BincodeRead`er using the default configuration.
    /// It is highly recommended to use `deserialize_from` unless you need to implement
    /// `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    pub fn deserialize_from_custom<'a, R: BincodeRead<'a>, T: serde::de::DeserializeOwned>(
        &self,
        reader: R,
    ) -> Result<T> {
        config_map!(self, opts => ::internal::deserialize_from_custom(reader, opts))
    }

    /// Deserializes an object from a custom `BincodeRead`er with state `seed` using the default
    /// configuration. It is highly recommended to use `deserialize_from` unless you need to
    /// implement `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    pub fn deserialize_from_custom_seed<
        'a,
        R: BincodeRead<'a>,
        T: serde::de::DeserializeSeed<'a>,
    >(
        &self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        config_map!(self, opts => ::internal::deserialize_from_custom_seed(seed, reader, opts))
    }
}

mod internal {
    use super::*;
    use byteorder::ByteOrder;

    pub trait InternalOptions {
        type Limit: SizeLimit + 'static;
        type Endian: BincodeByteOrder + 'static;
        type IntEncoding: IntEncoding + 'static;

        fn limit(&mut self) -> &mut Self::Limit;
    }

    impl<'a, O: InternalOptions> InternalOptions for &'a mut O {
        type Limit = O::Limit;
        type Endian = O::Endian;
        type IntEncoding = O::IntEncoding;

        #[inline(always)]
        fn limit(&mut self) -> &mut Self::Limit {
            (*self).limit()
        }
    }

    /// A trait for stopping serialization and deserialization when a certain limit has been reached.
    pub trait SizeLimit {
        /// Tells the SizeLimit that a certain number of bytes has been
        /// read or written.  Returns Err if the limit has been exceeded.
        fn add(&mut self, n: u64) -> Result<()>;
        /// Returns the hard limit (if one exists)
        fn limit(&self) -> Option<u64>;
    }

    pub trait BincodeByteOrder {
        type Endian: ByteOrder + 'static;
    }

    pub trait IntEncoding {
        /// Gets the size (in bytes) that a value would be serialized to.
        fn u16_size(n: u16) -> u64;
        /// Gets the size (in bytes) that a value would be serialized to.
        fn u32_size(n: u32) -> u64;
        /// Gets the size (in bytes) that a value would be serialized to.
        fn u64_size(n: u64) -> u64;

        /// Gets the size (in bytes) that a value would be serialized to.
        fn i16_size(n: i16) -> u64;
        /// Gets the size (in bytes) that a value would be serialized to.
        fn i32_size(n: i32) -> u64;
        /// Gets the size (in bytes) that a value would be serialized to.
        fn i64_size(n: i64) -> u64;

        #[inline(always)]
        fn len_size(len: usize) -> u64 {
            Self::u64_size(len as u64)
        }

        /// Serializes a sequence length.
        #[inline(always)]
        fn serialize_len<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            len: usize,
        ) -> Result<()> {
            Self::serialize_u64(ser, len as u64)
        }

        fn serialize_u16<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            val: u16,
        ) -> Result<()>;

        fn serialize_u32<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            val: u32,
        ) -> Result<()>;

        fn serialize_u64<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            val: u64,
        ) -> Result<()>;

        fn serialize_i16<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            val: i16,
        ) -> Result<()>;

        fn serialize_i32<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            val: i32,
        ) -> Result<()>;

        fn serialize_i64<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            val: i64,
        ) -> Result<()>;

        /// Deserializes a sequence length.
        #[inline(always)]
        fn deserialize_len<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<usize> {
            Self::deserialize_u64(de).and_then(cast_u64_to_usize)
        }

        fn deserialize_u16<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<u16>;

        fn deserialize_u32<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<u32>;

        fn deserialize_u64<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<u64>;

        fn deserialize_i16<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<i16>;

        fn deserialize_i32<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<i32>;

        fn deserialize_i64<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<i64>;

        serde_if_integer128! {
            fn u128_size(v: u128) -> u64;
            fn i128_size(v: i128) -> u64;
            fn serialize_u128<W: Write, O: Options>(
                ser: &mut ::Serializer<W, O>,
                val: u128,
            ) -> Result<()>;
            fn deserialize_u128<'de, R: BincodeRead<'de>, O: Options>(
                de: &mut ::Deserializer<R, O>,
            ) -> Result<u128>;
            fn serialize_i128<W: Write, O: Options>(
                ser: &mut ::Serializer<W, O>,
                val: i128,
            ) -> Result<()>;
            fn deserialize_i128<'de, R: BincodeRead<'de>, O: Options>(
                de: &mut ::Deserializer<R, O>,
            ) -> Result<i128>;
        }
    }
}

#[cfg(test)]
mod test {
    use super::VarintEncoding;

    #[test]
    fn test_zigzag_encode() {
        let zigzag = VarintEncoding::zigzag_encode;

        assert_eq!(zigzag(0), 0);
        for x in 1..512 {
            assert_eq!(zigzag(x), (x as u64) * 2);
            assert_eq!(zigzag(-x), (x as u64) * 2 - 1);
        }
    }

    #[test]
    fn test_zigzag_decode() {
        // zigzag'
        let zigzagp = VarintEncoding::zigzag_decode;
        for x in (0..512).map(|x| x * 2) {
            assert_eq!(zigzagp(x), x as i64 / 2);
            assert_eq!(zigzagp(x + 1), -(x as i64) / 2 - 1);
        }
    }

    #[test]
    fn test_zigzag_edge_cases() {
        let (zigzag, zigzagp) = (VarintEncoding::zigzag_encode, VarintEncoding::zigzag_decode);

        assert_eq!(zigzag(i64::max_value()), u64::max_value() - 1);
        assert_eq!(zigzag(i64::min_value()), u64::max_value());

        assert_eq!(zigzagp(u64::max_value() - 1), i64::max_value());
        assert_eq!(zigzagp(u64::max_value()), i64::min_value());
    }
}
