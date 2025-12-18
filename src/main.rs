use indexer::db::model::{InstructionsIndex, TransactionsIndex};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_commitment_config::CommitmentConfig;
use solana_transaction_status_client_types::{
    EncodedTransaction, TransactionDetails, UiMessage, UiTransactionEncoding,
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

    if let Some(transactions) = &block.transactions {
        for txn in transactions.iter() {
            // Decalare the vars in each iteration
            let mut all_signatures: Vec<String> = Vec::new();
            let mut all_fees: Vec<u64> = Vec::new();
            let mut all_acc_keys: Vec<String> = Vec::new();
            let mut all_logs: Vec<String> = Vec::new();
            let mut status: String = String::new();
            let mut program_id_index: u8 = 0;
            let mut accounts: Vec<u8> = Vec::new();

            // Fetch all signatures
            if let EncodedTransaction::Json(inner_txn) = &txn.transaction {
                all_signatures.extend(inner_txn.signatures.clone());
                if let UiMessage::Raw(msg) = &inner_txn.message {
                    all_acc_keys.extend(msg.account_keys.clone());
                    program_id_index = msg.instructions[0].program_id_index;
                    accounts = msg.instructions[0].accounts.clone();
                }
            }

            // Fetch fees
            if let Some(meta) = &txn.meta {
                all_fees.push(meta.fee);
                all_logs.extend(meta.log_messages.clone().unwrap());
                // fetch the status and map to Ok if success
                if let Ok(()) = &meta.status {
                    status = String::from("Ok");
                }
            }

            // Populate the transaction struct to store into
            let txn_set = TransactionsIndex {
                signatures: all_signatures.clone(),
                status,
                fees: all_fees.clone(),
                log_message: all_logs.clone(),
                account_keys: all_acc_keys.clone(),
            };
            // Populate the Instruction struct to store into
            let ins_set = InstructionsIndex {
                program_id_index,
                accounts,
            };
            // push it to the db
            println!("{:#?}", txn_set);
            println!("{:#?}", ins_set);
        }
    }

    Ok(())
}
