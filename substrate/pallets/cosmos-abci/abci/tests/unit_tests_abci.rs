#[cfg(test)]
pub mod unit_tests_abci {
    use abci::{
        get_on_initialize_variable,
        increment_on_initialize_variable,
        defines_chain_id,
        get_chain_id
    };
    use frame_support::{assert_ok}; 

    #[test]
    fn should_define_chain_id() {
        assert_ok!(defines_chain_id("default-chain-id".to_string()));
    }

    #[test]
    fn should_get_defined_chain_id() {
        let existed_chain_id = get_chain_id();
        assert_eq!(existed_chain_id.unwrap().as_str(), "default-chain-id".to_string());
    }

    #[test]
    fn should_init_variable() {
        let init_variable = get_on_initialize_variable();
        assert_eq!(init_variable, 0);
    }

    #[test]
    fn should_increment_variable() {
        assert_eq!(increment_on_initialize_variable(), 1);
    }
}
