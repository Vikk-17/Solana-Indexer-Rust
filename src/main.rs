use std::time::Duration;

use anyhow::Result;
use dotenvy::dotenv;

mod rpc_client;
use self::rpc_client::RpcConfig;
use rpc_client::RpcClientRepo;
use solana_commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::pubkey;
use solana_sdk::signer::Signer;
// use solana_client::nonblocking::rpc_client::RpcClient;

#[tokio::main]
async fn main() -> Result<()> {
    // load the .env file
    dotenv().ok();

    let rpc_url =
        std::env::var("RPC_URL").map_err(|_| anyhow::anyhow!("RPC_URL env vars is not set"));

    let config = RpcConfig {
        url: rpc_url?,
        commitment_config: CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        },
        timeout: Duration::from_secs(30),
    };
    // println!("RPC URL: {:?}", rpc_url);
    // let client = RpcClient::new(rpc_url);

    let client = RpcClientRepo::new(config).await?;
    let keypair = RpcClientRepo::keypair();
    let my_pubkey = keypair.pubkey();
    //airdrop
    let amount_lamports = RpcClientRepo::sol_to_lamports(1.0)?;
    let signature = client.airdrop(&my_pubkey, amount_lamports).await?;
    println!("airdrop signature: {}", signature);
    Ok(())
}
