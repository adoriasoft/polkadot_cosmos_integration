use codec::{Decode, Encode};
use sp_std::vec::Vec;

pub mod sha_hasher;
pub mod ripe_hasher;

/// Curves for convert nodes pub keys.
#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub enum PubKeyTypes {
    Ed25519 = 0,
    Secp256k1 = 1,
}

/// Method for getting address from pub key.
pub fn get_address_from_pub_key(pub_key: &[u8], key_type: PubKeyTypes) -> Vec<u8> {
    match key_type {
        PubKeyTypes::Ed25519 => {
            let sha_digest = &sha_hasher::get_sha256_hash(pub_key)[0..20];
            sha_digest.to_vec()
        }
        PubKeyTypes::Secp256k1 => {
            let sha_digest = sha_hasher::get_sha256_hash(pub_key);
            let ripemd160_digest = &ripe_hasher::get_ripemd160_hash(&sha_digest);
            ripemd160_digest.clone()
        }
    }
}

/// Method for encode value from base64 string to utf8 string.
pub fn encode_value_from_base64(value: &[u8]) -> Vec<u8> {
    base64::decode(value).unwrap().to_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_get_address_from_ed25519_pub_key() {
        const PUB_KEY: &str = "4MQ5aiE4zs1IqkLU3C0vWHUYhZcg40AX4k/wlsgcLCY=";
        const ADDRESS: &str = "1AD4A13D8239FB9C6917DF0C52DACE3DC3D9C046";

        let pub_key = base64::decode(PUB_KEY).unwrap();
        let address = crate::crypto_transform::get_address_from_pub_key(
            &pub_key,
            crate::crypto_transform::PubKeyTypes::Ed25519,
        );
        let address_str = hex::encode(address).to_ascii_uppercase();

        assert_eq!(address_str, ADDRESS);
    }

    #[test]
    fn should_get_address_from_secp256k1_pub_key() {
        const ADDRESS: &str = "2jLQaRKYMH6Sukx97FZRPqn494Ar";
        const PUB_KEY: &str = "02950e1cdfcb133d6024109fd489f734eeb4502418e538c28481f22bce276f248c";

        let pub_key = hex::decode(PUB_KEY).unwrap();
        let address = crate::crypto_transform::get_address_from_pub_key(
            &pub_key,
            crate::crypto_transform::PubKeyTypes::Secp256k1,
        );
        let address_str = bs58::encode(address).into_string();

        assert_eq!(address_str, ADDRESS);
    }
}
