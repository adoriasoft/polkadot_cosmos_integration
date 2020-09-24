#[cfg(test)]
pub mod integration_tests_abci {

    // use frame_support::{assert_ok};
    use abci::DEFAULT_ABCI_URL;

    #[test]
    fn should_connect() {
        let mut connection = abci::connect_or_get_connection(DEFAULT_ABCI_URL).unwrap();
        let echo = connection.echo("Test echo".to_owned());
        assert_eq!(echo.is_ok(), true);
    }

    #[test]
    fn should_query() {
        let result = abci::connect_or_get_connection(DEFAULT_ABCI_URL).unwrap().query(
            "/a/b/c".to_owned(),
            "IHAVENOIDEA".as_bytes().to_vec(),
            0,
            false
        );
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn should_init_chain() {
        let result = abci::connect_or_get_connection(DEFAULT_ABCI_URL).unwrap()
            .init_chain(
                vec![],
                2000000,
                100,
                360000,
                18004000,
                vec!["abc".to_string()]
            );
        // todo
        assert_eq!(true, true);
        // assert_ok!();
    }

    #[test]
    fn should_deliver_tx() {
        let result = abci::connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .deliver_tx(vec![1, 2, 3]);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn should_check_tx() {
        let result = abci::connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .check_tx(vec![], 0);
        assert_eq!(result.is_ok(), true);
    }
}
