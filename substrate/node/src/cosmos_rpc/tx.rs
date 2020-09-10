use std::sync::Arc;

use codec::{Decode, Encode};
use cosmos_abci::ExtrinsicConstructionApi;
use node_template_runtime::{opaque::Block, SignedExtra};
use sc_client_api::BlockBackend;
pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use sp_runtime::generic::{BlockId, Era, SignedPayload, UncheckedExtrinsic};
use sp_transaction_pool::{TransactionPool, TransactionSource};
use substrate_frame_rpc_system::AccountNonceApi;

pub async fn deliver_tx<P: TransactionPool + 'static>(
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
        frame_system::CheckEra::from(Era::mortal(256, 0)),
        frame_system::CheckNonce::from(nonce),
        frame_system::CheckWeight::new(),
        pallet_transaction_payment::ChargeTransactionPayment::from(1),
    );
    let raw_payload = SignedPayload::<Vec<u8>, SignedExtra>::new(call.clone().into(), extra.clone()).unwrap();
    let signature = raw_payload.using_encoded(|payload| alice.pair().sign(payload));
    let (call, extra, _) = raw_payload.deconstruct();
    let tx = UncheckedExtrinsic::new_signed(call, alice.to_account_id(), signature, extra);

    let bytes: Vec<u8> = tx.encode();
    let xt = Decode::decode(&mut &bytes[..]).unwrap();
    let hash = pool.submit_one(&best_block_num, TransactionSource::External, xt).await;
    println!("{:?}", hash);
}
