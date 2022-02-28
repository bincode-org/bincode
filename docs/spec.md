# Serialization specification

*NOTE*: Serialization is done by `bincode_derive` by default. If you enable the `serde` flag, serialization with `serde-derive` is supported as well. `serde-derive` has the same guarantees as `bincode_derive` for now.

Related issue: <https://github.com/serde-rs/serde/issues/1756#issuecomment-689682123>

## Endian

By default `bincode` will serialize values in little endian encoding. This can be overwritten in the `Config`.

## Basic types

Boolean types are encoded with 1 byte for each boolean type, with `0` being `false`, `1` being true. Whilst deserializing every other value will throw an error.

All basic numeric types will be encoded based on the configured [IntEncoding](#intencoding).

All floating point types will take up exactly 4 (for `f32`) or 8 (for `f64`) bytes.

All tuples have no additional bytes, and are encoded in their specified order, e.g.
```rust
let tuple = (u32::min_value(), i32::max_value()); // 8 bytes
let encoded = bincode::encode_to_vec(tuple, bincode::config::legacy()).unwrap();
assert_eq!(encoded.as_slice(), &[
    0,   0,   0,   0,  // 4 bytes for first type:  u32
    255, 255, 255, 127 // 4 bytes for second type: i32
]);
```

## IntEncoding
Bincode currently supports 2 different types of `IntEncoding`. With the default config, `VarintEncoding` is selected.

### VarintEncoding
Encoding an unsigned integer v (of any type excepting u8/i8) works as follows:

1. If `u < 251`, encode it as a single byte with that value.
1. If `251 <= u < 2**16`, encode it as a literal byte 251, followed by a u16 with value `u`.
1. If `2**16 <= u < 2**32`, encode it as a literal byte 252, followed by a u32 with value `u`.
1. If `2**32 <= u < 2**64`, encode it as a literal byte 253, followed by a u64 with value `u`.
1. If `2**64 <= u < 2**128`, encode it as a literal byte 254, followed by a u128 with value `u`.

`usize` is being encoded/decoded as a `u64` and `isize` is being encoded/decoded as a `i64`.

See the documentation of [VarintEncoding](https://docs.rs/bincode/2.0.0-rc/bincode/config/struct.Configuration.html#method.with_variable_int_encoding) for more information.

### FixintEncoding

- Fixed size integers are encoded directly
- Enum discriminants are encoded as u32
- Lengths and usize are encoded as u64

See the documentation of [FixintEncoding](https://docs.rs/bincode/2.0.0-rc/bincode/config/struct.Configuration.html#method.with_fixed_int_encoding) for more information.

## Enums

Enums are encoded with their variant first, followed by optionally the variant fields. The variant index is based on the `IntEncoding` during serialization.

Both named and unnamed fields are serialized with their values only, and therefor encode to the same value.

```rust
#[derive(bincode::Encode)]
pub enum SomeEnum {
    A,
    B(u32),
    C { value: u32 },
}

// SomeEnum::A
let encoded = bincode::encode_to_vec(SomeEnum::A, bincode::config::legacy()).unwrap();
assert_eq!(encoded.as_slice(), &[
    0, 0, 0, 0, // first variant, A
    // no extra bytes because A has no fields
]);

// SomeEnum::B(0)
let encoded = bincode::encode_to_vec(SomeEnum::B(0), bincode::config::legacy()).unwrap();
assert_eq!(encoded.as_slice(), &[
    1, 0, 0, 0, // first variant, B
    0, 0, 0, 0  // B has 1 unnamed field, which is an u32, so 4 bytes
]);

// SomeEnum::C { value: 0u32 }
let encoded = bincode::encode_to_vec(SomeEnum::C { value: 0u32 }, bincode::config::legacy()).unwrap();
assert_eq!(encoded.as_slice(), &[
    2, 0, 0, 0, // first variant, C
    0, 0, 0, 0  // C has 1 named field which is a u32, so 4 bytes
]);
```

# Collections

Collections are encoded with their length value first, following by each entry of the collection. The length value is based on your `IntEncoding`.

**note**: fixed array length may not have their `len` encoded. See [Arrays](#arrays)

```rust
let list = vec![
    0u8,
    1u8,
    2u8
];

let encoded = bincode::encode_to_vec(list, bincode::config::legacy()).unwrap();
assert_eq!(encoded.as_slice(), &[
    3, 0, 0, 0, 0, 0, 0, 0, // length of 3u64
    0, // entry 0
    1, // entry 1
    2, // entry 2
]);
```

This also applies to e.g. `HashMap`, where each entry is a [tuple](#basic-types) of the key and value.

# String and &str

Both `String` and `&str` are treated as a `Vec<u8>`. See [Collections](#collections) for more information.

```rust
let str = "Hello"; // Could also be `String::new(...)`

let encoded = bincode::encode_to_vec(str, bincode::config::legacy()).unwrap();
assert_eq!(encoded.as_slice(), &[
    5, 0, 0, 0, 0, 0, 0, 0, // length of the string, 5 bytes
    b'H', b'e', b'l', b'l', b'o'
]);
```

# Arrays

Array length is encoded based on the `.write_fixed_array_length` and `.skip_fixed_array_length()` config. When an array length is written, it will be encoded as a `u64`.

Note that `&[T]` is encoded as a [Collection](#collections).


```rust
let arr: [u8; 5] = [10, 20, 30, 40, 50];
let encoded = bincode::encode_to_vec(arr, bincode::config::legacy()).unwrap();
assert_eq!(encoded.as_slice(), &[
    5, 0, 0, 0, 0, 0, 0, 0, // The length, as a u64
    10, 20, 30, 40, 50, // the bytes
]);

let encoded = bincode::encode_to_vec(arr, bincode::config::legacy().skip_fixed_array_length()).unwrap();
assert_eq!(encoded.as_slice(), &[
    // no length
    10, 20, 30, 40, 50, // the bytes
]);
```

This applies to any type `T` that implements `Encode`/`Decode`

```rust
#[derive(bincode::Encode)]
struct Foo {
    first: u8,
    second: u8
};

let arr: [Foo; 2] = [
    Foo {
        first: 10,
        second: 20,
    },
    Foo {
        first: 30,
        second: 40,
    },
];

let encoded = bincode::encode_to_vec(&arr, bincode::config::legacy()).unwrap();
assert_eq!(encoded.as_slice(), &[
    2, 0, 0, 0, 0, 0, 0, 0, // Length of the array
    10, 20, // First Foo
    30, 40, // Second Foo
]);

let encoded = bincode::encode_to_vec(&arr, bincode::config::legacy().skip_fixed_array_length()).unwrap();
assert_eq!(encoded.as_slice(), &[
    // no length
    10, 20, // First Foo
    30, 40, // Second Foo
]);
```

