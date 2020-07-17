/// To execute most of these tests you need to run Cosmos node with gRPC ABCI.

use super::{TxMessage, BlockMessage, abci_interface};
use crate::mock::*;
use frame_support::assert_ok;
use sp_runtime::transaction_validity::TransactionSource;

#[test]
fn test_rest_methods() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        assert_ok!(crate::request::get_method("InitChain"));

        let blk_msg = BlockMessage { height: 1 };
        assert_ok!(crate::request::post_method("OnInitialize", &blk_msg));
    });
}

#[test]
fn test_call_deliver_tx() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        let source: TransactionSource = TransactionSource::InBlock;
        let tx = r#"
            {
                "From": "Bob",
                "To": "Bob2",
                "Amount": 100,
                "Signature": "IAAAAONEAbVha/jJGLMWc/kwmi653hfLvJMZqVGhH8uLEIx/IAAAAFrlbHncf56+eCcHSZCiJJRutjhuWZ5muuUj6UBp0bYv"
            }
        "#;
        let tx_message = TxMessage { tx: tx.as_bytes().to_vec() };

        AbciModule::do_check_tx(source, tx_message.tx.clone());

        assert_ok!(AbciModule::deliver_tx(
            Origin::signed(Default::default()),
            tx_message,
        ));
    });
}

#[test]
fn test_on_initialize_and_on_finalize() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        let blk_msg = BlockMessage { height: 1 };

        assert_eq!(abci_interface::on_initialize(&blk_msg), true);
        assert_eq!(abci_interface::on_finalize(&blk_msg), true);
    });
}

#[test]
fn test_check_tx_and_deliver_tx_to_cosmos() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        let tx = r#"
            {
                "From": "Bob",
                "To": "Bob2",
                "Amount": 100,
                "Signature": "IAAAAONEAbVha/jJGLMWc/kwmi653hfLvJMZqVGhH8uLEIx/IAAAAFrlbHncf56+eCcHSZCiJJRutjhuWZ5muuUj6UBp0bYv"
            }
        "#;
        let tx_message = TxMessage { tx: tx.as_bytes().to_vec() };

        assert_eq!(abci_interface::check_tx(&tx_message), true);
        assert_eq!(abci_interface::deliver_tx(&tx_message), true);
    });
}

#[test]
fn test_block_message_commit() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        let blk_msg = BlockMessage { height: 1 };

        AbciModule::do_commit(blk_msg.height);
        assert_eq!(abci_interface::commit(&blk_msg), true);
    });
}
