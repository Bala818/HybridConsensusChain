use crate::models::block::Block;
use crate::utils::hash::calculate_hash;

pub fn mine_block(mut block: Block, difficulty: usize) -> Block {
    let target = "0".repeat(difficulty);
    while !block.hash.starts_with(&target) {
        block.nonce += 1;
        block.hash = calculate_hash(
            block.index,
            block.timestamp,
            &block.data,
            &block.previous_hash,
            block.nonce,
        );
    }
    block
}
