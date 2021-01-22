use codec::{Decode, Encode};
use sp_std::vec::Vec;

pub mod hashers;

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub enum PubKeyTypes {
    Ed25519 = 0,
    Secp256k1 = 1,
}

pub fn get_address_from_pub_key(pub_key: &[u8], key_type: PubKeyTypes) -> Vec<u8> {
    match key_type {
        PubKeyTypes::Ed25519 => {
            let sha_digest = &hashers::get_sha_hash(pub_key)[0..20];
            sha_digest.to_vec()
        }
        PubKeyTypes::Secp256k1 => {
            let sha_digest = hashers::get_sha_hash(pub_key);
            let ripemd160_digest = &hashers::get_ripemd160_hash(&sha_digest);
            ripemd160_digest.clone()
        }
    }
}

pub fn encode_value_from_base64(value: &[u8]) -> Vec<u8> {
    base64::decode(value).unwrap().to_vec()
}

#[test]
fn should_get_address_from_ed25519_pub_key() {
    const PUB_KEY: &str = "4MQ5aiE4zs1IqkLU3C0vWHUYhZcg40AX4k/wlsgcLCY=";
    const ADDRESS: &str = "1AD4A13D8239FB9C6917DF0C52DACE3DC3D9C046";

    let pub_key = base64::decode(PUB_KEY).unwrap();
    let address = get_address_from_pub_key(
        &pub_key,
        PubKeyTypes::Ed25519,
    );
    let address_str = hex::encode(address).to_ascii_uppercase();

    assert_eq!(address_str, ADDRESS);
}

#[test]
fn should_get_address_from_secp256k1_pub_key() {
    const PUB_KEY: &str = "4MQ5aiE4zs1IqkLU3C0vWHUYhZcg40AX4k/wlsgcLCY=";
    const ADDRESS: &str = "QJBGWofcbE8e+PX5+PADgdxzOW8=";

    let pub_key = base64::decode(PUB_KEY).unwrap();
    let address = get_address_from_pub_key(
        &pub_key,
        PubKeyTypes::Secp256k1,
    );
    let address_str = base64::encode(address);

    assert_eq!(address_str, ADDRESS);
}
