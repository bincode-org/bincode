#![cfg(all(feature = "std", feature = "derive"))]

extern crate std;

use bincode::{config::Configuration, Decode, Encode};
use std::borrow::Cow;
use std::string::String;

#[derive(Encode, Decode, PartialEq, Debug)]
struct T<'a, A: Clone + Encode + Decode> {
    t: Cow<'a, U<'a, A>>,
}

#[derive(Clone, Encode, Decode, PartialEq, Debug)]
struct U<'a, A: Clone + Encode + Decode> {
    u: Cow<'a, A>,
}

#[test]
fn test() {
    let u = U {
        u: Cow::Owned(String::from("Hello world")),
    };
    let t = T {
        t: Cow::Borrowed(&u),
    };
    let vec = bincode::encode_to_vec(&t, Configuration::standard()).unwrap();

    let (decoded, len): (T<String>, usize) =
        bincode::decode_from_slice(&vec, Configuration::standard()).unwrap();

    assert_eq!(t, decoded);
    assert_eq!(len, 12);
}
