//! Shared helpers for integration tests. Spins up a `wiremock` server and
//! returns a `Client` configured to talk to it, alongside the mock server
//! handle for setting up expectations and inspecting received requests.

use kiriminaja::{Client, Config, Env};
use wiremock::MockServer;

/// Default JSON body returned when a test does not register an explicit `Mock`.
pub const DEFAULT_OK_BODY: &str = r#"{"status":true}"#;

/// Build a fresh client + mock server pair. The client's `base_url` is
/// pointed at the mock server so all requests are intercepted.
pub async fn new_mock_client(api_key: &str) -> (Client, MockServer) {
    let server = MockServer::start().await;
    let client = Client::new(Config {
        env: Env::Sandbox,
        base_url: Some(server.uri()),
        api_key: api_key.to_string(),
        ..Default::default()
    });
    (client, server)
}
