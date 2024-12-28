use sha2::{Digest, Sha256};

/// Calculates the hash of a block based on its content.
///
/// # Parameters:
/// - `index`: The position of the block in the blockchain.
/// - `timestamp`: The timestamp when the block was created (in milliseconds since UNIX epoch).
/// - `data`: The payload or content stored in the block.
/// - `previous_hash`: The hash of the previous block in the blockchain.
/// - `nonce`: The nonce value used for Proof-of-Work.
///
/// # Returns:
/// - A hexadecimal string representing the hash of the block.
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

    // Finalize and return the hash as a hexadecimal string
    format!("{:x}", hasher.finalize())
}
