// Simplified case, taken from:
// https://github.com/rust-random/rand/blob/19404d68764ed08513131f82157e2ccad69dcf83/rand_pcg/src/pcg64.rs#L37-L40
// Original license: MIT OR Apache-2.0

use rand::Rng;

#[derive(
    Debug, bincode_2::Encode, bincode_2::Decode, serde::Serialize, serde::Deserialize, PartialEq, Eq,
)]
#[bincode(crate = "bincode_2")]
pub struct Lcg64Xsh32 {
    state: u64,
    increment: u64,
}

#[test]
pub fn test() {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        crate::test_same(Lcg64Xsh32 {
            state: rng.gen(),
            increment: rng.gen(),
        });
    }
}
