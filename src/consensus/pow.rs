use crate::models::block::Block;
use crate::utils::hash::calculate_hash;

/// Mines a block by solving the Proof-of-Work puzzle.
///
/// # Parameters:
/// - `block`: The block to be mined.
/// - `difficulty`: The number of leading zeroes required in the hash.
///
/// # Returns:
/// - A mined block with a valid hash.
pub fn mine_block(mut block: Block, difficulty: usize) -> Block {
    let target = "0".repeat(difficulty); // Target hash prefix
    while !block.hash.starts_with(&target) {
        block.nonce += 1; // Increment nonce to find a valid hash
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
