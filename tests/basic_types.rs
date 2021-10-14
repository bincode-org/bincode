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
    let mut buffer = [0u8; 32];
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
fn the_same<V>(element: V)
where
    V: bincode::enc::Encodeable
        + for<'de> bincode::de::Decodable
        + PartialEq
        + Debug
        + Clone
        + 'static,
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
    // integer types
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

    // bool
    the_same(true);
    the_same(false);

    // utf8 characters
    for char in "aÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö文".chars()
    {
        the_same(char);
    }

    // tuples, up to 8
    the_same((1u8,));
    the_same((1u8, 2u8));
    the_same((1u8, 2u8, 3u8));
    the_same((1u8, 2u8, 3u8, 4u8));
    the_same((1u8, 2u8, 3u8, 4u8, 5u8));
    the_same((1u8, 2u8, 3u8, 4u8, 5u8, 6u8));
    the_same((1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8));
    the_same((1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8));
}

#[test]
fn test_slice() {
    let mut buffer = [0u8; 32];
    let input: &[u8] = &[1, 2, 3, 4, 5, 6, 7];
    bincode::encode_into_slice(input, &mut buffer).unwrap();
    assert_eq!(&buffer[..8], &[7, 1, 2, 3, 4, 5, 6, 7]);

    let output: &[u8] = bincode::decode(&mut buffer[..8]).unwrap();
    assert_eq!(input, output);
}

#[test]
fn test_str() {
    let mut buffer = [0u8; 32];
    let input: &str = "Hello world";
    bincode::encode_into_slice(input, &mut buffer).unwrap();
    assert_eq!(
        &buffer[..12],
        &[11, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
    );

    let output: &str = bincode::decode(&mut buffer[..12]).unwrap();
    assert_eq!(input, output);
}

#[test]
fn test_array() {
    let mut buffer = [0u8; 32];
    let input: [u8; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    bincode::encode_into_slice(input, &mut buffer).unwrap();
    assert_eq!(&buffer[..10], &[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);

    let output: [u8; 10] = bincode::decode(&mut buffer[..10]).unwrap();
    assert_eq!(input, output);
}
