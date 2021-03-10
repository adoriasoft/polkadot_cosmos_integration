use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

/// Return Exposure for account.
pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

/// Exposure for account.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, Balance> {
    pub total: Balance,
    pub own: Balance,
    pub others: Vec<(AccountId, Balance)>,
}

/// Return Stash for account.
pub struct StashOf<T>(sp_std::marker::PhantomData<T>);

/// Cosmos account Public key.
pub type CosmosAccountPubKey = Vec<u8>;

/// Cosmos account structure.
#[derive(Encode, Decode)]
pub struct CosmosAccount {
    pub pub_key: CosmosAccountPubKey,
    pub power: i64,
}
