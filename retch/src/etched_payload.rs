// Add serde and serde_json to your Cargo.toml dependencies
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

extern crate serde;
extern crate serde_json;

use ethers::{abi::Address, types::H160};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// Define custom types and implement Deserialize where necessary
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EtchedPayload {
    pub fn_: u64,
    pub input: String,
}

// You may want to handle Hex as a special case, depending on its specific encoding
#[derive(Serialize, Deserialize, Debug)]
struct Hex(String);

impl FromStr for Hex {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hex(s.to_owned()))
    }
}
