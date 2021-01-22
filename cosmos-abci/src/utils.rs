use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

pub struct AbciCommitResponse {
    pub height: i64,
    pub hash: Vec<u8>,
}

/// ExposureOf for account.
pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

/// Exposure for account.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, Balance> {
    pub total: Balance,
    pub own: Balance,
    pub others: Vec<(AccountId, Balance)>,
}

/// StashOf for account.
pub struct StashOf<T>(sp_std::marker::PhantomData<T>);

/// Cosmos account Public key.
pub type CosmosAccountPubKey = Vec<u8>;

/// Cosmos account structure.
#[derive(Encode, Decode, Default)]
pub struct CosmosAccount {
    pub pub_key: CosmosAccountPubKey,
    pub pub_key_type: u64,
    pub power: i64,
}
