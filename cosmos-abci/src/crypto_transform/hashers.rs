use sha2::*;
use ripemd160;
use sp_std::vec::Vec;

pub fn get_sha_hash(from: &[u8]) -> Vec<u8> {
    let mut digest = sha2::Sha256::new();
    digest.update(from);
    let value = digest.finalize();
    value.clone().to_vec()
}

pub fn get_ripemd160_hash(from: &[u8]) -> Vec<u8> {
    let mut digest = ripemd160::Ripemd160::new();
    digest.update(from);
    let value = digest.finalize();
    value.clone().to_vec()
}
