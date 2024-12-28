use crate::models::block::Block;
use rand::seq::SliceRandom;

pub fn select_validator(stakeholders: &Vec<(String, u64)>) -> String {
    let total_stake: u64 = stakeholders.iter().map(|(_, stake)| stake).sum();
    let mut rng = rand::thread_rng();

    stakeholders
        .choose_weighted(&mut rng, |&(_, stake)| stake as f64 / total_stake as f64)
        .expect("Failed to select a validator")
        .0
        .clone()
}
pub fn create_block_with_validator(mut block: Block, validator: String) -> Block {
    block.validator = Some(validator);
    block.hash = format!("pos_hash_{}", block.timestamp);
    block
}
