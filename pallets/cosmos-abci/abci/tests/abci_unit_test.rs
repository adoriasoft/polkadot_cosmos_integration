use abci::*;

#[test]
fn test_abci_echo() {
    let mut abci_mock = MockAbciInterface::new();
    abci_mock.expect_echo().returning(|mut v: String| {
        let mut ret = MockResponseEcho::new();
        ret.expect_get_message().returning(move || -> String {
            v.push_str(&v.clone());
            v.clone()
        });
        Ok(Box::new(ret))
    });

    set_abci_instance(Box::new(abci_mock)).unwrap();

    assert_eq!(
        get_abci_instance()
            .unwrap()
            .echo("Hello".to_string())
            .unwrap()
            .get_message(),
        "HelloHello".to_string()
    );

    assert_eq!(
        get_abci_instance()
            .unwrap()
            .echo("HiHello".to_string())
            .unwrap()
            .get_message(),
        "HiHelloHiHello".to_string()
    );

    assert_ne!(
        get_abci_instance()
            .unwrap()
            .echo("HiHello".to_string())
            .unwrap()
            .get_message(),
        "HiHello".to_string()
    );
}

#[test]
fn test_abci_check_tx() {
    let mut abci_mock = MockAbciInterface::new();
    abci_mock
        .expect_check_tx()
        .returning(|v: Vec<u8>, r#_type: i32| {
            let mut ret = MockResponseCheckTx::new();
            ret.expect_get_data()
                .returning(move || -> Vec<u8> { v.clone() });
            Ok(Box::new(ret))
        });

    set_abci_instance(Box::new(abci_mock)).unwrap();

    assert_eq!(
        get_abci_instance()
            .unwrap()
            .check_tx(vec![1, 2, 3], 0)
            .unwrap()
            .get_data(),
        vec![1, 2, 3]
    );

    assert_ne!(
        get_abci_instance()
            .unwrap()
            .check_tx(vec![1, 2, 3], 0)
            .unwrap()
            .get_data(),
        vec![1, 2, 4]
    );
}

#[test]
fn test_abci_deliver_tx() {
    let mut abci_mock = MockAbciInterface::new();
    abci_mock.expect_deliver_tx().returning(|v: Vec<u8>| {
        let mut ret = MockResponseDeliverTx::new();
        ret.expect_get_data()
            .returning(move || -> Vec<u8> { v.clone() });
        Ok(Box::new(ret))
    });

    set_abci_instance(Box::new(abci_mock)).unwrap();

    assert_eq!(
        get_abci_instance()
            .unwrap()
            .deliver_tx(vec![1, 2, 3])
            .unwrap()
            .get_data(),
        vec![1, 2, 3]
    );

    assert_ne!(
        get_abci_instance()
            .unwrap()
            .deliver_tx(vec![1, 2, 3])
            .unwrap()
            .get_data(),
        vec![1, 2, 4]
    );
}

#[test]
fn test_abci_info() {
    let mut abci_mock = MockAbciInterface::new();
    let cosmos_response_app_version = 0;
    let cosmos_response_version = "".to_string();
    let cosmos_response_data = "SimApp".to_string();

    abci_mock.expect_info().returning(|| {
        let mut ret = MockResponseInfo::new();
        ret.expect_get_data()
            .returning(move || -> String { "SimApp".to_string() });
        ret.expect_get_app_version().returning(move || -> u64 { 0 });
        ret.expect_get_version()
            .returning(move || -> String { "".to_string() });
        Ok(Box::new(ret))
    });

    set_abci_instance(Box::new(abci_mock)).unwrap();

    assert_eq!(
        get_abci_instance().unwrap().info().unwrap().get_data(),
        cosmos_response_data
    );

    assert_eq!(
        get_abci_instance()
            .unwrap()
            .info()
            .unwrap()
            .get_app_version(),
        cosmos_response_app_version
    );

    assert_eq!(
        get_abci_instance().unwrap().info().unwrap().get_version(),
        cosmos_response_version
    );
}

#[test]
fn test_abci_set_option() {
    let mut abci_mock = MockAbciInterface::new();
    let cosmos_response_code: u32 = 0;
    let cosmos_response_log = "IHAVEIDEA";
    let cosmos_response_info = "IHAVENOIDEA";
    abci_mock
        .expect_set_option()
        .returning(|_key: &str, _value: &str| {
            println!("{}", _key);
            println!("{}", _value);
            let mut ret = MockResponseSetOption::new();
            ret.expect_get_code().returning(move || -> u32 { 0 });
            ret.expect_get_log()
                .returning(move || -> String { "IHAVEIDEA".to_string() });
            ret.expect_get_info()
                .returning(move || -> String { "IHAVENOIDEA".to_string() });
            Ok(Box::new(ret))
        });

    set_abci_instance(Box::new(abci_mock)).unwrap();

    assert_eq!(
        get_abci_instance()
            .unwrap()
            .set_option("max_tx_fee", "amount=300")
            .unwrap()
            .get_code(),
        cosmos_response_code
    );

    assert_eq!(
        get_abci_instance()
            .unwrap()
            .set_option("max_tx_fee", "moneys=300")
            .unwrap()
            .get_info(),
        cosmos_response_info
    );

    assert_eq!(
        get_abci_instance()
            .unwrap()
            .set_option("max_tx_fee", "amount=300")
            .unwrap()
            .get_log(),
        cosmos_response_log
    );
}

#[test]
fn test_abci_flush() {
    let mut abci_mock = MockAbciInterface::new();
    abci_mock.expect_flush().returning(|| {
        let ret = MockResponseFlush::new();
        Ok(Box::new(ret))
    });

    set_abci_instance(Box::new(abci_mock)).unwrap();

    assert_eq!(get_abci_instance().unwrap().flush().is_ok(), true);
}
