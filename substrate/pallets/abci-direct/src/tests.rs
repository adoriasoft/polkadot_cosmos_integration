// Tests to be written here

use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_module() {
    new_test_ext().execute_with(|| {
        AbciModule::do_finalize();

        AbciModule::do_initialize(100);
        AbciModule::do_initialize(12);
        AbciModule::do_initialize(3);
    });
}

#[test]
fn block_on_initialize() {
    new_test_ext().execute_with(|| {
        assert_ok!(crate::request::get_method("InitChain"));
    });
}
