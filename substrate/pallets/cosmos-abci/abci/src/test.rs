#[cfg(test)]
pub mod test {

    const DEFAULT_ABCI_URL: &str = "tcp://localhost:26658";

    #[test]
    fn should_connect() {
        let connection = super::connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .echo("Hello there".to_owned());
        assert_eq!(connection.is_ok(), true);
    }

    #[test]
    fn should_deliver_tx() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .deliver_tx(vec![]);
        println!("deliver_tx result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn should_check_tx() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL)
            .unwrap()
            .check_tx(vec![], 0);
        println!("check_tx result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn should_query() {
        let result = connect_or_get_connection(DEFAULT_ABCI_URL).unwrap().query(
            "/a/b/c".to_owned(),
            "IHAVENOIDEA".as_bytes().to_vec(),
            0,
            false,
        );
        println!("query result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
