use sha2::{Digest, Sha256};

pub fn calculate_hash(
    index: u64,
    timestamp: u128,
    data: &str,
    previous_hash: &str,
    nonce: u64,
) -> String {
    let mut hasher = Sha256::new();

    // Update the hash with each component of the block
    hasher.update(index.to_string());
    hasher.update(timestamp.to_string());
    hasher.update(data);
    hasher.update(previous_hash);
    hasher.update(nonce.to_string());

    format!("{:x}", hasher.finalize())
}
