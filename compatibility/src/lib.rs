#![cfg(test)]

use ::rand::Rng;
use bincode_1::Options;

mod rand;
mod sway;

pub fn test_same_with_config<T, C, O>(t: &T, bincode_1_options: O, bincode_2_config: C)
where
    T: bincode_2::Encode
        + bincode_2::Decode
        + serde::Serialize
        + serde::de::DeserializeOwned
        + core::fmt::Debug
        + PartialEq,
    C: bincode_2::config::Config,
    O: bincode_1::Options + Copy,
{
    let bincode_1_output = bincode_1_options.serialize(t).unwrap();
    let bincode_2_output = bincode_2::encode_to_vec(t, bincode_2_config).unwrap();

    assert_eq!(
        bincode_1_output, bincode_2_output,
        "{:?} serializes differently",
        t
    );

    let decoded: T = bincode_1_options.deserialize(&bincode_1_output).unwrap();
    assert_eq!(&decoded, t);
    let decoded: T = bincode_1_options.deserialize(&bincode_2_output).unwrap();
    assert_eq!(&decoded, t);

    let decoded: T = bincode_2::decode_from_slice(&bincode_1_output, bincode_2_config)
        .unwrap()
        .0;
    assert_eq!(&decoded, t);
    let decoded: T = bincode_2::decode_from_slice(&bincode_2_output, bincode_2_config)
        .unwrap()
        .0;
    assert_eq!(&decoded, t);
}

pub fn test_same<T>(t: T)
where
    T: bincode_2::Encode
        + bincode_2::Decode
        + serde::Serialize
        + serde::de::DeserializeOwned
        + core::fmt::Debug
        + PartialEq,
{
    test_same_with_config(
        &t,
        bincode_1::options().with_fixint_encoding(),
        bincode_2::config::legacy(),
    );

    test_same_with_config(
        &t,
        bincode_1::options()
            .with_big_endian()
            .with_varint_encoding(),
        bincode_2::config::legacy()
            .with_big_endian()
            .with_variable_int_encoding(),
    );
    test_same_with_config(
        &t,
        bincode_1::options()
            .with_little_endian()
            .with_varint_encoding(),
        bincode_2::config::legacy()
            .with_little_endian()
            .with_variable_int_encoding(),
    );
    test_same_with_config(
        &t,
        bincode_1::options()
            .with_big_endian()
            .with_fixint_encoding(),
        bincode_2::config::legacy()
            .with_big_endian()
            .with_fixed_int_encoding(),
    );
    test_same_with_config(
        &t,
        bincode_1::options()
            .with_little_endian()
            .with_fixint_encoding(),
        bincode_2::config::legacy()
            .with_little_endian()
            .with_fixed_int_encoding(),
    );
}

pub fn gen_string(rng: &mut impl Rng) -> String {
    let len = rng.gen_range(0..100usize);
    let mut result = String::with_capacity(len * 4);
    for _ in 0..len {
        result.push(rng.gen_range('\0'..char::MAX));
    }
    result
}
