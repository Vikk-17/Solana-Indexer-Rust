use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_commitment_config::CommitmentConfig;
use solana_transaction_status_client_types::{
    EncodedTransaction, TransactionDetails, UiTransactionEncoding,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("https://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed(),
    );

    // let slots: u64 = client.get_slot_with_commitment(CommitmentConfig::confirmed()).await?;
    // println!("Slots: {:#?}", slots);
    let slot_number = 377261141;

    // === Fetch block at specific slot ===
    // Use Full to get complete transaction data with account balances
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Json),
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(false),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    let block = client.get_block_with_config(slot_number, config).await?;
    let mut all_signatures: Vec<String> = Vec::new();

    if let Some(transactions) = &block.transactions {
        for txn in transactions.iter() {
            if let EncodedTransaction::Json(in_txn) = &txn.transaction {
                all_signatures.extend(in_txn.signatures.clone());
            }
        }
    }

    Ok(())
}
