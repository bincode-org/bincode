#![cfg(all(feature = "serde", feature = "alloc", feature = "derive"))]

extern crate alloc;

use alloc::string::String;
use bincode::config::Configuration;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode)]
#[serde(crate = "serde_incl")]
pub struct SerdeRoundtrip {
    pub a: u32,
    #[serde(skip)]
    pub b: u32,
}

#[test]
fn test_serde_round_trip() {
    // validate serde attribute working
    let json = serde_json::to_string(&SerdeRoundtrip { a: 5, b: 5 }).unwrap();
    assert_eq!("{\"a\":5}", json);

    let result: SerdeRoundtrip = serde_json::from_str(&json).unwrap();
    assert_eq!(result.a, 5);
    assert_eq!(result.b, 0);

    // validate bincode working
    let bytes =
        bincode::encode_to_vec(SerdeRoundtrip { a: 15, b: 15 }, Configuration::standard()).unwrap();
    assert_eq!(bytes, &[15, 15]);
    let result: SerdeRoundtrip =
        bincode::decode_from_slice(&bytes, Configuration::standard()).unwrap();
    assert_eq!(result.a, 15);
    assert_eq!(result.b, 15);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "serde_incl")]
pub struct SerdeWithBorrowedData<'a> {
    pub a: u32,
    #[serde(skip)]
    pub b: u32,
    pub str: &'a str,
}

#[test]
fn test_serialize_deserialize_borrowed_data() {
    let input = SerdeWithBorrowedData {
        a: 5,
        b: 5,
        str: "Hello world",
    };

    #[rustfmt::skip]
    let expected = &[
        5, // a
        // b is skipped
        11, // str length
        b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd' // str
    ];

    let mut result = [0u8; 20];
    let len =
        bincode::serde_encode_to_slice(&input, &mut result, Configuration::standard()).unwrap();
    let result = &result[..len];
    assert_eq!(result, expected);

    let result = bincode::serde_encode_to_vec(&input, Configuration::standard()).unwrap();

    assert_eq!(result, expected);

    let output: SerdeWithBorrowedData =
        bincode::serde_decode_borrowed_from_slice(&result, Configuration::standard()).unwrap();
    assert_eq!(
        SerdeWithBorrowedData {
            b: 0, // remember: b is skipped
            ..input
        },
        output
    );
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "serde_incl")]
pub struct SerdeWithOwnedData {
    pub a: u32,
    #[serde(skip)]
    pub b: u32,
    pub str: String,
}

#[test]
fn test_serialize_deserialize_owned_data() {
    let input = SerdeWithOwnedData {
        a: 5,
        b: 5,
        str: String::from("Hello world"),
    };

    #[rustfmt::skip]
    let expected = &[
        5, // a
        // b is skipped
        11, // str length
        b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd' // str
    ];

    let mut result = [0u8; 20];
    let len =
        bincode::serde_encode_to_slice(&input, &mut result, Configuration::standard()).unwrap();
    let result = &result[..len];
    assert_eq!(result, expected);

    let result = bincode::serde_encode_to_vec(&input, Configuration::standard()).unwrap();

    assert_eq!(result, expected);

    let output: SerdeWithOwnedData =
        bincode::serde_decode_from_slice(&result, Configuration::standard()).unwrap();
    assert_eq!(
        SerdeWithOwnedData {
            b: 0, // remember: b is skipped
            ..input
        },
        output
    );
}
