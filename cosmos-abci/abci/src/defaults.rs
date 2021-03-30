/// Method for getting gRPC url form active env.
pub fn get_server_url() -> String {
    crate::utils::get_option_from_node_args(crate::utils::NodeOptionVariables::AbciServerUrl)
        .unwrap_or_else(|| DEFAULT_ABCI_URL.to_owned())
}

pub fn get_storage_name() -> String {
    match std::env::var("ABCI_STORAGE_NAME") {
        Ok(val) => val,
        Err(_) => DEFAULT_ABCI_STORAGE_NAME.to_owned(),
    }
}

/// Default ABCI gRPC url.
pub const DEFAULT_ABCI_URL: &str = "tcp://localhost:26658";

/// Default ABCI storage name.
pub const DEFAULT_ABCI_STORAGE_NAME: &str = "abci_storage_rocksdb";

/// App version type.
pub type AppVersion = String;
/// App block version type.
pub type BlockVersion = u64;
/// App P2P version type.
pub type P2PVersion = u64;

/// VersionConfigs struct that represent app version confuration.
pub struct VersionConfigs {
    pub app_version: String,
    pub block_version: u64,
    pub p2p_version: u64,
}

/// Implementation for VersionConfigs struct.
impl VersionConfigs {
    fn log_info(&self) {
        println!("BlockVersion is {}", self.block_version);
        println!("AppVersion is {}", self.app_version);
        println!("P2PVersion is {}", self.p2p_version);
    }
}

/// Method for getting app version configs.
pub fn get_app_configs() -> VersionConfigs {
    let version_configs = VersionConfigs {
        app_version: "0.1.0".to_string(), // version specified at Cargo.toml of `abci` pallet.
        block_version: 0,
        p2p_version: 0,
    };
    version_configs.log_info();
    version_configs
}
