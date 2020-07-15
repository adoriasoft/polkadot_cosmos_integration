// Tests to be written here

use super::TxMessage;
use crate::mock::*;
use frame_support::assert_ok;
use sp_runtime::transaction_validity::TransactionSource;

#[test]
fn test_get_method() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        assert_ok!(crate::request::get_method("InitChain"));
    });
}

#[test]
fn test_call_deliver_tx() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        assert_ok!(AbciModule::deliver_tx(
            Origin::signed(Default::default()),
            TxMessage { tx: vec![] },
        ));
        let source: TransactionSource = TransactionSource::InBlock;
        AbciModule::do_check_tx(source, vec![]);
    });
}
