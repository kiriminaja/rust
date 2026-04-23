mod common;

use common::new_mock_client;
use serde_json::json;
use wiremock::matchers::{any, body_json, body_string, header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

use kiriminaja::types::{
    ExpressService, InstantPickupItem, InstantPickupPackage, InstantPickupPayload, InstantService,
    InstantVehicle, PricingExpressPayload, PricingInstantLocationPayload, PricingInstantPayload,
    RequestPickupItem, RequestPickupItemMetadata, RequestPickupPackage, RequestPickupPayload,
};
use kiriminaja::{Client, Config, Env, Error};

// --------------------------------------------------------------------
// Helpers
// --------------------------------------------------------------------

fn ok_json() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_string(common::DEFAULT_OK_BODY)
}

async fn install_default(server: &wiremock::MockServer) {
    Mock::given(any()).respond_with(ok_json()).mount(server).await;
}

async fn last_request(server: &wiremock::MockServer) -> wiremock::Request {
    server
        .received_requests()
        .await
        .expect("recording is enabled")
        .pop()
        .expect("at least one request was made")
}

// --------------------------------------------------------------------
// Base URL & headers
// --------------------------------------------------------------------

#[tokio::test]
async fn sandbox_base_url_is_default() {
    let cfg = Config {
        env: Env::Sandbox,
        ..Default::default()
    };
    assert_eq!(cfg.env.base_url(), "https://tdev.kiriminaja.com");
}

#[tokio::test]
async fn production_base_url() {
    assert_eq!(Env::Production.base_url(), "https://client.kiriminaja.com");
}

#[tokio::test]
async fn bearer_token_and_accept_header_are_set() {
    let (client, server) = new_mock_client("abc").await;
    install_default(&server).await;

    client.address.provinces().await.unwrap();

    let req = last_request(&server).await;
    assert_eq!(req.headers.get("authorization").unwrap(), "Bearer abc");
    assert_eq!(req.headers.get("accept").unwrap(), "application/json");
}

#[tokio::test]
async fn no_authorization_header_when_api_key_empty() {
    let (client, server) = new_mock_client("").await;
    install_default(&server).await;

    client.address.provinces().await.unwrap();

    let req = last_request(&server).await;
    assert!(req.headers.get("authorization").is_none());
}

// --------------------------------------------------------------------
// Address
// --------------------------------------------------------------------

#[tokio::test]
async fn provinces_endpoint() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/province"))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.address.provinces().await.unwrap();
}

#[tokio::test]
async fn cities_endpoint_sends_provinsi_id() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/city"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({ "provinsi_id": 5 })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.address.cities(5).await.unwrap();
}

#[tokio::test]
async fn districts_endpoint_sends_kabupaten_id() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/kecamatan"))
        .and(body_json(json!({ "kabupaten_id": 12 })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.address.districts(12).await.unwrap();
}

#[tokio::test]
async fn sub_districts_endpoint_sends_kecamatan_id() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/kelurahan"))
        .and(body_json(json!({ "kecamatan_id": 77 })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.address.sub_districts(77).await.unwrap();
}

#[tokio::test]
async fn districts_by_name_endpoint() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/v2/get_address_by_name"))
        .and(body_json(json!({ "search": "jakarta" })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.address.districts_by_name("jakarta").await.unwrap();
}

// --------------------------------------------------------------------
// Coverage area & pricing
// --------------------------------------------------------------------

#[tokio::test]
async fn pricing_express_endpoint_sends_payload() {
    let (client, server) = new_mock_client("").await;
    let payload = PricingExpressPayload {
        origin: 1,
        destination: 2,
        weight: 1000,
        item_value: 5000,
        insurance: 0,
        courier: vec![ExpressService::Jne, ExpressService::Jnt],
    };

    Mock::given(method("POST"))
        .and(path("/api/mitra/v6.1/shipping_price"))
        .and(body_json(&payload))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.coverage_area.pricing_express(&payload).await.unwrap();
}

#[tokio::test]
async fn pricing_instant_endpoint_sends_payload() {
    let (client, server) = new_mock_client("").await;
    let payload = PricingInstantPayload {
        service: vec![InstantService::GrabExpress, InstantService::Gosend],
        item_price: 10_000.0,
        origin: PricingInstantLocationPayload {
            lat: -6.2,
            long: 106.8,
            address: "A".into(),
        },
        destination: PricingInstantLocationPayload {
            lat: -6.21,
            long: 106.81,
            address: "B".into(),
        },
        weight: 1000,
        vehicle: InstantVehicle::Bike,
        timezone: "Asia/Jakarta".into(),
    };

    Mock::given(method("POST"))
        .and(path("/api/mitra/v4/instant/pricing"))
        .and(body_json(&payload))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.coverage_area.pricing_instant(&payload).await.unwrap();
}

#[tokio::test]
async fn coverage_area_delegates_address_methods() {
    let (client, server) = new_mock_client("").await;
    Mock::given(any()).respond_with(ok_json()).mount(&server).await;

    client.coverage_area.provinces().await.unwrap();
    client.coverage_area.cities(1).await.unwrap();
    client.coverage_area.districts(1).await.unwrap();
    client.coverage_area.sub_districts(1).await.unwrap();
    client.coverage_area.districts_by_name("test").await.unwrap();

    let reqs = server.received_requests().await.unwrap();
    let paths: Vec<&str> = reqs.iter().map(|r| r.url.path()).collect();
    assert!(paths.contains(&"/api/mitra/province"));
    assert!(paths.contains(&"/api/mitra/city"));
    assert!(paths.contains(&"/api/mitra/kecamatan"));
    assert!(paths.contains(&"/api/mitra/kelurahan"));
    assert!(paths.contains(&"/api/mitra/v2/get_address_by_name"));
}

// --------------------------------------------------------------------
// Order — Express
// --------------------------------------------------------------------

#[tokio::test]
async fn express_track_endpoint() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/tracking"))
        .and(body_json(json!({ "order_id": "OID_EXP_1" })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.order.express.track("OID_EXP_1").await.unwrap();
}

#[tokio::test]
async fn express_cancel_uses_query_params() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/v3/cancel_shipment"))
        .and(query_param("awb", "AWB123"))
        .and(query_param("reason", "reason here"))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client
        .order
        .express
        .cancel("AWB123", "reason here")
        .await
        .unwrap();
}

#[tokio::test]
async fn express_request_pickup_endpoint() {
    let (client, server) = new_mock_client("").await;
    let payload = RequestPickupPayload {
        address: "Jl. Jodipati No.29".into(),
        phone: "08133345678".into(),
        name: "Tokotries".into(),
        kecamatan_id: 548,
        schedule: "2021-11-30 22:00:00".into(),
        packages: vec![RequestPickupPackage {
            order_id: "YGL-000000019".into(),
            destination_name: "Flag Test".into(),
            destination_phone: "082223323333".into(),
            destination_address: "Jl. Magelang KM 11".into(),
            destination_kecamatan_id: 548,
            weight: 520,
            width: 8,
            length: 8,
            height: 8,
            item_value: 275_000,
            shipping_cost: 65_000,
            service: "jne".into(),
            service_type: "REG23".into(),
            cod: 0,
            package_type_id: 7,
            item_name: "TEST Item name".into(),
            items: vec![RequestPickupItem {
                name: "Kaos Polos".into(),
                price: 125_000,
                qty: 2,
                weight: 260,
                width: 4,
                length: 4,
                height: 4,
                metadata: Some(RequestPickupItemMetadata {
                    sku: "KP-001".into(),
                    variant_label: "Merah / L".into(),
                }),
            }],
            ..Default::default()
        }],
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/mitra/v6.1/request_pickup"))
        .and(body_json(&payload))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.order.express.request_pickup(&payload).await.unwrap();
}

// --------------------------------------------------------------------
// Order — Instant
// --------------------------------------------------------------------

#[tokio::test]
async fn instant_track_uses_get_with_path_param() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("GET"))
        .and(path("/api/mitra/v4/instant/tracking/OID123"))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.order.instant.track("OID123").await.unwrap();
}

#[tokio::test]
async fn instant_create_endpoint() {
    let (client, server) = new_mock_client("").await;
    let payload = InstantPickupPayload {
        service: InstantService::Gosend,
        service_type: "instant".into(),
        vehicle: InstantVehicle::Bike,
        order_prefix: "BDI".into(),
        packages: vec![InstantPickupPackage {
            origin_name: "Rizky".into(),
            origin_phone: "081280045616".into(),
            origin_lat: -7.854584,
            origin_long: 110.331154,
            origin_address: "Wirobrajan, Yogyakarta".into(),
            origin_address_note: "Dekat Kantor".into(),
            destination_name: "Okka".into(),
            destination_phone: "081280045616".into(),
            destination_lat: -7.776192,
            destination_long: 110.325053,
            destination_address: "Godean, Sleman".into(),
            destination_address_note: "Dekat Pasar".into(),
            shipping_price: 34_000,
            item: InstantPickupItem {
                name: "Barang 1".into(),
                description: "Barang 1 Description".into(),
                price: 20_000,
                weight: 1000,
            },
        }],
    };

    Mock::given(method("POST"))
        .and(path("/api/mitra/v4/instant/pickup/request"))
        .and(body_json(&payload))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.order.instant.create(&payload).await.unwrap();
}

#[tokio::test]
async fn instant_find_new_driver_endpoint() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/v4/instant/pickup/find-new-driver"))
        .and(body_json(json!({ "order_id": "OID123" })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client
        .order
        .instant
        .find_new_driver("OID123")
        .await
        .unwrap();
}

#[tokio::test]
async fn instant_cancel_uses_delete() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("DELETE"))
        .and(path("/api/mitra/v4/instant/pickup/void/OID123"))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.order.instant.cancel("OID123").await.unwrap();
}

// --------------------------------------------------------------------
// Courier
// --------------------------------------------------------------------

#[tokio::test]
async fn courier_list_endpoint() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/couriers"))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.courier.list().await.unwrap();
}

#[tokio::test]
async fn courier_group_endpoint_has_no_json_body() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/couriers_group"))
        .and(body_string(""))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.courier.group().await.unwrap();

    let req = last_request(&server).await;
    // No JSON body, so Content-Type should not be set to application/json.
    let ct = req
        .headers
        .get("content-type")
        .map(|v| v.to_str().unwrap_or(""))
        .unwrap_or("");
    assert!(!ct.contains("application/json"));
}

#[tokio::test]
async fn courier_detail_endpoint() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/courier_services"))
        .and(body_json(json!({ "courier_code": "jne" })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.courier.detail("jne").await.unwrap();
}

#[tokio::test]
async fn courier_set_whitelist_services_endpoint() {
    let (client, server) = new_mock_client("").await;
    let services = vec!["jne_reg".to_string(), "jne_yes".to_string()];
    Mock::given(method("POST"))
        .and(path("/api/mitra/v3/set_whitelist_services"))
        .and(body_json(json!({ "services": services })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client
        .courier
        .set_whitelist_services(&services)
        .await
        .unwrap();
}

// --------------------------------------------------------------------
// Pickup
// --------------------------------------------------------------------

#[tokio::test]
async fn pickup_schedules_endpoint() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/v2/schedules"))
        .and(body_string(""))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.pickup.schedules().await.unwrap();
}

// --------------------------------------------------------------------
// Payment
// --------------------------------------------------------------------

#[tokio::test]
async fn payment_get_payment_endpoint() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("POST"))
        .and(path("/api/mitra/v2/get_payment"))
        .and(body_json(json!({ "payment_id": "PAY123" })))
        .respond_with(ok_json())
        .expect(1)
        .mount(&server)
        .await;

    client.payment.get_payment("PAY123").await.unwrap();
}

// --------------------------------------------------------------------
// Credit
// --------------------------------------------------------------------

#[tokio::test]
async fn credit_balance_endpoint_decodes_response() {
    let (client, server) = new_mock_client("").await;
    Mock::given(method("GET"))
        .and(path("/api/mitra/v6.2/credit/balance"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"status":true,"text":"ok","method":"GET","code":"200","data":{"balance":125000}}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let resp = client.credit.balance().await.unwrap();
    assert!(resp.status);
    assert_eq!(resp.code, "200");
    assert_eq!(resp.data.balance, 125_000.0);
}

// --------------------------------------------------------------------
// Validation (negative tests) — should NOT hit the network
// --------------------------------------------------------------------

#[tokio::test]
async fn validation_empty_express_order_id() {
    let client = Client::new(Config::default());
    let err = client.order.express.track("").await.unwrap_err();
    assert!(matches!(err, Error::InvalidArgument(_)));
}

#[tokio::test]
async fn validation_empty_awb() {
    let client = Client::new(Config::default());
    let err = client.order.express.cancel("", "reason").await.unwrap_err();
    assert!(matches!(err, Error::InvalidArgument(_)));
}

#[tokio::test]
async fn validation_empty_instant_order_id() {
    let client = Client::new(Config::default());
    assert!(matches!(
        client.order.instant.track("").await.unwrap_err(),
        Error::InvalidArgument(_)
    ));
    assert!(matches!(
        client.order.instant.cancel("").await.unwrap_err(),
        Error::InvalidArgument(_)
    ));
    assert!(matches!(
        client.order.instant.find_new_driver("").await.unwrap_err(),
        Error::InvalidArgument(_)
    ));
}

#[tokio::test]
async fn validation_empty_payment_id() {
    let client = Client::new(Config::default());
    assert!(matches!(
        client.payment.get_payment("").await.unwrap_err(),
        Error::InvalidArgument(_)
    ));
}

#[tokio::test]
async fn validation_empty_courier_code() {
    let client = Client::new(Config::default());
    assert!(matches!(
        client.courier.detail("").await.unwrap_err(),
        Error::InvalidArgument(_)
    ));
}

#[tokio::test]
async fn validation_empty_services_slice() {
    let client = Client::new(Config::default());
    assert!(matches!(
        client
            .courier
            .set_whitelist_services(&[])
            .await
            .unwrap_err(),
        Error::InvalidArgument(_)
    ));
}

#[tokio::test]
async fn validation_zero_provinsi_id() {
    let client = Client::new(Config::default());
    assert!(matches!(
        client.address.cities(0).await.unwrap_err(),
        Error::InvalidArgument(_)
    ));
}

#[tokio::test]
async fn validation_empty_search_query() {
    let client = Client::new(Config::default());
    assert!(matches!(
        client.address.districts_by_name("").await.unwrap_err(),
        Error::InvalidArgument(_)
    ));
}

#[tokio::test]
async fn validation_request_pickup_missing_fields() {
    let client = Client::new(Config::default());
    let err = client
        .order
        .express
        .request_pickup(&RequestPickupPayload::default())
        .await
        .unwrap_err();
    assert!(matches!(err, Error::InvalidArgument(_)));
}

// --------------------------------------------------------------------
// Error handling on non-2xx and malformed responses
// --------------------------------------------------------------------

#[tokio::test]
async fn api_error_on_401() {
    let (client, server) = new_mock_client("").await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(401).set_body_string(r#"{"message":"Unauthorized"}"#))
        .mount(&server)
        .await;

    let err = client.address.provinces().await.unwrap_err();
    match err {
        Error::Api { status, body, .. } => {
            assert_eq!(status, 401);
            assert!(body.contains("Unauthorized"));
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn api_error_on_500() {
    let (client, server) = new_mock_client("").await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(500).set_body_string("<html>Internal Server Error</html>"))
        .mount(&server)
        .await;

    let err = client.address.provinces().await.unwrap_err();
    match err {
        Error::Api { status, .. } => assert_eq!(status, 500),
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn malformed_json_response_is_decode_error() {
    let (client, server) = new_mock_client("").await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;

    let err = client.address.provinces().await.unwrap_err();
    assert!(matches!(err, Error::Decode(_)));
}
