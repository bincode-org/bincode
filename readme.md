# Bincode

<img align="right" src="./logo.png">

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
extern crate "rustc-serialize" as rustc_serialize;

use bincode::SizeLimit;

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct World {
    entities: Vec<Entity>
}

fn main() {
    let world = World {
        entities: vec![Entity {x: 0.0, y: 4.0}, Entity {x: 10.0, y: 20.5}]
    };

    let encoded: Vec<u8> = bincode::encode(&world, SizeLimit::Infinite).unwrap();

    // 8 bytes for the length of the vector, 4 bytes per float.
    assert_eq!(encoded.len(), 8 + 4 * 4);

    let decoded: World = bincode::decode(&encoded[]).unwrap();

    assert!(world == decoded);
}

```


## Details

The encoding (and thus decoding) proceeds unsurprisingly -- primitive
types are encoded according to the underlying `Writer`, tuples and
structs are encoded by encoding their fields one-by-one, and enums are
encoded by first writing out the tag representing the variant and
then the contents.

However, there are some implementation details to be aware of:

* `isize`/`usize` are encoded as `i64`/`u64`, for portability.
* enums variants are encoded as a `u32` instead that as a `uint`.
  `u32` is enough for all practical uses.
* `str` is encoded as `(u64, &[u8])`, where the `u64` is the number of
  bytes contained in the encoded string.
