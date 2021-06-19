extern crate bincode;

use core::fmt::Debug;

fn the_same<V>(element: V)
where
    V: bincode::enc::Encodeable + bincode::de::Decodable + PartialEq + Debug + Clone + 'static,
{
    let mut buffer = [0u8; 32];
    bincode::encode_into_slice(element.clone(), &mut buffer).unwrap();
    let decoded: V = bincode::decode(&mut buffer).unwrap();

    assert_eq!(element, decoded);
}

#[test]
fn test_numbers() {
    the_same(5u32);
}
