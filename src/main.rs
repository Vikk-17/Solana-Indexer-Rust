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
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Json),
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(false),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    let block = client.get_block_with_config(slot_number, config).await?;
    // println!("{:#?}", block);
    // let data = block.transactions.unwrap();
    //
    // for (i, _txns) in data.iter().enumerate(){
    //     println!("{:#?}", data[i].transaction);
    // }
    let mut all_signatures: Vec<String> = Vec::new();
    for txn in block.transactions.unwrap_or_default() {
        if let EncodedTransaction::Json(ui_tx) = txn.transaction {
            all_signatures.extend(ui_tx.signatures);
        }
    }
    // println!("Found {} signatures", all_signatures.len());
    // for (i, sig) in all_signatures.iter().enumerate() {
    //     println!("{}: {}", i + 1, sig);
    // }
    Ok(())
}
