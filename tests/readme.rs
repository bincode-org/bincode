#![cfg(all(feature = "derive", feature = "alloc"))]

// Examples from readme.md to make sure they're valid bincode

use bincode::{config::Configuration, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct World(Vec<Entity>);

#[test]
fn readme_1() {
    let config = Configuration::standard();

    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = bincode::encode_to_vec(&world, config).unwrap();

    // The length of the vector is encoded as a varint u64, which in this case gets collapsed to a single byte
    // See the documentation on varint for more info for that.
    // The 4 floats are encoded in 4 bytes each.
    assert_eq!(encoded.len(), 1 + 4 * 4);

    let decoded: World = bincode::decode_from_slice(&encoded[..], config).unwrap();

    assert_eq!(world, decoded);
}
