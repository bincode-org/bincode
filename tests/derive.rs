#[derive(bincode::Encodable, PartialEq, Debug)]
pub struct Test {
    a: i32,
    b: u32,
    c: u8,
}

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
