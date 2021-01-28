use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use pallet_cosmos_abci::{crypto, Call, Module, Trait, KEY_TYPE};
use pallet_session::*;
use sp_core::{
    crypto::{key_types::DUMMY, KeyTypeId},
    H256,
};
use sp_runtime::{
    generic, impl_opaque_keys,
    testing::{TestXt, UintAuthorityId},
    traits::{
        BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, OpaqueKeys, Verify,
    },
    AccountId32, MultiSignature, Perbill, RuntimeAppPublic,
};
use sp_std::boxed::*;
use std::cell::RefCell;

impl_opaque_keys! {
    pub struct MockSessionKeys {
        pub dummy: UintAuthorityId,
    }
}

impl From<UintAuthorityId> for MockSessionKeys {
    fn from(dummy: UintAuthorityId) -> Self {
        Self { dummy }
    }
}

pub const KEY_ID_A: KeyTypeId = KeyTypeId([4; 4]);
pub const KEY_ID_B: KeyTypeId = KeyTypeId([9; 4]);
pub type Historical = pallet_session::historical::Module<Test>;

#[derive(Debug, Clone, codec::Encode, codec::Decode, PartialEq, Eq)]
pub struct PreUpgradeMockSessionKeys {
    pub a: [u8; 32],
    pub b: [u8; 64],
}

impl OpaqueKeys for PreUpgradeMockSessionKeys {
    type KeyTypeIdProviders = ();

    fn key_ids() -> &'static [KeyTypeId] {
        &[KEY_ID_A, KEY_ID_B]
    }

    fn get_raw(&self, i: KeyTypeId) -> &[u8] {
        match i {
            i if i == KEY_ID_A => &self.a[..],
            i if i == KEY_ID_B => &self.b[..],
            _ => &[],
        }
    }
}

impl_outer_origin! {
    pub enum Origin for Test where system = frame_system {}
}

thread_local! {
    pub static VALIDATORS: RefCell<Vec<AccountId>> = RefCell::new(vec![AccountId32::default(), AccountId32::default(), AccountId32::default()]);
    pub static NEXT_VALIDATORS: RefCell<Vec<AccountId>> = RefCell::new(vec![AccountId32::default(), AccountId32::default(), AccountId32::default()]);
    pub static AUTHORITIES: RefCell<Vec<UintAuthorityId>> =
        RefCell::new(vec![UintAuthorityId(1), UintAuthorityId(2), UintAuthorityId(3)]);
    pub static FORCE_SESSION_END: RefCell<bool> = RefCell::new(false);
    pub static SESSION_LENGTH: RefCell<u64> = RefCell::new(2);
    pub static SESSION_CHANGED: RefCell<bool> = RefCell::new(false);
    pub static TEST_SESSION_CHANGED: RefCell<bool> = RefCell::new(false);
    pub static DISABLED: RefCell<bool> = RefCell::new(false);
    // Stores if `on_before_session_end` was called
    pub static BEFORE_SESSION_END_CALLED: RefCell<bool> = RefCell::new(false);
}

pub struct TestShouldEndSession;
impl ShouldEndSession<u64> for TestShouldEndSession {
    fn should_end_session(now: u64) -> bool {
        let l = SESSION_LENGTH.with(|l| *l.borrow());
        now % l == 0
            || FORCE_SESSION_END.with(|l| {
                let r = *l.borrow();
                *l.borrow_mut() = false;
                r
            })
    }
}

pub struct TestSessionHandler;
impl SessionHandler<AccountId> for TestSessionHandler {
    const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[UintAuthorityId::ID];
    fn on_genesis_session<T: OpaqueKeys>(_validators: &[(AccountId, T)]) {}
    fn on_new_session<T: OpaqueKeys>(
        changed: bool,
        validators: &[(AccountId, T)],
        _queued_validators: &[(AccountId, T)],
    ) {
        SESSION_CHANGED.with(|l| *l.borrow_mut() = changed);
        AUTHORITIES.with(|l| {
            *l.borrow_mut() = validators
                .iter()
                .map(|(_, id)| id.get::<UintAuthorityId>(DUMMY).unwrap_or_default())
                .collect()
        });
    }
    fn on_disabled(_validator_index: usize) {
        DISABLED.with(|l| *l.borrow_mut() = true)
    }
    fn on_before_session_ending() {
        BEFORE_SESSION_END_CALLED.with(|b| *b.borrow_mut() = true);
    }
}

pub struct TestSessionManager;
impl SessionManager<AccountId> for TestSessionManager {
    fn end_session(_: u32) {}
    fn start_session(_: u32) {}
    fn new_session(_: u32) -> Option<Vec<AccountId>> {
        if !TEST_SESSION_CHANGED.with(|l| *l.borrow()) {
            VALIDATORS.with(|v| {
                let mut v = v.borrow_mut();
                *v = NEXT_VALIDATORS.with(|l| l.borrow().clone());
                Some(v.clone())
            })
        } else if DISABLED.with(|l| std::mem::replace(&mut *l.borrow_mut(), false)) {
            // If there was a disabled validator, underlying conditions have changed
            // so we return `Some`.
            Some(VALIDATORS.with(|v| v.borrow().clone()))
        } else {
            None
        }
    }
}

impl pallet_session::historical::SessionManager<AccountId, AccountId> for TestSessionManager {
    fn end_session(_: u32) {}
    fn start_session(_: u32) {}
    fn new_session(new_index: u32) -> Option<Vec<(AccountId, AccountId)>> {
        <Self as SessionManager<_>>::new_session(new_index).map(|vals| {
            vals.into_iter()
                .map(|val| (val.clone(), val.clone()))
                .collect()
        })
    }
}

#[derive(Clone, Eq, PartialEq)]
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
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
}

impl pallet_sudo::Trait for Test {
    type Event = ();
    type Call = Call<Test>;
}

// TODO
// Complete pallet trait impl.
impl pallet_grandpa::Trait for Test {
    type Event = ();
   // type Call = From<Call<Test>>;
   // type KeyOwnerProofSystem = Historical;
   // type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, crypto::ABCIAuthId)>>::Proof;
   // type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
   // type HandleEquivocation = ();
    type WeightInfo = ();
}

impl Trait for Test {
    type AuthorityId = crypto::ABCIAuthId;
    type Call = Call<Test>;
    type Subscription = ();
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

parameter_types! {
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
    /// 2 blocks = session duration.
    pub const Period: u64 = 2;
    pub const Offset: u64 = 0;
}

// TODO: Fix it with this example: https://github.com/paritytech/substrate/blob/master/frame/session/src/mock.rs

impl pallet_session::Trait for Test {
    type Event = ();
    type ValidatorId = <Self as frame_system::Trait>::AccountId;
    type ValidatorIdOf = pallet_cosmos_abci::utils::StashOf<Self>;
    type ShouldEndSession = TestShouldEndSession;
    type NextSessionRotation = ();
    type SessionManager = TestSessionManager;
    type SessionHandler = TestSessionHandler;
    type Keys = MockSessionKeys;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type WeightInfo = ();
}

pub type AbciModule = Module<Test>;

#[test]
fn should_match_key_type() {
    println!("{:?}", KEY_TYPE);
    assert_eq!(KEY_TYPE, KeyTypeId(*b"abci"));
}

#[test]
fn should_begin_block_on_initialize() {
    use testcontainers::*;
    let docker = clients::Cli::default();
    let cosmos = images::generic::GenericImage::new("andoriasoft/cosmos-node:latest")
        .with_args(vec![
            "start".to_owned(),
            "--with-tendermint=false".to_owned(),
            "--transport=grpc".to_owned(),
        ])
        .with_wait_for(images::generic::WaitFor::message_on_stdout("starting ABCI"));
    let node = docker.run(cosmos);

    let url = format!(
        "tcp://localhost:{}",
        node.get_host_port(26658).unwrap_or(26658)
    );
    pallet_abci::set_abci_instance(Box::new(
        pallet_abci::grpc::AbciinterfaceGrpc::connect(&url)
            .map_err(|_| "failed to connect")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci instance")
    .unwrap();

    let mut client = pallet_abci::get_abci_instance().unwrap();

    let genesis = pallet_abci::utils::parse_cosmos_genesis_file(pallet_abci::TEST_GENESIS).unwrap();
    let result = client.init_chain(
        genesis.time_seconds,
        genesis.time_nanos,
        &genesis.chain_id,
        genesis.pub_key_types,
        genesis.max_bytes,
        genesis.max_gas,
        genesis.max_age_num_blocks,
        genesis.max_age_duration,
        genesis.app_state_bytes,
        vec![],
    );
    assert!(result.is_ok(), "should successfully call init chain");

    // FIXME: Doesn't work after begin_block call
    // assert_eq!(AbciModule::call_on_initialize(1), true);
    // assert_eq!(AbciModule::call_on_finalize(1), true);

    // let data = [0u8, 24];
    // let hash = Hasher::hash(&data);
    // AbciModule::deliver_tx(Origin::signed(AccountId32::from(Into::<[u8; 32]>::into(hash))), vec![]);
    // todo
}
