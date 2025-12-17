use solana_transaction_status_client_types::UiTransactionError;

#[derive(Debug)]
pub struct TransactionsIndex {
    pub signatures: Vec<String>,
    // pub status: Result<(), UiTransactionError>,
    pub fees: Vec<u64>,
    // pub log_message: Vec<String>,
    pub account_keys: Vec<String>,
}

#[derive(Debug)]
pub struct InstructionsIndex {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
}
