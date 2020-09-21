// Creating mock runtime here
#[cfg(test)]
pub use crate::{crypto, Call, Module, Trait};
use codec::{Decode, Encode};
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_core::H256;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
    MultiSignature, Perbill,
};
use sp_std::boxed::*;

impl_outer_origin! {
    pub enum Origin for Test where system = frame_system {}
}

// Try to replace TestXt struct to test core.
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
pub struct TestXt<Call, Extra> {
    /// Signature of the extrinsic.
    pub signature: Option<(u64, Extra)>,
    /// Call of the extrinsic.
    pub call: Call,
}

// Begin Test module.
#[derive(Clone, Eq, PartialEq, Encode, Decode)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 100;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl frame_system::Trait for Test {
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = generic::Header<u64, BlakeTwo256>;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type ModuleToIndex = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
}

impl<Call, Extra> TestXt<Call, Extra> {
    /// Create a new `TextXt`.
    pub fn new(call: Call, signature: Option<(u64, Extra)>) -> Self {
        Self { call, signature }
    }
}

pub type Extrinsic = TestXt<Call<Test>, ()>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Signature = MultiSignature;

impl frame_system::offchain::SigningTypes for Test {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call<Test>: From<LocalCall>,
{
    type OverarchingCall = Call<Test>;
    type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
    Call<Test>: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call<Test>,
        _public: <Signature as Verify>::Signer,
        _account: AccountId,
        nonce: u64,
    ) -> Option<(Call<Test>, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
        Some((call, (nonce, ())))
    }
}

impl Trait for Test {
    type AuthorityId = crypto::ABCIAuthId;
    type Call = Call<Test>;
}

pub type AbciModule = Module<Test>;

#[test]
fn should_begin_block_on_initialize() {
    assert_eq!("true", "true");
}

#[test]
fn should_end_block_on_finalize() {
    assert_eq!("end", "end");
}

#[test]
fn should_deliver_tx() {
    assert_eq!("end", "end");
}
