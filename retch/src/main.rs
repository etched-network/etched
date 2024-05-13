//! This example shows how to run a custom dev node programmatically and submit a transaction
//! through rpc.

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use ethers::{types::{TransactionRequest, Address}};
use futures_util::StreamExt;
use reth::{
    builder::{NodeBuilder, NodeHandle},
    providers::CanonStateSubscriptions,
    rpc::eth::EthTransactions,
    tasks::TaskManager,
};
use reth_node_core::{args::RpcServerArgs, node_config::NodeConfig};
use reth_node_ethereum::EthereumNode;
use reth_primitives::{b256, hex, Bytes, ChainSpec, Genesis};
use tokio::time::{sleep, Duration};
use std::{sync::Arc};
mod fetcher;
mod etched_payload;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let tasks = TaskManager::current();

    // create node config
    let node_config = NodeConfig::test()
        .dev()
        .with_rpc(RpcServerArgs::default().with_http())
        .with_chain(custom_chain());

    let NodeHandle { mut node, node_exit_future: _ } = NodeBuilder::new(node_config)
        .testing_node(tasks.executor())
        .node(EthereumNode::default())
        .launch()
        .await?;

    // let mut notifications = node.provider.canonical_state_stream();
    let start_block: u64 = 19846551;
    let mut fetcher: fetcher::EtchedFetcher = fetcher::EtchedFetcher::new(start_block);
    let eth_api = node.rpc_registry.eth_api();

    println!("node started - polling for ");
    loop {
        match fetcher.fetch_next_block().await { // Use the instance to call next_block
            Ok(etched_transactions) => {
                for tx in etched_transactions {
                    let raw_bytes = hex::decode(tx.input).unwrap();
                    let raw_tx = Bytes::from(raw_bytes);
                    let err = eth_api.send_raw_transaction(raw_tx).await;
                    if let Err(e) = err {
                        println!("Error submitting transaction: {}", e);
                    }
                }
            },
            Err(e) => println!("Error fetching block: {}", e),
        }
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

fn custom_chain() -> Arc<ChainSpec> {
    let custom_genesis = r#"
{
    "nonce": "0x42",
    "timestamp": "0x0",
    "extraData": "0x5343",
    "gasLimit": "0x1388",
    "difficulty": "0x400000000",
    "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "coinbase": "0x0000000000000000000000000000000000000000",
    "alloc": {
        "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266": {
            "balance": "0x4a47e3c12448f4ad000000"
        }
    },
    "number": "0x0",
    "gasUsed": "0x0",
    "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "config": {
        "ethash": {},
        "chainId": 2727,
        "homesteadBlock": 0,
        "eip150Block": 0,
        "eip155Block": 0,
        "eip158Block": 0,
        "byzantiumBlock": 0,
        "constantinopleBlock": 0,
        "petersburgBlock": 0,
        "istanbulBlock": 0,
        "berlinBlock": 0,
        "londonBlock": 0,
        "terminalTotalDifficulty": 0,
        "terminalTotalDifficultyPassed": true,
        "shanghaiTime": 0
    }
}
"#;
    let genesis: Genesis = serde_json::from_str(custom_genesis).unwrap();
    Arc::new(genesis.into())
}
