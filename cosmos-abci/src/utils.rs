use sp_std::{fmt, prelude::*};

pub struct AbciCommitResponse {
    pub height: i64,
    pub hash: Vec<u8>,
}

pub trait AbciCommitResponseToVec {
    fn owned_to_vec(&self, value: Vec<u8>) -> Vec<u8>;
}

impl AbciCommitResponseToVec for AbciCommitResponse {
    fn owned_to_vec(&self, value: Vec<u8>) -> Vec<u8> {
        [&value[..], &vec![101][..], &self.hash[..]].concat()
    }
}

impl fmt::Display for AbciCommitResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.height)
    }
}
