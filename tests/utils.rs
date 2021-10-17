use bincode::config::{self, Config};
use core::fmt::Debug;

fn the_same_with_config<V, C, CMP>(element: &V, config: C, cmp: CMP)
where
    V: bincode::enc::Encode + bincode::de::Decode + Debug + 'static,
    C: Config,
    CMP: Fn(&V, &V) -> bool,
{
    let mut buffer = [0u8; 1024];
    let len = bincode::encode_into_slice_with_config(&element, &mut buffer, config).unwrap();
    println!(
        "{:?}: {:?} ({:?})",
        element,
        &buffer[..len],
        core::any::type_name::<C>()
    );
    let decoded: V = bincode::decode_with_config(&mut buffer, config).unwrap();

    assert!(
        cmp(&element, &decoded),
        "Comparison failed\nDecoded:  {:?}\nExpected: {:?}\nBytes: {:?}",
        decoded,
        element,
        &buffer[..len],
    );
}

pub fn the_same_with_comparer<V, CMP>(element: V, cmp: CMP)
where
    V: bincode::enc::Encode + bincode::de::Decode + Debug + 'static,
    CMP: Fn(&V, &V) -> bool,
{
    // A matrix of each different config option possible
    the_same_with_config(
        &element,
        config::Configuration::new()
            .with_little_endian()
            .with_fixed_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        config::Configuration::new()
            .with_big_endian()
            .with_fixed_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        config::Configuration::new()
            .with_little_endian()
            .with_variable_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        config::Configuration::new()
            .with_big_endian()
            .with_variable_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        config::Configuration::new()
            .with_little_endian()
            .with_fixed_int_encoding()
            .write_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        config::Configuration::new()
            .with_big_endian()
            .with_fixed_int_encoding()
            .write_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        config::Configuration::new()
            .with_little_endian()
            .with_variable_int_encoding()
            .write_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        config::Configuration::new()
            .with_big_endian()
            .with_variable_int_encoding()
            .write_fixed_array_length(),
        &cmp,
    );
}

#[allow(dead_code)] // This is not used in every test
pub fn the_same<V>(element: V)
where
    V: bincode::enc::Encode + bincode::de::Decode + PartialEq + Debug + 'static,
{
    the_same_with_comparer(element, |a, b| a == b);
}
