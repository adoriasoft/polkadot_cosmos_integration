#[cfg(test)]
pub mod integration_tests_abci {

    use frame_support::{assert_ok};
    use abci::DEFAULT_ABCI_URL;

    #[test]
    fn should_connect() {
        assert_ok!(abci::connect_or_get_connection(DEFAULT_ABCI_URL).unwrap().echo("Test echo".to_owned()));
    }

    #[test]
    fn should_query() {
        assert_ok!(abci::connect_or_get_connection(DEFAULT_ABCI_URL).unwrap().query(
            "/a/b/c".to_owned(),
            "SomeQuery".as_bytes().to_vec(),
            0,
            false
        ));
    }
}
