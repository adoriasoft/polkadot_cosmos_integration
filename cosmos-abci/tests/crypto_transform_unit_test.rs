use pallet_cosmos_abci::*;

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
        const PUB_KEY: &str = "02950e1cdfcb133d6024109fd489f734eeb4502418e538c28481f22bce276f248c";
        const ADDRESS: &str = "7c2bb42a8be69791ec763e51f5a49bcd41e82237";

        let pub_key = hex::decode(PUB_KEY).unwrap();
        let address = crate::crypto_transform::get_address_from_pub_key(
            &pub_key,
            crate::crypto_transform::PubKeyTypes::Secp256k1,
        );
        let address_str = hex::encode(address);

        assert_eq!(address_str, ADDRESS);
    }
}
