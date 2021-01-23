use ripemd160::*;
use sp_std::vec::Vec;

/// Method for generate ripemd160 hash from value.
pub fn get_ripemd160_hash(from: &[u8]) -> Vec<u8> {
    let mut digest = ripemd160::Ripemd160::new();
    digest.update(from);
    let value = digest.finalize();
    value.clone().to_vec()
}
