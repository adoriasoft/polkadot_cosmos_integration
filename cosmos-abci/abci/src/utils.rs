use chrono::DateTime;

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

pub fn parse_cosmos_genesis_file(genesis: &str) -> Result<GenesisInfo, Box<dyn std::error::Error>> {
    let genesis: serde_json::Value = serde_json::from_str(genesis).map_err(|e| e.to_string())?;
    let chain_id = genesis["chain_id"]
        .as_str()
        .ok_or("chain_id not found".to_owned())?;
    let genesis_time = genesis["genesis_time"]
        .as_str()
        .ok_or("chain_id not found".to_owned())?;
    let pub_key_types: Vec<String> = genesis["consensus_params"]["validator"]["pub_key_types"]
        .as_array()
        .ok_or("pub_keys_types not found".to_owned())?
        .into_iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect();
    let max_bytes = genesis["consensus_params"]["block"]["max_bytes"]
        .as_str()
        .ok_or("chain_id not found".to_owned())?
        .parse::<i64>()?;
    let max_gas = genesis["consensus_params"]["block"]["max_gas"]
        .as_str()
        .ok_or("chain_id not found".to_owned())?
        .parse::<i64>()?;
    let max_age_num_blocks = genesis["consensus_params"]["evidence"]["max_age_num_blocks"]
        .as_str()
        .ok_or("chain_id not found".to_owned())?
        .parse::<i64>()?;
    let max_age_duration = genesis["consensus_params"]["evidence"]["max_age_duration"]
        .as_str()
        .ok_or("chain_id not found".to_owned())?
        .parse::<u64>()?;
    let app_state_bytes = genesis["app_state"].to_string().as_bytes().to_vec();

    let time = DateTime::parse_from_rfc3339(genesis_time)?;

    let result: GenesisInfo = GenesisInfo {
        time_seconds: time.timestamp(),
        time_nanos: 0,
        chain_id: chain_id.to_string(),
        pub_key_types: pub_key_types,
        max_bytes: max_bytes,
        max_gas: max_gas,
        max_age_num_blocks: max_age_num_blocks,
        max_age_duration: max_age_duration,
        app_state_bytes: app_state_bytes,
    };

    Ok(result)
}
