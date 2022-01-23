#![cfg(test)]

use bincode_1::Options;

mod rand;
mod sway;

pub fn test_same_with_config<T, C, O>(t: &T, bincode_1_options: O, bincode_2_config: C)
where
    T: bincode_2::Encode + serde::Serialize + core::fmt::Debug,
    C: bincode_2::config::Config + Clone,
    O: bincode_1::Options + Clone,
{
    let bincode_1_output = bincode_1_options.serialize(t).unwrap();
    let bincode_2_output = bincode_2::encode_to_vec(t, bincode_2_config).unwrap();

    assert_eq!(
        bincode_1_output, bincode_2_output,
        "{:?} serializes differently",
        t
    );
}

pub fn test_same<T>(t: T)
where
    T: bincode_2::Encode + serde::Serialize + core::fmt::Debug,
{
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
