mod libs {
    pub mod kv {
        tonic::include_proto!("tendermint.libs.kv");
    }
}

pub mod crypto {
    pub mod merkle {
        tonic::include_proto!("tendermint.crypto.merkle");
    }
}

mod proto {
    pub mod abci_proto {
        tonic::include_proto!("tendermint.abci.types");
    }
}

pub use crypto::merkle::*;
pub use libs::kv::*;
pub use proto::abci_proto::*;

impl crate::ResponseSetOption for ResponseSetOption {
    fn get_code(&self) -> u32 {
        self.code
    }

    fn get_log(&self) -> String {
        self.log.clone()
    }

    fn get_info(&self) -> String {
        self.info.clone()
    }
}

impl crate::ResponseInfo for ResponseInfo {
    fn get_version(&self) -> String {
        self.version.clone()
    }

    fn get_app_version(&self) -> u64 {
        self.app_version
    }

    fn get_data(&self) -> String {
        self.data.clone()
    }

    fn get_last_block_app_hash(&self) -> Vec<u8> {
        self.last_block_app_hash.clone()
    }

    fn get_last_block_height(&self) -> i64 {
        self.last_block_height
    }
}

impl crate::ResponseFlush for ResponseFlush {}

impl crate::ResponseEcho for ResponseEcho {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn set_message(&mut self, v: String) {
        self.message = v;
    }
}

impl crate::ResponseCheckTx for ResponseCheckTx {
    fn get_code(&self) -> u32 {
        self.code
    }
    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    fn get_log(&self) -> String {
        self.log.clone()
    }
    fn get_info(&self) -> String {
        self.info.clone()
    }
    fn get_gas_wanted(&self) -> i64 {
        self.gas_wanted
    }
    fn get_gas_used(&self) -> i64 {
        self.gas_used
    }
    fn get_codespace(&self) -> String {
        self.codespace.clone()
    }

    fn set_code(&mut self, v: u32) {
        self.code = v
    }
    fn set_data(&mut self, v: Vec<u8>) {
        self.data = v
    }
    fn set_log(&mut self, v: String) {
        self.log = v
    }
    fn set_info(&mut self, v: String) {
        self.info = v
    }
    fn set_gas_wanted(&mut self, v: i64) {
        self.gas_wanted = v
    }
    fn set_gas_used(&mut self, v: i64) {
        self.gas_used = v
    }
    fn set_codespace(&mut self, v: String) {
        self.codespace = v
    }
}

impl crate::ResponseDeliverTx for ResponseDeliverTx {
    fn get_code(&self) -> u32 {
        self.code
    }
    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    fn get_log(&self) -> String {
        self.log.clone()
    }
    fn get_info(&self) -> String {
        self.info.clone()
    }
    fn get_gas_wanted(&self) -> i64 {
        self.gas_wanted
    }
    fn get_gas_used(&self) -> i64 {
        self.gas_used
    }
    fn get_codespace(&self) -> String {
        self.codespace.clone()
    }

    fn set_code(&mut self, v: u32) {
        self.code = v
    }
    fn set_data(&mut self, v: Vec<u8>) {
        self.data = v
    }
    fn set_log(&mut self, v: String) {
        self.log = v
    }
    fn set_info(&mut self, v: String) {
        self.info = v
    }
    fn set_gas_wanted(&mut self, v: i64) {
        self.gas_wanted = v
    }
    fn set_gas_used(&mut self, v: i64) {
        self.gas_used = v
    }
    fn set_codespace(&mut self, v: String) {
        self.codespace = v
    }
}

impl crate::ResponseInitChain for ResponseInitChain {}

impl crate::ResponseBeginBlock for ResponseBeginBlock {}

impl crate::ResponseEndBlock for ResponseEndBlock {
    fn get_validator_updates(&self) -> Vec<ValidatorUpdate> {
        self.validator_updates.clone()
    }
    fn get_events(&self) -> Vec<Event> {
        self.events.clone()
    }
    fn set_events(&mut self, events: Vec<Event>) {
        self.events = events;
    }
    fn set_validator_updates(&mut self, validator_updates: Vec<ValidatorUpdate>) {
        self.validator_updates = validator_updates;
    }
}

impl crate::ResponseCommit for ResponseCommit {
    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    fn get_retain_height(&self) -> i64 {
        self.retain_height
    }

    fn set_data(&mut self, v: Vec<u8>) {
        self.data = v
    }
    fn set_retain_height(&mut self, v: i64) {
        self.retain_height = v
    }
}

impl crate::ResponseQuery for ResponseQuery {
    fn get_code(&self) -> u32 {
        self.code
    }
    fn get_log(&self) -> String {
        self.log.clone()
    }
    fn get_info(&self) -> String {
        self.info.clone()
    }
    fn get_index(&self) -> i64 {
        self.index
    }
    fn get_key(&self) -> Vec<u8> {
        self.key.clone()
    }
    fn get_value(&self) -> Vec<u8> {
        self.value.clone()
    }
    fn get_height(&self) -> i64 {
        self.height
    }
    fn get_codespace(&self) -> String {
        self.codespace.clone()
    }
    fn get_proof(&self) -> Option<crypto::merkle::Proof> {
        self.proof.clone()
    }

    fn set_code(&mut self, v: u32) {
        self.code = v
    }
    fn set_log(&mut self, v: String) {
        self.log = v
    }
    fn set_info(&mut self, v: String) {
        self.info = v
    }
    fn set_index(&mut self, v: i64) {
        self.index = v
    }
    fn set_key(&mut self, v: Vec<u8>) {
        self.key = v
    }
    fn set_value(&mut self, v: Vec<u8>) {
        self.value = v
    }
    fn set_height(&mut self, v: i64) {
        self.height = v
    }
    fn set_codespace(&mut self, v: String) {
        self.codespace = v
    }
}

#[test]
fn abci_pub_key_serde() {
    let pub_key = PubKey {
        data: vec![10, 12, 15],
        r#type: "ed25519".to_string(),
    };
    let bytes_from_pub_key = bincode::serialize(&pub_key).unwrap();
    println!("{:?}", bytes_from_pub_key);
    let pub_key_deserialized: Result<PubKey, Box<bincode::ErrorKind>> =
        bincode::deserialize(&bytes_from_pub_key);
    assert_eq!(pub_key, pub_key_deserialized.unwrap());
}

#[test]
fn validation_update_serde() {
    let pub_key = PubKey {
        data: vec![10, 12, 15],
        r#type: "ed25519".to_string(),
    };
    let validator_update = ValidatorUpdate {
        pub_key: Some(pub_key),
        power: 14515,
    };
    let bytes_from_pub_key = bincode::serialize(&validator_update).unwrap();
    println!("{:?}", bytes_from_pub_key);
    let validator_update_deserialized: Result<ValidatorUpdate, Box<bincode::ErrorKind>> =
        bincode::deserialize(&bytes_from_pub_key);
    assert_eq!(validator_update, validator_update_deserialized.unwrap());
}
