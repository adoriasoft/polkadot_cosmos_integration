use chrono::DateTime;
use std::fs;

struct ArgIds {
    server_url_temp_id: usize,
    genesis_state_path_temp_id: usize,
    rpc_url_temp_id: usize,
}

pub enum NodeOptionVariables {
    AbciServerUrl,
    AbciGenesisStatePath,
    AbciRPCUrl,
}

pub struct GenesisInfo {
    pub time_seconds: i64,
    pub time_nanos: i32,
    pub chain_id: String,
    pub pub_key_types: Vec<String>,
    pub max_bytes: i64,
    pub max_gas: i64,
    pub max_age_num_blocks: i64,
    pub max_age_duration: u64,
    pub app_state_bytes: Vec<u8>,
}

pub fn serialize_vec<T: serde::Serialize>(
    validators: Vec<T>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(bincode::serialize(&validators).map_err(|_| "cannot serialize")?)
}

pub fn deserialize_vec<'a, T: serde::Deserialize<'a>>(
    bytes: &'a [u8],
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    Ok(bincode::deserialize(bytes).map_err(|_| "cannot deserialize")?)
}

fn get_genesis_from_file() -> Result<String, Box<dyn std::error::Error>> {
    let mut path: String = "/fake/path/to/genesis".to_owned();

    if let Some(_path) = get_option_from_node_args(NodeOptionVariables::AbciGenesisStatePath) {
        path = _path;
    }

    let app_state = fs::read_to_string(&path).map_err(|_| "Error opening app state file")?;
    Ok(app_state)
}

pub fn get_abci_genesis() -> String {
    match get_genesis_from_file() {
        Ok(v) => v,
        _ => std::env::var("ABCI_GENESIS_STATE")
            .map_err(|_| "Failed to get abci genesis state file")
            .unwrap(),
    }
}

pub fn parse_cosmos_genesis_file(genesis: &str) -> Result<GenesisInfo, Box<dyn std::error::Error>> {
    let genesis: serde_json::Value = serde_json::from_str(genesis).map_err(|e| e.to_string())?;
    let chain_id = genesis["chain_id"]
        .as_str()
        .ok_or_else(|| "chain_id not found".to_owned())?;
    let genesis_time = genesis["genesis_time"]
        .as_str()
        .ok_or_else(|| "chain_id not found".to_owned())?;
    let pub_key_types: Vec<String> = genesis["consensus_params"]["validator"]["pub_key_types"]
        .as_array()
        .ok_or_else(|| "pub_keys_types not found".to_owned())?
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect();
    let max_bytes = genesis["consensus_params"]["block"]["max_bytes"]
        .as_str()
        .ok_or_else(|| "chain_id not found".to_owned())?
        .parse::<i64>()?;
    let max_gas = genesis["consensus_params"]["block"]["max_gas"]
        .as_str()
        .ok_or_else(|| "chain_id not found".to_owned())?
        .parse::<i64>()?;
    let max_age_num_blocks = genesis["consensus_params"]["evidence"]["max_age_num_blocks"]
        .as_str()
        .ok_or_else(|| "chain_id not found".to_owned())?
        .parse::<i64>()?;
    let max_age_duration = genesis["consensus_params"]["evidence"]["max_age_duration"]
        .as_str()
        .ok_or_else(|| "chain_id not found".to_owned())?
        .parse::<u64>()?;
    let app_state_bytes = genesis["app_state"].to_string().as_bytes().to_vec();

    let time = DateTime::parse_from_rfc3339(genesis_time)?;

    let result: GenesisInfo = GenesisInfo {
        time_seconds: time.timestamp(),
        time_nanos: 0,
        chain_id: chain_id.to_string(),
        pub_key_types,
        max_bytes,
        max_gas,
        max_age_num_blocks,
        max_age_duration,
        app_state_bytes,
    };

    Ok(result)
}

pub fn get_option_from_node_args(option_name: NodeOptionVariables) -> Option<String> {
    let node_args: Vec<String> = std::env::args().collect();
    let mut arg_ids = ArgIds {
        server_url_temp_id: 0,
        genesis_state_path_temp_id: 0,
        rpc_url_temp_id: 0,
    };
    let abci_server_url_option = "--abci_server_url";
    let abci_genesis_state_path_option = "--abci_genesis_state_path";
    let abci_rpc_url_option = "--abci_rpc_url";

    for (arg_id, arg) in node_args.iter().enumerate() {
        if arg == abci_server_url_option {
            arg_ids.server_url_temp_id = arg_id + 1;
        } else if arg == abci_genesis_state_path_option {
            arg_ids.genesis_state_path_temp_id = arg_id + 1;
        } else if arg == abci_rpc_url_option {
            arg_ids.rpc_url_temp_id = arg_id + 1;
        }
    }

    let mut abci_server_url = None;
    let mut abci_genesis_state_path = None;
    let mut abci_rpc_url = None;

    if arg_ids.server_url_temp_id != 0 {
        abci_server_url = Some(node_args[arg_ids.server_url_temp_id].to_string());
    }
    if arg_ids.genesis_state_path_temp_id != 0 {
        abci_genesis_state_path = Some(node_args[arg_ids.genesis_state_path_temp_id].to_string());
    }
    if arg_ids.rpc_url_temp_id != 0 {
        abci_rpc_url = Some(node_args[arg_ids.rpc_url_temp_id].to_string());
    }

    match option_name {
        NodeOptionVariables::AbciServerUrl => abci_server_url,
        NodeOptionVariables::AbciGenesisStatePath => abci_genesis_state_path,
        NodeOptionVariables::AbciRPCUrl => abci_rpc_url,
    }
}
