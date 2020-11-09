use testcontainers::*;

const STDOUT_MESSAGE: &str = "starting ABCI";
const DOCKER_IMAGE: &str = "andoriasoft/cosmos-node:latest";
const DOCKER_PORT: u16 = 26658;

#[test]
fn test_abci_begin_block() {
    let docker = clients::Cli::default();
    let cosmos = images::generic::GenericImage::new(DOCKER_IMAGE)
        .with_args(vec![
            "start".to_owned(),
            "--with-tendermint=false".to_owned(),
            "--transport=grpc".to_owned(),
        ])
        .with_wait_for(images::generic::WaitFor::message_on_stdout(STDOUT_MESSAGE));
    let node = docker.run(cosmos);
    let url = format!(
        "tcp://localhost:{}",
        node.get_host_port(DOCKER_PORT).unwrap_or(DOCKER_PORT)
    );
    abci::set_abci_instance(Box::new(
        abci::grpc::AbciinterfaceGrpc::connect(&url)
            .map_err(|_| "failed to connect")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci instance")
    .unwrap();

    let mut client = abci::get_abci_instance().unwrap();
    let result = client.echo("test".to_owned());
    assert!(result.is_ok(), "should successfully call echo");

    let genesis = abci::utils::parse_cosmos_genesis_file(abci::TEST_GENESIS).unwrap();
    let result = client.init_chain(
        genesis.time_seconds,
        genesis.time_nanos,
        &genesis.chain_id,
        genesis.pub_key_types,
        genesis.max_bytes,
        genesis.max_gas,
        genesis.max_age_num_blocks,
        genesis.max_age_duration,
        genesis.app_state_bytes,
    );
    assert!(result.is_ok(), "should successfully call init chain");

    let height = 1;
    let result = client.begin_block(height, vec![], vec![], vec![]);
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

#[test]
fn test_abci_flush() {
    let docker = clients::Cli::default();
    let cosmos = images::generic::GenericImage::new(DOCKER_IMAGE)
        .with_args(vec![
            "start".to_owned(),
            "--with-tendermint=false".to_owned(),
            "--transport=grpc".to_owned(),
        ])
        .with_wait_for(images::generic::WaitFor::message_on_stdout(STDOUT_MESSAGE));
    let node = docker.run(cosmos);
    let url = format!(
        "tcp://localhost:{}",
        node.get_host_port(DOCKER_PORT).unwrap_or(DOCKER_PORT)
    );
    abci::set_abci_instance(Box::new(
        abci::grpc::AbciinterfaceGrpc::connect(&url)
            .map_err(|_| "failed to connect")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci instance")
    .unwrap();

    let mut client = abci::get_abci_instance().unwrap();
    let flush_result = client.flush();

    assert_eq!(flush_result.is_ok(), true);
}

#[test]
fn test_abci_info() {
    let docker = clients::Cli::default();
    let cosmos = images::generic::GenericImage::new(DOCKER_IMAGE)
        .with_args(vec![
            "start".to_owned(),
            "--with-tendermint=false".to_owned(),
            "--transport=grpc".to_owned(),
        ])
        .with_wait_for(images::generic::WaitFor::message_on_stdout(STDOUT_MESSAGE));
    let node = docker.run(cosmos);
    let url = format!(
        "tcp://localhost:{}",
        node.get_host_port(DOCKER_PORT).unwrap_or(DOCKER_PORT)
    );
    abci::set_abci_instance(Box::new(
        abci::grpc::AbciinterfaceGrpc::connect(&url)
            .map_err(|_| "failed to connect")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci instance")
    .unwrap();

    let mut client = abci::get_abci_instance().unwrap();
    let info_result = client.info();

    assert_eq!(info_result.unwrap().get_data(), "SimApp");
}

#[test]
fn test_abci_set_option() {
    let docker = clients::Cli::default();
    let cosmos = images::generic::GenericImage::new(DOCKER_IMAGE)
        .with_args(vec![
            "start".to_owned(),
            "--with-tendermint=false".to_owned(),
            "--transport=grpc".to_owned(),
        ])
        .with_wait_for(images::generic::WaitFor::message_on_stdout(STDOUT_MESSAGE));
    let node = docker.run(cosmos);
    let url = format!(
        "tcp://localhost:{}",
        node.get_host_port(DOCKER_PORT).unwrap_or(DOCKER_PORT)
    );
    abci::set_abci_instance(Box::new(
        abci::grpc::AbciinterfaceGrpc::connect(&url)
            .map_err(|_| "failed to connect")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci instance")
    .unwrap();

    let mut client = abci::get_abci_instance().unwrap();
    let set_option_result = client.set_option("my_opt", "yes");

    assert_eq!(set_option_result.unwrap().get_code(), 0);
}