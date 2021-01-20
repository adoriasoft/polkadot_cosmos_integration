use sp_std::vec::Vec;

pub mod sha256_transform;
pub mod ripemd160_transform;

pub enum PubKeyTypes {
    Ed25519,
    Secp256k1
}

pub fn get_address_from_pub_key(pub_key: &[u8], key_type: PubKeyTypes) -> Vec<u8> {
    match key_type {
        PubKeyTypes::Ed25519 => {
            let sha_digest = &sha256_transform::get_sha_value(pub_key)[0..20];
            sha_digest.to_vec().clone()
        },
        PubKeyTypes::Secp256k1 => {
            let sha_digest = sha256_transform::get_sha_value(pub_key);
            let ripemd160_digest = &ripemd160_transform::get_ripemd160_value(&sha_digest);
            ripemd160_digest.clone()
        },
    }
}
