#![cfg(feature = "blocking")]

use kiriminaja::blocking::Client;
use kiriminaja::{Config, Env};
use serde_json::json;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_string(r#"{"status":true}"#)
}

fn new_client(server: &MockServer) -> Client {
    Client::new(Config {
        env: Env::Sandbox,
        base_url: Some(server.uri()),
        api_key: "test-key".into(),
        ..Default::default()
    })
}

#[test]
fn blocking_provinces() {
    // The runtime inside the blocking client is current-thread, so a separate
    // multi-thread runtime is needed to drive wiremock during setup.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let server = rt.block_on(async {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/mitra/province"))
            .respond_with(ok())
            .expect(1)
            .mount(&s)
            .await;
        s
    });

    let client = new_client(&server);
    client.address.provinces().expect("call should succeed");
}

#[test]
fn blocking_cities_sends_body() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let server = rt.block_on(async {
        let s = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/mitra/city"))
            .and(body_json(json!({ "provinsi_id": 7 })))
            .respond_with(ok())
            .expect(1)
            .mount(&s)
            .await;
        s
    });

    let client = new_client(&server);
    client.address.cities(7).expect("call should succeed");
}

#[test]
fn blocking_validation_returns_error_without_network() {
    let client = Client::new(Config::default());
    let err = client.order.express.track("").unwrap_err();
    assert!(matches!(err, kiriminaja::Error::InvalidArgument(_)));
}
