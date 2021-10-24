#![cfg(feature = "derive")]

use bincode::config::Configuration;
use bincode::{de::Decode, enc::Encode};

#[derive(bincode::Encode, PartialEq, Debug)]
pub(crate) struct Test<T: Encode> {
    a: T,
    b: u32,
    c: u8,
}

#[derive(bincode::Decode, PartialEq, Debug, Eq)]
pub struct Test2<T: Decode> {
    a: T,
    b: u32,
    c: u32,
}

#[derive(bincode::Decode, PartialEq, Debug, Eq)]
pub struct Test3<'a> {
    a: &'a str,
    b: u32,
    c: u32,
}

#[derive(bincode::Encode, bincode::Decode, PartialEq, Debug, Eq)]
pub struct TestTupleStruct(u32, u32, u32);

#[derive(bincode::Encode, bincode::Decode, PartialEq, Debug, Eq)]
pub enum TestEnum {
    Foo,
    Bar { name: u32 },
    Baz(u32, u32, u32),
}

#[derive(bincode::Encode, bincode::Decode, PartialEq, Debug, Eq)]
pub enum TestEnum2<'a> {
    Foo,
    Bar { name: &'a str },
    Baz(u32, u32, u32),
}

#[test]
fn test_encode() {
    let start = Test {
        a: 5i32,
        b: 10u32,
        c: 20u8,
    };
    let mut slice = [0u8; 1024];
    let bytes_written =
        bincode::encode_into_slice(start, &mut slice, Configuration::standard()).unwrap();
    assert_eq!(bytes_written, 3);
    assert_eq!(&slice[..bytes_written], &[10, 10, 20]);
}

#[cfg(feature = "std")]
#[test]
fn test_decode() {
    let start = Test2 {
        a: 5u32,
        b: 10u32,
        c: 1024u32,
    };
    let slice = [5, 10, 251, 0, 4];
    let result: Test2<u32> =
        bincode::decode_from_std_read(&mut slice.as_ref(), Configuration::standard()).unwrap();
    assert_eq!(result, start);
}

#[test]
fn test_encode_tuple() {
    let start = TestTupleStruct(5, 10, 1024);
    let mut slice = [0u8; 1024];
    let bytes_written =
        bincode::encode_into_slice(start, &mut slice, Configuration::standard()).unwrap();
    assert_eq!(bytes_written, 5);
    assert_eq!(&slice[..bytes_written], &[5, 10, 251, 0, 4]);
}

#[test]
fn test_decode_tuple() {
    let start = TestTupleStruct(5, 10, 1024);
    let mut slice = [5, 10, 251, 0, 4];
    let result: TestTupleStruct =
        bincode::decode_from_slice(&mut slice, Configuration::standard()).unwrap();
    assert_eq!(result, start);
}

#[test]
fn test_encode_enum_struct_variant() {
    let start = TestEnum::Bar { name: 5u32 };
    let mut slice = [0u8; 1024];
    let bytes_written =
        bincode::encode_into_slice(start, &mut slice, Configuration::standard()).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(&slice[..bytes_written], &[1, 5]);
}

#[test]
fn test_decode_enum_struct_variant() {
    let start = TestEnum::Bar { name: 5u32 };
    let mut slice = [1, 5];
    let result: TestEnum =
        bincode::decode_from_slice(&mut slice, Configuration::standard()).unwrap();
    assert_eq!(result, start);
}

#[test]
fn test_encode_enum_tuple_variant() {
    let start = TestEnum::Baz(5, 10, 1024);
    let mut slice = [0u8; 1024];
    let bytes_written =
        bincode::encode_into_slice(start, &mut slice, Configuration::standard()).unwrap();
    assert_eq!(bytes_written, 6);
    assert_eq!(&slice[..bytes_written], &[2, 5, 10, 251, 0, 4]);
}

#[test]
fn test_decode_enum_unit_variant() {
    let start = TestEnum::Foo;
    let mut slice = [0];
    let result: TestEnum =
        bincode::decode_from_slice(&mut slice, Configuration::standard()).unwrap();
    assert_eq!(result, start);
}

#[test]
fn test_encode_enum_unit_variant() {
    let start = TestEnum::Foo;
    let mut slice = [0u8; 1024];
    let bytes_written =
        bincode::encode_into_slice(start, &mut slice, Configuration::standard()).unwrap();
    assert_eq!(bytes_written, 1);
    assert_eq!(&slice[..bytes_written], &[0]);
}

#[test]
fn test_decode_enum_tuple_variant() {
    let start = TestEnum::Baz(5, 10, 1024);
    let mut slice = [2, 5, 10, 251, 0, 4];
    let result: TestEnum =
        bincode::decode_from_slice(&mut slice, Configuration::standard()).unwrap();
    assert_eq!(result, start);
}
