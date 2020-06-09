// Tests to be written here

use crate::{mock::*};
use frame_support::{assert_ok};
use sp_runtime::transaction_validity::TransactionSource;

#[test]
fn block_on_finalize() {
	new_test_ext().execute_with(|| {
		ABCIModule::do_finalize();
	});
}

#[test]
fn block_on_initialize() {
	new_test_ext().execute_with(|| {
		ABCIModule::do_initialize(100);
		ABCIModule::do_initialize(12);
		ABCIModule::do_initialize(3);
	});
}

#[test]
fn transaction_deliver_tx() {
	new_test_ext().execute_with(|| {
		let message : Vec<u8> = vec![1, 2, 3, 4, 5];
		assert_ok!(ABCIModule::deliver_tx(Origin::signed(1), message));
	});
}

#[test]
fn transaction_check_tx() {
	new_test_ext().execute_with(|| {
		let source : TransactionSource = TransactionSource::InBlock;
		let message : Vec<u8> = vec![1, 2, 3, 4, 5];
		ABCIModule::check_tx(source, &message);
	});
}