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
use serde::{de::{Visitor, Error, MapAccess}, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

impl crate::ResponseInitChain for ResponseInitChain {
    fn get_validators(&self) -> Vec<ValidatorUpdate> {
        self.validators.clone()
    }
}

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

impl crate::ResponseBeginBlock for ResponseBeginBlock {}

impl Serialize for PubKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PubKey", 2)?;
        state.serialize_field("r#type", &self.r#type)?;
        state.serialize_field("data", &self.data)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for PubKey {
    fn deserialize<D>(deserialize: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Data, Type }
    
        struct PubKeyVisitor;

        impl<'de> Visitor<'de> for PubKeyVisitor {
            type Value = PubKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PubKey")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PubKey, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut r_type = None;
                let mut data = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Data => {
                            data = Some(map.next_value()?);
                        }
                        Field::Type => {
                            r_type = Some(map.next_value()?);
                        }
                    }
                }
                let r_type = r_type.ok_or_else(|| Error::missing_field("type"))?;
                let data = data.ok_or_else(|| Error::missing_field("data"))?;
                Ok(PubKey { r#type: r_type, data })
            }
        }

        deserialize.deserialize_struct("PubKey", &["type", "data"], PubKeyVisitor)
    }
}

impl Serialize for ValidatorUpdate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ValidatorUpdate", 2)?;
        state.serialize_field("pub_key", &self.pub_key)?;
        state.serialize_field("power", &self.power)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ValidatorUpdate {
    fn deserialize<D>(deserialize: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValidatorUpdateVisitor;

        impl<'de> Visitor<'de> for ValidatorUpdateVisitor {
            type Value = ValidatorUpdate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PubKey")
            }
        }

        deserialize.deserialize_struct(
            "ValidatorUpdate",
            &["pub_key", "power"],
            ValidatorUpdateVisitor,
        )
    }
}

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
