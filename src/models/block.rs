use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a blockchain block with metadata and data payload.
///
/// # Fields:
/// - `index`: The position of the block in the chain.
/// - `timestamp`: The time when the block was created (in milliseconds since UNIX epoch).
/// - `data`: The payload or content stored in the block.
/// - `previous_hash`: The hash of the previous block in the chain.
/// - `hash`: The unique hash of this block, calculated based on its content.
/// - `nonce`: The nonce value for Proof-of-Work.
/// - `validator`: The ID of the validator for Proof-of-Stake (optional).
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
    /// Creates a new block with the given index, data, and previous hash.
    ///
    /// # Parameters:
    /// - `index`: The position of the block in the blockchain.
    /// - `data`: The data to be stored in the block.
    /// - `previous_hash`: The hash of the previous block in the blockchain.
    ///
    /// # Returns:
    /// - A new block instance with a default hash, nonce, and no validator.
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
