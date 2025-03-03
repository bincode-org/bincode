// Credits to Sway in the Rust Programming Language

use rand::Rng;
use serde::{Deserialize, Serialize};

#[test]
pub fn test() {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        crate::test_same(random(&mut rng));
    }
}

fn random(rng: &mut impl Rng) -> FTXresponse<Trade> {
    if rng.gen() {
        FTXresponse::Result(FTXresponseSuccess {
            result: Trade::random(rng),
            success: rng.gen(),
        })
    } else {
        FTXresponse::Error(FTXresponseFailure {
            success: rng.gen(),
            error: crate::gen_string(rng),
        })
    }
}

#[derive(bincode_2::Encode, bincode_2::Decode, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[bincode(crate = "bincode_2")]
pub enum FTXresponse<T> {
    Result(FTXresponseSuccess<T>),
    Error(FTXresponseFailure),
}

#[derive(
    bincode_2::Encode, bincode_2::Decode, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq,
)]
#[bincode(crate = "bincode_2")]
pub struct FTXresponseSuccess<T> {
    pub success: bool,
    pub result: T,
}

#[derive(bincode_2::Encode, bincode_2::Decode, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[bincode(crate = "bincode_2")]
pub struct FTXresponseFailure {
    pub success: bool,
    pub error: String,
}

#[derive(bincode_2::Encode, bincode_2::Decode, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[bincode(crate = "bincode_2")]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(bincode_2::Encode, bincode_2::Decode, Serialize, Deserialize, Debug, PartialEq)]
#[bincode(crate = "bincode_2")]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub liquidation: bool,
    pub price: f64,
    pub side: TradeSide,
    pub size: f64,
    pub time: String,
}

impl Trade {
    fn random(rng: &mut impl Rng) -> Self {
        Self {
            id: rng.gen(),
            liquidation: rng.gen(),
            price: rng.gen(),
            side: if rng.gen() {
                TradeSide::Buy
            } else {
                TradeSide::Sell
            },
            size: rng.gen(),
            time: crate::gen_string(rng),
        }
    }
}
