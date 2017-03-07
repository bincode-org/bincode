#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{serialize, deserialize, SizeLimit};

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct World(Vec<Entity>);

fn main() {
    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = serialize(&world, SizeLimit::Infinite).unwrap();
    println!("Encoded: {:?} ({} bytes)", encoded, encoded.len());

    let decoded: World = deserialize(&encoded[..]).unwrap();
    println!("Decoded: {:?}", decoded);
}
