#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{serialize, deserialize, DEFAULT_CONFIG};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = serialize(&world, DEFAULT_CONFIG).unwrap();

    // 8 bytes for the length of the vector (usize), 4 bytes per float.
    assert_eq!(encoded.len(), 8 + 4 * 4);

    let decoded: World = deserialize(&encoded[..], DEFAULT_CONFIG).unwrap();

    assert_eq!(world, decoded);
}
