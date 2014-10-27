extern crate bincode;
extern crate serialize;

#[deriving(Encodable, Decodable, PartialEq)]
struct Entity {
    x: f32,
    y: f32,
}

#[deriving(Encodable, Decodable, PartialEq)]
struct World {
    entities: Vec<Entity>
}

fn main() {
    let world = World {
        entities: vec![Entity {x: 0.0, y: 4.0}, Entity {x: 10.0, y: 20.5}]
    };

    let encoded: Vec<u8> = bincode::encode(&world).unwrap();
    let decoded: World = bincode::decode(encoded).unwrap();

    assert!(world == decoded);
}
