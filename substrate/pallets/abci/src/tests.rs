// Tests to be written here

use crate::abci_grpc::*;
use crate::mock::*;
use frame_support::assert_ok;
use sp_core::offchain::{testing, OffchainExt};
use sp_io::TestExternalities;
use sp_runtime::{offchain::http, transaction_validity::TransactionSource};
use sp_std::str;

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
        let message_1 = TxMessage {
            tx: vec![1, 2, 3, 4, 5],
        };
        assert_ok!(AbciModule::deliver_tx(
            Origin::signed(Default::default()),
            message_1.clone(),
        ));

        let message_2 = TxMessage {
            tx: vec![5, 4, 3, 2, 1],
        };
        assert_ok!(AbciModule::deliver_tx(
            Origin::signed(Default::default()),
            message_2.clone(),
        ));

        let req = vec![message_1, message_2];
        assert_eq!(AbciModule::requests(), req);

        assert_ok!(AbciModule::finish_deliver_tx(
            Origin::signed(Default::default()),
            vec![req[0].clone()],
        ));
        assert_eq!(AbciModule::requests(), vec![req[1].clone()]);
    });
}

#[test]
fn transaction_check_tx() {
    new_test_ext().execute_with(|| {
        let source: TransactionSource = TransactionSource::InBlock;
        AbciModule::do_check_tx(source);
    });
}

#[test]
fn abci_request_echo() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let mut t = TestExternalities::default();
    t.register_extension(OffchainExt::new(offchain));

    t.execute_with(|| {
        let url: &[u8] = &[ABCI_SERVER_URL, b"Echo"].concat();
        let request_url = str::from_utf8(url).unwrap();

        let request = http::Request::get(request_url);
        let pending = request.send().unwrap();

        // make sure it's sent correctly
        state.write().fulfill_pending_request(
            0,
            testing::PendingRequest {
                method: "GET".into(),
                uri: request_url.into(),
                sent: true,
                ..Default::default()
            },
            b"8082".to_vec(),
            None,
        );

        assert_ok!(pending.wait());
    });
}
