// Tests to be written here

use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn block_on_initialize() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        //AbciModule::do_init_chain();
        //assert_ok!(crate::request::get_method("InitChain"));
    });
}
