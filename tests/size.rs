use bincode::{size::MaxSize, get_max_value};

#[test]
fn test_sanity() {
    assert_eq!(u8::ENCODED_MAX_SIZE, 1);
}

#[test]
fn test_array() {
    type Array = [u16; 50];
    assert_eq!(Array::ENCODED_MAX_SIZE, 100);
}

#[test]
fn test_tuple() {
    type Tuple = (u8, i8, u128, [u16; 50], [u32; 25]);
    assert_eq!(Tuple::ENCODED_MAX_SIZE, 218);
}

#[test]
fn test_get_max_value() {
    assert_eq!(get_max_value([3, 5, 6, 7, u8::ENCODED_MAX_SIZE]), 7)
}
