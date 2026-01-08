#[derive(Debug)]
pub struct TransactionsIndex {
    pub signatures: Vec<String>,
    // pub slot: u64,
    // pub block_time: Option<i64>, // block time (optional)
    pub status: String,
    pub fees: u64,
    pub log_message: Vec<String>,
    pub account_keys: Vec<String>,
}

#[derive(Debug)]
pub struct InstructionsIndex {
    // pub tx_signature: String,
    // pub slot: u64,
    // pub instruction_index: u32,
    // pub is_inner: bool,
    pub program_id: u8,
    pub accounts: Vec<u8>,
    // pub data: String,
}
