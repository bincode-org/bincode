extern crate bincode;

use bincode::config::{self, Config};
use core::fmt::Debug;

fn the_same_with_config<V, C>(element: V, config: C)
where
    V: bincode::enc::Encodeable + bincode::de::Decodable + PartialEq + Debug + Clone + 'static,
    C: Config,
{
    let mut buffer = [0u8; 32];
    bincode::encode_into_slice_with_config(element.clone(), &mut buffer, config).unwrap();
    let decoded: V = bincode::decode_with_config(&mut buffer, config).unwrap();

    assert_eq!(element, decoded);
}
fn the_same<V>(element: V)
where
    V: bincode::enc::Encodeable + bincode::de::Decodable + PartialEq + Debug + Clone + 'static,
{
    the_same_with_config(
        element.clone(),
        config::Default
            .with_little_endian()
            .with_fixed_int_encoding(),
    );
    the_same_with_config(
        element.clone(),
        config::Default.with_big_endian().with_fixed_int_encoding(),
    );
    the_same_with_config(
        element.clone(),
        config::Default
            .with_little_endian()
            .with_variable_int_encoding(),
    );
    the_same_with_config(
        element,
        config::Default
            .with_big_endian()
            .with_variable_int_encoding(),
    );
}

#[test]
fn test_numbers() {
    the_same(5u8);
    the_same(5u16);
    the_same(5u32);
    the_same(5u64);
    the_same(5u128);
    the_same(5usize);

    the_same(5i8);
    the_same(5i16);
    the_same(5i32);
    the_same(5i64);
    the_same(5i128);
    the_same(5isize);

    the_same(5.0f32);
    the_same(5.0f64);
}
