#[test]
fn test_abci_storage() {
    // init storage instance
    let storage = abci_storage::rocksdb::AbciStorageRocksdb::init("abci_storage_test").unwrap();
    abci_storage::set_abci_storage_instance(Box::new(storage)).unwrap();

    abci_storage::get_abci_storage_instance()
        .unwrap()
        .write(b"key 1".to_vec(), b"value 1".to_vec())
        .unwrap();

    abci_storage::get_abci_storage_instance()
        .unwrap()
        .write(b"key 2".to_vec(), b"value 2".to_vec())
        .unwrap();

    abci_storage::get_abci_storage_instance()
        .unwrap()
        .write(b"key 3".to_vec(), b"value 3".to_vec())
        .unwrap();

    assert_eq!(
        abci_storage::get_abci_storage_instance()
            .unwrap()
            .get(b"key 1".to_vec())
            .unwrap(),
        Some(b"value 1".to_vec())
    );

    assert_eq!(
        abci_storage::get_abci_storage_instance()
            .unwrap()
            .get(b"key 2".to_vec())
            .unwrap(),
        Some(b"value 2".to_vec())
    );

    assert_eq!(
        abci_storage::get_abci_storage_instance()
            .unwrap()
            .get(b"key 3".to_vec())
            .unwrap(),
        Some(b"value 3".to_vec())
    );

    assert_eq!(
        abci_storage::get_abci_storage_instance()
            .unwrap()
            .get(b"key 4".to_vec())
            .unwrap(),
        None
    );
}
