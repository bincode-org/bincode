use bincode::{de::Decodable, enc::Encodeable};

#[derive(bincode::Encodable, PartialEq, Debug)]
pub struct Test<T: Encodeable> {
    a: T,
    b: u32,
    c: u8,
}

#[derive(bincode::Decodable, PartialEq, Debug, Eq)]
pub struct Test2<T: Decodable> {
    a: T,
    b: u32,
    c: u32,
}

#[derive(bincode::Encodable, bincode::Decodable, PartialEq, Debug, Eq)]
pub struct TestTupleStruct(u32, u32, u32);

#[test]
fn test_encodable() {
    let start = Test {
        a: 5i32,
        b: 10u32,
        c: 20u8,
    };
    let mut slice = [0u8; 1024];
    let bytes_written = bincode::encode_into_slice(start, &mut slice).unwrap();
    assert_eq!(bytes_written, 3);
    assert_eq!(&slice[..bytes_written], &[10, 10, 20]);
}

#[test]
fn test_decodable() {
    let start = Test2 {
        a: 5u32,
        b: 10u32,
        c: 1024u32,
    };
    let mut slice = [5, 10, 251, 0, 4];
    let result: Test2<u32> = bincode::decode(&mut slice).unwrap();
    assert_eq!(result, start);
}

#[test]
fn test_encodable_tuple() {
    let start = TestTupleStruct(5, 10, 1024);
    let mut slice = [0u8; 1024];
    let bytes_written = bincode::encode_into_slice(start, &mut slice).unwrap();
    assert_eq!(bytes_written, 5);
    assert_eq!(&slice[..bytes_written], &[5, 10, 251, 0, 4]);
}

#[test]
fn test_decodable_tuple() {
    let start = TestTupleStruct(5, 10, 1024);
    let mut slice = [5, 10, 251, 0, 4];
    let result: TestTupleStruct = bincode::decode(&mut slice).unwrap();
    assert_eq!(result, start);
}
