use sha2::*;
use sp_std::vec::Vec;

/// Method for generate sha256 hash from value.
pub fn get_sha256_hash(from: &[u8]) -> Vec<u8> {
    let mut digest = sha2::Sha256::new();
    digest.update(from);
    let value = digest.finalize();
    value.clone().to_vec()
}
