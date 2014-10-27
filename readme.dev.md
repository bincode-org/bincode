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

^code(./examples/basic.rs)
