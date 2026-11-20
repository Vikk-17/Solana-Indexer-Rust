use anyhow::Result;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_sdk::{
    lamports,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
};
use std::sync::Arc;
use std::time::Duration;

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub url: String,
    pub commitment_config: CommitmentConfig,
    pub timeout: Duration,
}

#[derive(Clone)]
pub struct RpcClientRepo {
    pub config: RpcConfig,
    pub client: Arc<RpcClient>,
}

impl RpcClientRepo {
    pub async fn new(config: RpcConfig) -> Result<Self> {
        let client = RpcClient::new_with_timeout_and_commitment(
            config.url.clone(),
            config.timeout,
            config.commitment_config,
        );

        Ok(RpcClientRepo {
            config,
            client: Arc::new(client),
        })
    }

    pub fn keypair() -> Keypair {
        Keypair::new()
    }

    pub fn sol_to_lamports(sol: f64) -> Result<u64> {
        anyhow::ensure!(sol.is_finite() && sol > 0.0, "Invalid SOL amount: {sol}");
        Ok((sol * LAMPORTS_PER_SOL as f64).round() as u64)
    }

    pub async fn airdrop(&self, pub_key: &Pubkey, lamports: u64) -> Result<Signature> {
        let sig = self.client.request_airdrop(pub_key, lamports).await?;
        self.client
            .poll_for_signature_with_commitment(&sig, CommitmentConfig::confirmed())
            .await?;
        Ok(sig)
    }

    pub async fn check_balance(&self, public_key: &Pubkey) -> Result<f64> {
        Ok(self.client.get_balance(public_key).await? as f64)
    }
}
