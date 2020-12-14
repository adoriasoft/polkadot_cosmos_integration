use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{fmt, prelude::*};

/// Cosmos account ID.
pub type CosmosAccountId = Vec<u8>;

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

/// Return exposure of account.
pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, Balance> {
    pub total: Balance,
    pub own: Balance,
    pub others: Vec<(AccountId, Balance)>,
}

/// Return stash of account.
pub struct StashOf<T>(sp_std::marker::PhantomData<T>);

pub fn get_matched_accounts<AccountId>(
    all_cosmos_accounts: Vec<(CosmosAccountId, AccountId)>,
    last_cosmos_validators: Vec<CosmosAccountId>,
) -> Vec<CosmosAccountId> {
    let mut output = Vec::new();
    // output.extend(last_cosmos_validators.iter().map(|cosmos_acc_id| all_cosmos_accounts.get(cosmos_acc_id)) cosmos_acc_id).collect::<Vec<_>>());
    output
}
