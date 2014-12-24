extern crate bincode;
extern crate "rustc-serialize" as rustc_serialize;

#[deriving(RustcEncodable, RustcDecodable, PartialEq)]
struct Entity {
    x: f32,
    y: f32,
}

#[deriving(RustcEncodable, RustcDecodable, PartialEq)]
struct World {
    entities: Vec<Entity>
}

fn main() {
    let world = World {
        entities: vec![Entity {x: 0.0, y: 4.0}, Entity {x: 10.0, y: 20.5}]
    };

    let encoded: Vec<u8> = bincode::encode(&world).unwrap();
    // 8 bytes for the length of the vector, 4 bytes per float.
    assert_eq!(encoded.len(), 8 + 4 * 4);
    let decoded: World = bincode::decode(encoded).unwrap();

    assert!(world == decoded);
}
