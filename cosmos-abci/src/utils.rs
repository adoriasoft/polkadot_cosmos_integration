use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{fmt, prelude::*};

pub struct AbciCommitResponse {
    pub height: i64,
    pub hash: Vec<u8>,
}

/// Abci commit data to vector util.
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

/// Return exposure for account.
pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, Balance> {
    pub total: Balance,
    pub own: Balance,
    pub others: Vec<(AccountId, Balance)>,
}

/// Return stash for account.
pub struct StashOf<T>(sp_std::marker::PhantomData<T>);

/// Cosmos node account ID.
pub type CosmosAccountId = Vec<u8>;

/// Compare changes in two arrays.
pub fn is_array_changed<T: PartialEq>(prev_items: Vec<T>, curr_items: Vec<T>) -> bool {
    if prev_items.len() == curr_items.len() {
        return prev_items
            .iter()
            .zip(curr_items)
            .filter(|(a, b)| a == &b)
            .count() != prev_items.len();
    } else {
        return true;
    }
}

pub fn hardcoded_cosmos_validators(session_index: u32) -> Vec<CosmosAccountId> {
    if session_index > 10 {
        return vec![
            vec![66, 111, 98, 98, 121, 83, 111, 98, 98, 121],
            vec![76, 111, 118, 101, 108, 121, 77, 111, 110, 107, 101, 121],
            vec![83, 111, 100, 97, 67, 111, 111, 108]
        ];
    }
    vec![]
}
