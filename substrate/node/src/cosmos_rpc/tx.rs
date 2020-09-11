use std::sync::Arc;

use codec::Encode;
use cosmos_abci::ExtrinsicConstructionApi;
use node_template_runtime::{opaque::Block, SignedExtra};
use sc_client_api::BlockBackend;
pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use sp_runtime::{
    generic::{BlockId, Era, SignedPayload, UncheckedExtrinsic},
    OpaqueExtrinsic,
};
use sp_transaction_pool::{TransactionPool, TransactionSource, error::IntoPoolError};
use substrate_frame_rpc_system::AccountNonceApi;

pub async fn deliver_tx<P: TransactionPool<Block = Block> + 'static>(
    client: Arc<crate::service::FullClient>,
    pool: Arc<P>,
    tx_value: Vec<u8>,
) {
    let api = client.runtime_api();
    let alice = AccountKeyring::Alice;

    let best = client.info().best_hash;
    let at = BlockId::<Block>::hash(best);
    let best_block_num = BlockId::number(client.chain_info().best_number.into());

    let genesis_hash = client.block_hash(0).unwrap().unwrap();
    let runtime_version = client.runtime_version_at(&at).unwrap();
    let nonce = api.account_nonce(&at, alice.to_account_id()).unwrap();
    let call: Vec<u8> = api.deliver_tx_encoded(&at, tx_value).unwrap();

    let spec_version = runtime_version.spec_version;
    let tx_version = runtime_version.transaction_version;

    let extra: SignedExtra = (
        frame_system::CheckSpecVersion::new(),
        frame_system::CheckTxVersion::new(),
        frame_system::CheckGenesis::new(),
        frame_system::CheckEra::from(Era::mortal(256, best_block_num)),
        frame_system::CheckNonce::from(nonce),
        frame_system::CheckWeight::new(),
        pallet_transaction_payment::ChargeTransactionPayment::from(0),
    );
    let raw_payload = SignedPayload::<Vec<u8>, SignedExtra>::from_raw(
        call.clone().into(),
        extra.clone(),
        (
            spec_version,
            tx_version,
            genesis_hash,
            genesis_hash,
            (),
            (),
            (),
        ),
    );//.unwrap();
    let signature = raw_payload.using_encoded(|payload| alice.pair().sign(payload));
    // let (call, extra, _) = raw_payload.deconstruct();
    let tx = UncheckedExtrinsic::new_signed(call, alice.to_account_id(), signature, extra);
    let res = pool
        .submit_one(&best_block_num, TransactionSource::External, OpaqueExtrinsic::from(tx))
        .await;
    match res {
        Ok(hash) => println!("####### HASH: {:?}", hash),
        Err(e) => match e.into_pool_error() {
            Ok(sp_transaction_pool::error::Error::AlreadyImported(err)) => println!("####### ERR: {:?}", err),
            Ok(e) => {
                println!("Error adding transaction to the pool: {:?}", e);
            }
            Err(e) => {
                println!("Error converting pool error: {:?}", e);
            }
        }
    }
}
