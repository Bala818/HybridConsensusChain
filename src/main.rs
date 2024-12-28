mod consensus {
    pub mod pos;
    pub mod pow;
}
mod models {
    pub mod block;
}
mod utils;

use consensus::pos::{create_block_with_validator, select_validator};
use consensus::pow::mine_block;
use models::block::Block;

fn main() {
    let difficulty = 4; // PoW difficulty
    let mut blockchain: Vec<Block> = Vec::new();

    // Genesis Block
    let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
    blockchain.push(genesis_block);

    // Proof-of-Work
    let mut new_block = Block::new(
        1,
        "Transaction Data 1".to_string(),
        blockchain[0].hash.clone(),
    );
    let mined_block = mine_block(new_block, difficulty);
    println!("Mined Block: {:?}", mined_block);
    blockchain.push(mined_block);

    // Proof-of-Stake
    let stakeholders = vec![
        ("Alice".to_string(), 50),
        ("Bob".to_string(), 30),
        ("Charlie".to_string(), 20),
    ];
    let selected_validator = select_validator(&stakeholders);
    let mut pos_block = Block::new(
        2,
        "Transaction Data 2".to_string(),
        blockchain[1].hash.clone(),
    );
    let validated_block = create_block_with_validator(pos_block, selected_validator.clone());
    println!("Validated Block: {:?}", validated_block);
    blockchain.push(validated_block);

    // Final Blockchain State
    println!("Blockchain: {:?}", blockchain);
}
