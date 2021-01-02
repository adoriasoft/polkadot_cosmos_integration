use chrono::DateTime;
use std::{fs, path::PathBuf};
use serde::{Serialize, Deserialize};

// TODO Do we need this type for conversion?
pub struct CombinedValidator0 { }

#[derive(Debug)]
pub struct CombinedValidator {
    pub pub_key: Vec<u8>,
    pub address: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableValidatorUpdate {
    pub key_data: Vec<u8>,
    pub r#type: String,
    pub power: i64,
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
    pub validators: Vec<CombinedValidator>,
}

pub fn serialize_vec<T: serde::Serialize>(
    validators: Vec<T>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(bincode::serialize(&validators).map_err(|_| "cannot serialize")?)
}

pub fn deserialize_vec<'a, T: serde::Deserialize<'a>>(
    bytes: &'a [u8],
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let res = bincode::deserialize(bytes);
    match res {
        Ok(response) => {
            Ok(response)
        },
        Err(err) => {
            println!("Cannot deserialize {:?}", err);
            Ok(Vec::new())
        }
    }
}

fn get_genesis_from_file() -> Result<String, Box<dyn std::error::Error>> {
    let path: PathBuf = std::env::var("ABCI_GENESIS_STATE_PATH")
        .map_err(|_| "Failed to get app state file path")?
        .into();
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

pub fn get_validator_address(validator_pub_key: Vec<u8>) -> Option<Vec<u8>> {
    let genesis = parse_cosmos_genesis_file(&get_abci_genesis()).unwrap();
    let addresses: Vec<Vec<u8>> = genesis.validators
        .iter()
        .filter(|combined| {
            combined.pub_key.clone() == validator_pub_key
        })
        .map(|combined| {
            combined.address.clone()
        })
        .collect();
    if addresses.len() > 0 {
        return Some(addresses[0].clone());
    }
    None
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
    let validators: Vec<CombinedValidator0> = vec![];
    // TODO Parse initial validators set.

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
        validators: validators
            .iter()
            .map(|_validator| {
                CombinedValidator {
                    address: vec![],
                    pub_key: vec![],
                }
            })
            .collect()
    };

    println!("Validators from genesis {:?}", result.validators);

    Ok(result)
}
