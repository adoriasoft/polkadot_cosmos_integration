#[cfg(test)]
pub mod integration_tests_abci_2 {

    use frame_support::{assert_ok};
    use abci::DEFAULT_ABCI_URL;

    #[test]
    fn should_init_chain() {
        assert_ok!(abci::connect_or_get_connection(DEFAULT_ABCI_URL).unwrap()
            .init_chain(
                vec![],
                2000000,
                100,
                360000,
                18004000,
                vec!["abc".to_string(), "onemorekey".to_string()]
            ));
    }

    #[test]
    fn should_delfiver_tx() {
        assert_ok!(abci::connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .deliver_tx(vec![])
        );
    }

    #[test]
    fn should_check_tx() {
        assert_ok!(abci::connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .check_tx(vec![], 0)
        );
    }
}
