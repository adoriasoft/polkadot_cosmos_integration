use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ABCIQueryParams {
    pub path: String,
    pub data: String,
    pub height: String,
    pub prove: bool,
}