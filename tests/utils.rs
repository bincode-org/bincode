use bincode::config::{self, Config};
use core::fmt::Debug;

fn the_same_with_config<V, C>(element: V, config: C)
where
    V: bincode::enc::Encodeable
        + for<'de> bincode::de::Decodable
        + PartialEq
        + Debug
        + Clone
        + 'static,
    C: Config,
{
    let mut buffer = [0u8; 1024];
    let len = bincode::encode_into_slice_with_config(element.clone(), &mut buffer, config).unwrap();
    println!(
        "{:?}: {:?} ({:?})",
        element,
        &buffer[..len],
        core::any::type_name::<C>()
    );
    let decoded: V = bincode::decode_with_config(&mut buffer, config).unwrap();

    assert_eq!(element, decoded);
}

pub fn the_same<V>(element: V)
where
    V: bincode::enc::Encodeable
        + for<'de> bincode::de::Decodable
        + PartialEq
        + Debug
        + Clone
        + 'static,
{
    // A matrix of each different config option possible
    the_same_with_config(
        element.clone(),
        config::Default
            .with_little_endian()
            .with_fixed_int_encoding()
            .skip_fixed_array_length(),
    );
    the_same_with_config(
        element.clone(),
        config::Default
            .with_big_endian()
            .with_fixed_int_encoding()
            .skip_fixed_array_length(),
    );
    the_same_with_config(
        element.clone(),
        config::Default
            .with_little_endian()
            .with_variable_int_encoding()
            .skip_fixed_array_length(),
    );
    the_same_with_config(
        element.clone(),
        config::Default
            .with_big_endian()
            .with_variable_int_encoding()
            .skip_fixed_array_length(),
    );
    the_same_with_config(
        element.clone(),
        config::Default
            .with_little_endian()
            .with_fixed_int_encoding()
            .write_fixed_array_length(),
    );
    the_same_with_config(
        element.clone(),
        config::Default
            .with_big_endian()
            .with_fixed_int_encoding()
            .write_fixed_array_length(),
    );
    the_same_with_config(
        element.clone(),
        config::Default
            .with_little_endian()
            .with_variable_int_encoding()
            .write_fixed_array_length(),
    );
    the_same_with_config(
        element,
        config::Default
            .with_big_endian()
            .with_variable_int_encoding()
            .write_fixed_array_length(),
    );
}
