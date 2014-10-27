# Binary Encoder / Decoder

[![Build Status](https://travis-ci.org/TyOverby/bincode.svg)](https://travis-ci.org/TyOverby/bincode)

A compact encoder / decoder pair that uses an binary zero-fluff encoding scheme.
The size of the encoded object will be the same or smaller than the size that
the object takes up in memory in a running Rust program.

In addition to exposing two simple funcitons that encode to Vec<u8> and decode
from Vec<u8>, binary-encode exposes a Reader/Writer API that makes it work
perfectly with other stream-based apis such as rust files, network streams,
and the [flate2-rs](https://github.com/alexcrichton/flate2-rs) compression
library.

[Api Documentation](http://tyoverby.github.io/bincode/bincode/)

## Example

```rust
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

```
