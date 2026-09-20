// crates/adapters/nq/tests/integration_test.rs

mod mock_gateway;

use mock_gateway::MockGateway;
use nautilus_common::clients::{DataClient, ExecutionClient};
use nautilus_model::identifiers::{AccountId, ClientId, TraderId, Venue};
use nautilus_nq::{QuikDataClient, QuikExecutionClient};

#[tokio::test]
async fn test_execution_client_connect() {
    let mock = MockGateway::start().await;

    let client = QuikExecutionClient::create(
        TraderId::from("TESTER-001"),
        Venue::from("QUIK"),
        AccountId::from("QUIK-12345"),
        ClientId::from("QUIK-CLIENT"),
        mock.addr().to_string(),
    )
    .await
    .unwrap();

    assert!(client.is_connected());
    assert_eq!(client.venue(), Venue::from("QUIK"));

    mock.shutdown();
}

#[tokio::test]
async fn test_data_client_lifecycle() {
    let mut client = QuikDataClient::new(ClientId::from("QUIK-DATA"), Venue::from("QUIK"));

    assert!(!client.is_connected());

    client.start().unwrap();
    assert!(client.is_connected());

    client.stop().unwrap();
    assert!(!client.is_connected());
}
