// Credits to Sway in the Rust Programming Language

use serde::{Deserialize, Serialize};

#[derive(bincode_2::Encode, bincode_2::Decode, serde::Serialize, serde::Deserialize, Debug)]
#[bincode(crate = "bincode_2")]
pub struct FTXresponseSuccess<T> {
    pub success: bool,
    pub result: T,
}

#[derive(bincode_2::Encode, bincode_2::Decode, Serialize, Deserialize, Debug)]
#[bincode(crate = "bincode_2")]
pub struct FTXresponseFailure {
    pub success: bool,
    pub error: String,
}

#[derive(bincode_2::Encode, bincode_2::Decode, Serialize, Deserialize, Debug)]
#[bincode(crate = "bincode_2")]
#[serde(untagged)]
pub enum FTXresponse<T> {
    Result(FTXresponseSuccess<T>),
    Error(FTXresponseFailure),
}

#[derive(bincode_2::Encode, bincode_2::Decode, Serialize, Deserialize, Debug)]
#[bincode(crate = "bincode_2")]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(bincode_2::Encode, bincode_2::Decode, Serialize, Deserialize, Debug)]
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
