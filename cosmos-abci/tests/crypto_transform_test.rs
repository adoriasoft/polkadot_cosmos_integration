use pallet_cosmos_abci;

#[test]
fn should_get_address_from_pub_key() {
    // const PUB_KEY: &str = "4MQ5aiE4zs1IqkLU3C0vWHUYhZcg40AX4k/wlsgcLCY=";
    let pub_key: Vec<u8> = vec![218, 207, 110, 5, 107, 190, 254, 185, 51, 63, 53, 174, 193, 160, 164, 197, 7, 175, 196, 206, 23, 85, 46, 4, 9, 252, 114, 207, 126, 114, 139, 240];
    const ADDRESS: &str = "1AD4A13D8239FB9C6917DF0C52DACE3DC3D9C046";
    // let pub_key_bytes = base64::decode(PUB_KEY).unwrap();
    let address = pallet_cosmos_abci::crypto_transform::get_address_from_pub_key(&pub_key, pallet_cosmos_abci::crypto_transform::PubKeyTypes::Ed25519);
    let address_str = hex::encode(address);
    assert_eq!(address_str, ADDRESS);
}
