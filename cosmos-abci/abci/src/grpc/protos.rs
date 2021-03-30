pub mod tendermint {
    pub mod crypto {
        tonic::include_proto!("tendermint.crypto");
    }

    pub mod types {
        tonic::include_proto!("tendermint.types");
    }

    pub mod version {
        tonic::include_proto!("tendermint.version");
    }

    pub mod abci {
        tonic::include_proto!("tendermint.abci");
    }
}

impl crate::ResponseSetOption for tendermint::abci::ResponseSetOption {
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

impl crate::ResponseInfo for tendermint::abci::ResponseInfo {
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

impl crate::ResponseFlush for tendermint::abci::ResponseFlush {}

impl crate::ResponseEcho for tendermint::abci::ResponseEcho {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn set_message(&mut self, v: String) {
        self.message = v;
    }
}

impl crate::ResponseCheckTx for tendermint::abci::ResponseCheckTx {
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

impl crate::ResponseDeliverTx for tendermint::abci::ResponseDeliverTx {
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

impl crate::ResponseInitChain for tendermint::abci::ResponseInitChain {
    fn get_validators(&self) -> Vec<tendermint::abci::ValidatorUpdate> {
        self.validators.clone()
    }
}

impl crate::ResponseBeginBlock for tendermint::abci::ResponseBeginBlock {}

impl crate::ResponseEndBlock for tendermint::abci::ResponseEndBlock {
    fn get_validator_updates(&self) -> Vec<tendermint::abci::ValidatorUpdate> {
        self.validator_updates.clone()
    }
    fn get_events(&self) -> Vec<tendermint::abci::Event> {
        self.events.clone()
    }
    fn set_events(&mut self, events: Vec<tendermint::abci::Event>) {
        self.events = events;
    }
    fn set_validator_updates(&mut self, validator_updates: Vec<tendermint::abci::ValidatorUpdate>) {
        self.validator_updates = validator_updates;
    }
}

impl crate::ResponseCommit for tendermint::abci::ResponseCommit {
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

impl crate::ResponseQuery for tendermint::abci::ResponseQuery {
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
    fn get_proof(&self) -> Option<tendermint::crypto::ProofOps> {
        self.proof_ops.clone()
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
