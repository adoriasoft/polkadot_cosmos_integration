// Tests to be written here

use crate::{mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn block_on_finalize() {
	new_test_ext().execute_with(|| {
		assert_ok!(ABCIModule::do_finalize());
	});
}

#[test]
fn block_on_initialize() {
	new_test_ext().execute_with(|| {
		assert_ok!(ABCIModule::do_initialize(100));
		assert_ok!(ABCIModule::do_initialize(12));
		assert_ok!(ABCIModule::do_initialize(3));
	});
}

#[test]
fn transaction_deliver_tx() {
	new_test_ext().execute_with(|| {
		let message = vec![1, 2, 3, 4, 5];
		assert_ok!(ABCIModule::deliver_tx(Origin::signed(1), message));
	});
}

#[test]
fn transaction_check_tx() {
	new_test_ext().execute_with(|| {
	});
}