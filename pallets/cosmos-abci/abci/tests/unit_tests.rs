use abci::*;

#[test]
fn test_abci_echo() {
    let mut abci_mock = MockABCIInterface::new();
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
    let mut abci_mock = MockABCIInterface::new();
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
    let mut abci_mock = MockABCIInterface::new();
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
