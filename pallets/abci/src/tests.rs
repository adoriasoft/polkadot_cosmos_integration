// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn block_on_initialize() {
	new_test_ext().execute_with(|| {
		// Just a dummy test for the dummy function `on_initialize`
		assert_ok!(ABCIModule::on_initialize());
	});
}

fn block_on_finilize() {
	new_test_ext().execute_with(|| {
		// Just a dummy test for the dummy function `on_finalize`
		assert_ok!(ABCIModule::on_finalize());
	});
}

