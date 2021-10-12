#![cfg(all(feature = "serde", feature = "alloc", feature = "derive"))]

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, bincode::Encodable, bincode::Decodable)]
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
        bincode::encode_to_vec(SerdeRoundtrip { a: 15, b: 15 }).unwrap();
    assert_eq!(bytes, &[15, 15]);
    let result: SerdeRoundtrip = bincode::decode(&bytes).unwrap();
    assert_eq!(result.a, 15);
    assert_eq!(result.b, 15);
}
