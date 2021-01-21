use pallet_cosmos_abci;

#[test]
fn should_get_address_from_pub_key() {
    // Encoded pub_key `4MQ5aiE4zs1IqkLU3C0vWHUYhZcg40AX4k/wlsgcLCY=` from base64.
    let pub_key: Vec<u8> = vec![
        224, 196, 57, 106, 33, 56, 206, 205, 72, 170, 66, 212, 220, 45, 47, 88, 117, 24, 133, 151,
        32, 227, 64, 23, 226, 79, 240, 150, 200, 28, 44, 38,
    ];
    const ADDRESS: &str = "1AD4A13D8239FB9C6917DF0C52DACE3DC3D9C046";
    let address = pallet_cosmos_abci::crypto_transform::get_address_from_pub_key(
        &pub_key,
        pallet_cosmos_abci::crypto_transform::PubKeyTypes::Ed25519,
    );
    let address_str = hex::encode(address).to_ascii_uppercase();
    assert_eq!(address_str, ADDRESS);
}
