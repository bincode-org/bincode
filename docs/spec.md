# Serialization specification

*NOTE*: Serialization is done by `bincode_derive` by default. If you enable the `serde` flag, serialization is done by `serde-derive` instead. `serde-derive` has the same guarantees as `bincode_derive` for now.

Related issue: https://github.com/serde-rs/serde/issues/1756#issuecomment-689682123

## Basic types

Boolean types are encoded with 1 byte for each boolean type, with `0` being `false`, `1` being true. Whilst deserilizing every other value will throw an error.

All basic numeric types will be encoded based on the configured [IntEncoding](#IntEncoding).

All floating point types will take up exactly 4 (for `f32`) or 8 (for `f64`) bytes.

All tuples have no additional bytes, and are encoded in their specified order, e.g.
```rs
let tuple = (u32::min_value(), i32::max_value()); // 8 bytes
let encoded = bincode::encode_to_vec_with_options(&tuple, Options::default().with_fixint_encoding()).unwrap();
assert_eq!(encoded.as_slice(), &[
    0,   0,   0,   0,  // 4 bytes for first type:  u32
    255, 255, 255, 255 // 4 bytes for second type: i32
]);
```

## IntEncoding
Bincode currently supports 2 different types of `IntEncoding`:

### VarintEncoding
Encoding an unsigned integer v (of any type excepting u8) works as follows:

1. If `u < 251`, encode it as a single byte with that value.
1. If `251 <= u < 2**16`, encode it as a literal byte 251, followed by a u16 with value `u`.
1. If `2**16 <= u < 2**32`, encode it as a literal byte 252, followed by a u32 with value `u`.
1. If `2**32 <= u < 2**64`, encode it as a literal byte 253, followed by a u64 with value `u`.
1. If `2**64 <= u < 2**128`, encode it as a literal byte 254, followed by a u128 with value `u`.

See the documentation of [VarintEncoding](https://docs.rs/bincode/latest/bincode/config/struct.VarintEncoding.html) for more information.

### FixintEncoding

- Fixed size integers are encoded directly
- Enum discriminants are encoded as u32
- Lengths and usize are encoded as u64

See the documentation of [FixintEncoding](https://docs.rs/bincode/latest/bincode/config/struct.FixintEncoding.html) for more information.

## Enums

Enums are encoded with their variant first, followed by optionally the variant fields. The variant index is based on the `IntEncoding` during serilization.

Both named and unnamed fields are serialized with their values only, and therefor encode to the same value.

```rs
#[derive(bincode::Serialize)]
pub enum SomeEnum {
    A,
    B(u32),
    C { value: u32 },
}

// SomeEnum::A
let encoded = bincode::encode_to_vec_with_options(&SomeEnum::A, Options::default().with_fixint_encoding()).unwrap();
assert_eq!(encoded.as_slice(), &[
    0, 0, 0, 0, // first variant, A
    // no extra bytes because A has no fields
]);

// SomeEnum::B(0)
let encoded = bincode::encode_to_vec_with_options(&SomeEnum::B(0), Options::default().with_fixint_encoding()).unwrap();
assert_eq!(encoded.as_slice(), &[
    0, 0, 0, 1, // first variant, B
    0, 0, 0, 0  // B has 1 unnamed field, which is an u32, so 4 bytes
]);

// SomeEnum::C { value: 0u32 }
let encoded = bincode::encode_to_vec_with_options(&SomeEnum::C { value: 0u32 }, Options::default().with_fixint_encoding()).unwrap();
assert_eq!(encoded.as_slice(), &[
    0, 0, 0, 2, // first variant, C
    0, 0, 0, 0  // C has 1 named field which is a u32, so 4 bytes
]);
```

# Collections

Collections are encoded with their length value first, following by each entry of the collection. The length value is based on your `IntEncoding`.

```rs
let list = vec![
    0u8,
    1u8,
    2u8
];

let encoded = bincode::encode_to_vec_with_options(&list, Options::default().with_fixint_encoding()).unwrap();
assert_eq!(encoded.as_slice(), &[
    0, 0, 0, 0, 0, 0, 0, 3, // length of 3u64
    0, // entry 0
    1, // entry 1
    2, // entry 2
]);
```

This also applies to e.g. `HashMap`, where each entry is a [tuple](#Basic%20types) of the key and value.
