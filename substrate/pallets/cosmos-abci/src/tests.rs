// To execute most of these tests you need to run Cosmos node with gRPC ABCI.

use super::{abci_interface, CosmosAbci};
use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_abci_echo() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        let _res = abci_interface::echo("Hello from runtime interface");
        // assert_ok!(res);
    });
}

#[test]
fn test_abci_check_tx() {
    // Should run with the working abci server
    new_test_ext().execute_with(|| {
        let _res = <AbciModule as CosmosAbci>::check_tx(vec![]);
        // assert_ok!(res);
    });
}
