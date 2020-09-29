use testcontainers::*;

#[test]
fn test_abci_begin_block() {
    let docker = clients::Cli::default();
    let cosmos = images::generic::GenericImage::new("andoriasoft/cosmos-node:latest")
        .with_args(vec![
            "start".to_owned(),
            "--with-tendermint=false".to_owned(),
            "--transport=grpc".to_owned(),
        ])
        .with_wait_for(images::generic::WaitFor::message_on_stdout("starting ABCI"));
    let node = docker.run(cosmos);

    // Init ABCI instance
    let url = format!("tcp://localhost:{}", node.get_host_port(26658).unwrap());
    abci::set_abci_instance(Box::new(
        abci::grpc::AbciinterfaceGrpc::connect(&url)
            .map_err(|_| "failed to connect")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci instance")
    .unwrap();

    let mut client = abci::get_abci_instance().unwrap();
    assert!(
        abci::set_chain_id("nameservice").is_ok(),
        "should set chain id"
    );
    let result = client.echo("test".to_owned());
    assert!(result.is_ok(), "should successfully call echo");

    let result = client.init_chain(abci::TEST_GENESIS);
    assert!(result.is_ok(), "should successfully call init chain");

    let height = 1;
    let result = client.begin_block(height, vec![], vec![]);
    assert!(result.is_ok(), "should successfully call begin block");

    let result = client.check_tx(vec![], 0);
    assert!(result.is_ok(), "should successfully call check tx");

    let result = client.deliver_tx(vec![]);
    assert!(result.is_ok(), "should successfully call deliver tx");

    let result = client.end_block(height);
    assert!(result.is_ok(), "should successfully call end block");

    let result = client.commit();
    assert!(result.is_ok(), "should successfully call commit");

    let result = client.query(
        "/a/b/c".to_owned(),
        "SomeQuery".as_bytes().to_vec(),
        0,
        false,
    );
    assert!(result.is_ok(), "should successfully call query");
}
