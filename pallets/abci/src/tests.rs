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
