use codec::{Decode, Encode};
use sp_std::vec::Vec;

/// The hashers module that exports `ripemd160` and `sha256` hash algorithms.
pub mod hashers;

/// Pub key curve types.
#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub enum PubKeyTypes {
    Ed25519 = 0,
    Secp256k1 = 1,
}

/// Get authority address from authority pub key.
pub fn get_address_from_pub_key(pub_key: &[u8], key_type: PubKeyTypes) -> Vec<u8> {
    match key_type {
        PubKeyTypes::Ed25519 => {
            let sha_digest = &hashers::get_sha256_hash(pub_key)[0..20];
            sha_digest.to_vec()
        }
        PubKeyTypes::Secp256k1 => {
            let sha_digest = hashers::get_sha256_hash(pub_key);
            let ripemd160_digest = &hashers::get_ripemd160_hash(&sha_digest);
            ripemd160_digest.clone()
        }
    }
}

/// Encode value from `base64` string to `utf8` string.
pub fn encode_value_from_base64(value: &[u8]) -> Vec<u8> {
    base64::decode(value).unwrap().to_vec()
}
