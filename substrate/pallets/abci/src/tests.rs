// Tests to be written here

use crate::mock::*;
use frame_support::{debug, assert_ok};
use sp_runtime::transaction_validity::TransactionSource;
use codec::Decode;
use sp_core::{
	offchain::{OffchainExt, TransactionPoolExt, testing},
	testing::KeyStore,
	traits::KeystoreExt,
};
use sp_runtime::RuntimeAppPublic;

#[test]
fn block_on_finalize() {
	new_test_ext().execute_with(|| {
		AbciModule::do_finalize();
	});
}

#[test]
fn block_on_initialize() {
	new_test_ext().execute_with(|| {
		AbciModule::do_initialize(100);
		AbciModule::do_initialize(12);
		AbciModule::do_initialize(3);
	});
}

#[test]
fn transaction_deliver_tx() {
	new_test_ext().execute_with(|| {
		let message : Vec<u8> = vec![1, 2, 3, 4, 5];
		assert_ok!(AbciModule::deliver_tx(Origin::signed(Default::default()), message));
	});
}

#[test]
fn transaction_check_tx() {
	new_test_ext().execute_with(|| {
		let source : TransactionSource = TransactionSource::InBlock;
		let message : Vec<u8> = vec![1, 2, 3, 4, 5];
		AbciModule::do_check_tx(source, &message);
	});
}

#[test]
fn should_submit_signed_transaction_on_chain() {
	const PHRASE: &str = "news slush supreme milk chapter athlete soap sausage put clutch what kitten";

	let (offchain, _offchain_state) = testing::TestOffchainExt::new();
	let (pool, pool_state) = testing::TestTransactionPoolExt::new();
	let keystore = KeyStore::new();
	keystore.write().sr25519_generate_new(
		crate::crypto::Public::ID,
		Some(&format!("{}/hunter1", PHRASE))
	).unwrap();


	let mut t = sp_io::TestExternalities::default();
	t.register_extension(OffchainExt::new(offchain));
	t.register_extension(TransactionPoolExt::new(pool));
	t.register_extension(KeystoreExt(keystore));

	debug::info!("Hello there");
	println!("Hello there !");

	t.execute_with(|| {
		// when
		let res = AbciModule::make_request();
		match res {
			Ok(results) => {
				println!("Results: {:?}", results.len());
				for val in &results {
					match val {
						Ok(acc) => println!("Submitted transaction: {:?}", acc),
						Err(e) => println!("Failed to submit transaction: {:?}", e),
					}
				}
			}
			Err(e) => {
				println!("Error: {}", e);
			}
		}
		// then
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(tx.signature.unwrap().0, 0);
		assert_eq!(tx.call, Call::some_function());
	});
}
