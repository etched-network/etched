use std::str::FromStr;

use ethers::prelude::*;

use crate::etched_payload::{self, EtchedPayload};
use serde::{Deserialize, Serialize};

pub struct EtchedFetcher {
    batch_address: H160,
    next_block: u64,
}

// Fetches etched transactions from the blockchain
impl EtchedFetcher {

    pub fn new(start_block: u64) -> Self {
        Self {
            next_block: start_block,
            batch_address: H160::from_str("0xfF00000000000000000000000000000000000020")
                .expect("Failed to parse address"),
        }
    }

    pub async fn fetch_next_block(&mut self) -> Result<Vec<EtchedPayload>, Box<dyn std::error::Error>> {
        // Initialize the provider
        let provider = Provider::<Http>::try_from("http://127.0.0.1:8546").unwrap();
    
        // Attempt to fetch the next block with transactions
        let block = match provider.get_block_with_txs(self.next_block).await {
            Ok(Some(block)) => block,
            _ => return Ok(vec![]), // Return an empty vector on any failure or absence of a block
        };
    
        // Process transactions
        let transactions: Vec<EtchedPayload> = block.transactions.into_iter().filter_map(|tx| {
            match tx.to {
                Some(address) if address == self.batch_address => {
                    println!("etched: {:?}", address);
                    if let Ok(json_data) = String::from_utf8(tx.input.clone().to_vec()) {
                        match serde_json::from_str::<EtchedPayload>(&json_data) {
                            Ok(payload) => {
                                println!("result: {:?}", payload);
                                return Some(payload);
                            },
                            Err(_) => None, // Handle JSON parse errors
                        }
                    } else {
                        None // Handle UTF-8 conversion errors
                    }
                },
                _ => None // Handle cases where `tx.to` is None or doesn't match `self.batch_address`
            }
        }).collect();
    
        // Update next block number and print current block number
        println!("Block Number: {:?}", block.number);
        self.next_block += 1;
        Ok(transactions)
    }

}