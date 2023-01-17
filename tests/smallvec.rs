#![cfg(feature = "smallvec")]

use smallvec::{smallvec, SmallVec};

#[test]
fn test_smallvec() {
    let start: SmallVec<[u32; 8]> = smallvec![5, 6, 7, 8];

    let data = bincode::encode_to_vec(&start, bincode::config::standard()).unwrap();

    let decoded_unspilled: SmallVec<[u32; 8]> =
        bincode::decode_from_slice(&data, bincode::config::standard())
            .unwrap()
            .0;
    assert_eq!(start, decoded_unspilled);

    // SmallVec backing array size doesn't actually matter
    let decoded_spilled: SmallVec<[u32; 2]> =
        bincode::decode_from_slice(&data, bincode::config::standard())
            .unwrap()
            .0;
    assert_eq!(start, decoded_spilled);

    // And, in fact, we can even decode as a Vec
    let decoded_vec: Vec<u32> = bincode::decode_from_slice(&data, bincode::config::standard())
        .unwrap()
        .0;
    assert_eq!(start.as_slice(), decoded_vec.as_slice());
}
