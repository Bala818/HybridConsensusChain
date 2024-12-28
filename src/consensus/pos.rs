use crate::models::block::Block;
use rand::seq::SliceRandom;

/// Selects a validator for the Proof-of-Stake mechanism.
///
/// # Parameters:
/// - `stakeholders`: A vector of tuples where each tuple contains a stakeholder's ID and their stake.
///
/// # Returns:
/// - The ID of the selected validator.
pub fn select_validator(stakeholders: &Vec<(String, u64)>) -> String {
    let total_stake: u64 = stakeholders.iter().map(|(_, stake)| stake).sum();
    let mut rng = rand::thread_rng();

    stakeholders
        .choose_weighted(&mut rng, |&(_, stake)| stake as f64 / total_stake as f64)
        .expect("Failed to select a validator") // Ensure a validator is always selected
        .0
        .clone()
}

/// Creates a block with the selected validator for Proof-of-Stake.
///
/// # Parameters:
/// - `block`: The block to be validated.
/// - `validator`: The ID of the selected validator.
///
/// # Returns:
/// - A validated block with the validator's ID and a simulated hash.
pub fn create_block_with_validator(mut block: Block, validator: String) -> Block {
    block.validator = Some(validator);
    block.hash = format!("pos_hash_{}", block.timestamp); // Simulate hashing for simplicity
    block
}
