#[derive(Debug)]
pub struct TransactionsIndex {
    pub signatures: Vec<String>,
    // pub slot: u64,
    // pub block_time: Option<i64>,
    pub status: String,
    pub fees: u64,
    pub log_message: Vec<String>,
    pub account_keys: Vec<String>,
}

#[derive(Debug)]
pub struct InstructionsIndex {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
}
