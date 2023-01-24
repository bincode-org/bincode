#![cfg(all(feature = "serde", feature = "alloc", feature = "derive"))]

extern crate alloc;

use alloc::string::String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, bincode::EncodedSize)]
pub struct SerdeRoundtrip {
    pub a: u32,
    #[serde(skip)]
    pub b: u32,
    pub c: TupleS,
}

#[derive(
    Serialize, Deserialize, bincode::Encode, bincode::Decode, bincode::EncodedSize, PartialEq, Debug,
)]
pub struct TupleS(f32, f32, f32);

#[test]
fn test_serde_round_trip() {
    // validate serde attribute working
    let json = serde_json::to_string(&SerdeRoundtrip {
        a: 5,
        b: 5,
        c: TupleS(2.0, 3.0, 4.0),
    })
    .unwrap();
    assert_eq!("{\"a\":5,\"c\":[2.0,3.0,4.0]}", json);

    let result: SerdeRoundtrip = serde_json::from_str(&json).unwrap();
    assert_eq!(result.a, 5);
    assert_eq!(result.b, 0);

    // validate bincode working
    let start = SerdeRoundtrip {
        a: 15,
        b: 15,
        c: TupleS(2.0, 3.0, 4.0),
    };
    let encoded_size = bincode::serde::encoded_size(&start, bincode::config::standard()).unwrap();
    let bytes = bincode::serde::encode_to_vec(start, bincode::config::standard()).unwrap();
    assert_eq!(bytes.len(), encoded_size);
    assert_eq!(bytes, &[15, 0, 0, 0, 64, 0, 0, 64, 64, 0, 0, 128, 64]);

    let (result, len): (SerdeRoundtrip, usize) =
        bincode::serde::decode_from_slice(&bytes, bincode::config::standard()).unwrap();
    assert_eq!(result.a, 15);
    assert_eq!(result.b, 0); // remember: b is skipped
    assert_eq!(result.c, TupleS(2.0, 3.0, 4.0));
    assert_eq!(len, 13);
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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
    let encoded_size = bincode::serde::encoded_size(&input, bincode::config::standard()).unwrap();
    let len = bincode::serde::encode_into_slice(&input, &mut result, bincode::config::standard())
        .unwrap();
    assert_eq!(len, encoded_size);
    let result = &result[..len];
    assert_eq!(result, expected);

    let result = bincode::serde::encode_to_vec(&input, bincode::config::standard()).unwrap();

    assert_eq!(result, expected);

    let output: SerdeWithBorrowedData =
        bincode::serde::decode_borrowed_from_slice(&result, bincode::config::standard()).unwrap();
    assert_eq!(
        SerdeWithBorrowedData {
            b: 0, // remember: b is skipped
            ..input
        },
        output
    );
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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
    let encoded_size = bincode::serde::encoded_size(&input, bincode::config::standard()).unwrap();
    let len = bincode::serde::encode_into_slice(&input, &mut result, bincode::config::standard())
        .unwrap();
    assert_eq!(len, encoded_size);
    let result = &result[..len];
    assert_eq!(result, expected);

    let result = bincode::serde::encode_to_vec(&input, bincode::config::standard()).unwrap();

    assert_eq!(result, expected);

    let (output, len): (SerdeWithOwnedData, usize) =
        bincode::serde::decode_from_slice(&result, bincode::config::standard()).unwrap();
    assert_eq!(
        SerdeWithOwnedData {
            b: 0, // remember: b is skipped
            ..input
        },
        output
    );
    assert_eq!(len, 13);
}

#[cfg(feature = "derive")]
mod derive {
    use bincode::{Decode, Encode, EncodedSize};
    use serde_derive::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    pub struct SerdeType {
        pub a: u32,
    }

    #[derive(Decode, Encode, EncodedSize, PartialEq, Eq, Debug)]
    pub struct StructWithSerde {
        #[bincode(with_serde)]
        pub serde: SerdeType,
    }

    #[derive(Decode, Encode, EncodedSize, PartialEq, Eq, Debug)]
    pub enum EnumWithSerde {
        Unit(#[bincode(with_serde)] SerdeType),
        Struct {
            #[bincode(with_serde)]
            serde: SerdeType,
        },
    }

    #[test]
    fn test_serde_derive() {
        fn test_encode_decode<T>(start: T, expected_len: usize)
        where
            T: bincode::Encode
                + bincode::Decode
                + bincode::EncodedSize
                + PartialEq
                + core::fmt::Debug,
        {
            let mut slice = [0u8; 100];
            let encoded_size = bincode::encoded_size(&start, bincode::config::standard()).unwrap();
            let len = bincode::encode_into_slice(&start, &mut slice, bincode::config::standard())
                .unwrap();
            assert_eq!(len, expected_len);
            assert_eq!(len, encoded_size);
            let slice = &slice[..len];
            let (result, len): (T, usize) =
                bincode::decode_from_slice(slice, bincode::config::standard()).unwrap();

            assert_eq!(start, result);
            assert_eq!(len, expected_len);
        }
        test_encode_decode(
            StructWithSerde {
                serde: SerdeType { a: 5 },
            },
            1,
        );
        test_encode_decode(EnumWithSerde::Unit(SerdeType { a: 5 }), 2);
        test_encode_decode(
            EnumWithSerde::Struct {
                serde: SerdeType { a: 5 },
            },
            2,
        );
    }
}
