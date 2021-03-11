use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

pub struct ExposureOf<T>(sp_std::marker::PhantomData<T>);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, Balance> {
    pub total: Balance,
    pub own: Balance,
    pub others: Vec<(AccountId, Balance)>,
}

pub struct StashOf<T>(sp_std::marker::PhantomData<T>);

pub type CosmosAccountPubKey = Vec<u8>;

#[derive(Encode, Decode)]
pub struct CosmosAccount {
    pub pub_key: CosmosAccountPubKey,
    pub power: i64,
}
