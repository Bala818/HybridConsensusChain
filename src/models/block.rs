use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub validator: Option<String>, // Validator for PoS
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        Self {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            validator: None,
        }
    }
}
