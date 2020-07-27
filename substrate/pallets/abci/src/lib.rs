use abci_proto::abci_application_client::AbciApplicationClient;
use abci_proto::RequestEcho;
use std::time::Duration;
use tokio::runtime::Runtime;

pub mod abci_proto {
    tonic::include_proto!("tendermint.abci.types");
}

pub fn send_test_method(abci_endpoint: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = Runtime::new()?;

    // Get server URL from ENV variable and translate it into static str
    let endpoint: &'static str = Box::leak(abci_endpoint.into_boxed_str());
    let client = AbciApplicationClient::connect(endpoint);
    let mut client = rt.block_on(async move {
        tokio::time::timeout(Duration::from_secs(1), client)
            .await
            .expect("failed to set timeout for future")
    })?;

    let request = tonic::Request::new(RequestEcho {
        message: "Hello".to_owned(),
    });

    let req = client.echo(request);
    let response = rt.block_on(async move {
        tokio::time::timeout(Duration::from_secs(1), req)
            .await
            .expect("failed to set timeout for future")
    })?;

    println!("RESPONSE={:?}", response);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tendermint_req() {
        // let result = send_test_method("http://0.0.0.0:8081".to_owned());
        let result = send_test_method("tcp://127.0.0.1:26658".to_owned());
        println!("result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
