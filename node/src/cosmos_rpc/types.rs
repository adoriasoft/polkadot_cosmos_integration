//! types module that define Substrate RPC responses.

use serde_derive::{Deserialize, Serialize};

/// AbciQueryParams RPC response.
#[derive(Serialize, Deserialize)]
pub struct AbciQueryParams {
    pub path: String,
    pub data: String,
    pub height: String,
    pub prove: bool,
}

/// AbciTxCommitParams RPC response.
#[derive(Serialize, Deserialize)]
pub struct AbciTxCommitParams {
    pub tx: String,
}

/// AbciTxBroadcastParams RPC response.
#[derive(Serialize, Deserialize)]
pub struct AbciTxBroadcastParams {
    pub tx: String,
}

/// AbciInfo RPC response.
#[derive(Serialize, Deserialize, Debug)]
pub struct AbciInfo {}

/// AbciSetOption RPC response.
#[derive(Serialize, Deserialize, Debug)]
pub struct AbciSetOption {
    pub key: String,
    pub value: String,
}

/// AbciCheckTx RPC response.
#[derive(Serialize, Deserialize)]
pub struct AbciCheckTx {
    pub tx: String,
    pub check_tx_type: i32,
}
