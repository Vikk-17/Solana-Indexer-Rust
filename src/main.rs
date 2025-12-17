mod model;

use model::TransactionsIndex;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_commitment_config::CommitmentConfig;
use solana_transaction_status_client_types::{
    EncodedTransaction, TransactionDetails, UiMessage, UiTransactionEncoding
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("https://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed(),
    );

    // let slots: u64 = client.get_slot_with_commitment(CommitmentConfig::confirmed()).await?;
    // println!("Slots: {:#?}", slots);
    let slot_number = 377261149;

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

    // Vector to store all the signatures, fees
    // let mut all_signatures: Vec<String> = Vec::new();
    // let mut all_fees: Vec<u64> = Vec::new();
    let mut all_acc_keys: Vec<String> = Vec::new();
    let mut all_logs: Vec<String> = Vec::new();
    if let Some(transactions) = &block.transactions {
        for txn in transactions.iter() {
            // Fetch all signatures
            // if let EncodedTransaction::Json(inner_txn) = &txn.transaction {
            //     // all_signatures.extend(inner_txn.signatures.clone());
            //     // println!("{:#?}", inner_txn.message);
            //     if let UiMessage::Raw(msg) = &inner_txn.message {
            //         all_acc_keys.extend(msg.account_keys.clone());
            //     }
            // }

            // Fetch fees
            if let Some(meta) = &txn.meta {
                // all_fees.push(meta.fee);
                // all_logs.extend(meta.log_messages.clone());
                if let Some(logs) = &meta.log_messages {
                    for log in logs.iter {
                        all_logs.extend(log.clone());
                    }
                }
            }

        }
    }
    // let txn_ix = TransactionsIndex{
    //     signatures: all_signatures,
    //     fees: all_fees,
    // };

    // println!("{:#?}", txn_ix);

    // for (i, logs) in all_logs.iter().enumerate() {
    //     println!("{}: {}", i+1, logs);
    // }

    Ok(())
}
