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

/// Return Exposure obj for account.
pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

/// Structure that define Exposure obj.
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
