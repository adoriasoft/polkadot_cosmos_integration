use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ABCIQueryParams {
    pub path: String,
    pub data: String,
    pub height: String,
    pub prove: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ABCITxCommitParams {
    pub tx: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbciInfo {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbciSetOption {
    pub key: String,
    pub value: String,
}
