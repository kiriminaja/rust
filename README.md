# KiriminAja Rust SDK

[![Crates.io](https://img.shields.io/crates/v/kiriminaja)](https://crates.io/crates/kiriminaja)
[![Docs.rs](https://img.shields.io/docsrs/kiriminaja)](https://docs.rs/kiriminaja)
[![license](https://img.shields.io/crates/l/kiriminaja)](LICENSE)

Official Rust SDK for the [KiriminAja](https://kiriminaja.com) logistics API. Async-first (powered by `reqwest` + `tokio`) with an opt-in synchronous facade. The API surface mirrors the Go SDK so examples translate one-to-one.

## Requirements

- Rust 1.75+
- An async runtime compatible with `reqwest` (Tokio recommended)

## Installation

```bash
cargo add kiriminaja
```

Or in `Cargo.toml`:

```toml
[dependencies]
kiriminaja = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

---

## Quick Start

Create a client with `Client::new()`, then call any service method.

```rust
use kiriminaja::{Client, Config, Env};

#[tokio::main]
async fn main() -> kiriminaja::Result<()> {
    let client = Client::new(Config {
        env: Env::Sandbox, // or Env::Production
        api_key: std::env::var("KIRIMINAJA_API_KEY").unwrap_or_default(),
        ..Default::default()
    });

    // Use any service
    let provinces = client.address.provinces().await?;
    println!("{:#?}", provinces);
    Ok(())
}
```

---

## Config Options

| Field         | Type                      | Default            | Description                                  |
| ------------- | ------------------------- | ------------------ | -------------------------------------------- |
| `env`         | `Env`                     | `Env::Sandbox`     | Target environment                           |
| `api_key`     | `String`                  | _required_         | Your KiriminAja API key                      |
| `base_url`    | `Option<String>`          | derived from `env` | Override the base URL                        |
| `timeout`     | `Option<Duration>`        | 30s                | Request timeout (used when no `http_client`) |
| `http_client` | `Option<reqwest::Client>` | new client         | Bring-your-own `reqwest::Client`             |

```rust
use std::time::Duration;
use kiriminaja::{Client, Config, Env};

// Custom base URL
let client = Client::new(Config {
    base_url: Some("https://tdev.kiriminaja.com".into()),
    api_key: std::env::var("KIRIMINAJA_API_KEY").unwrap_or_default(),
    ..Default::default()
});

// Custom timeout
let client = Client::new(Config {
    api_key: "...".into(),
    timeout: Some(Duration::from_secs(10)),
    ..Default::default()
});

// Bring your own reqwest::Client (proxy / test mock)
let http = reqwest::Client::builder().user_agent("my-app/1.0").build().unwrap();
let client = Client::new(Config {
    api_key: "...".into(),
    http_client: Some(http),
    ..Default::default()
});
```

---

## Synchronous (blocking) API

For callers that don't use async, enable the `blocking` cargo feature:

```toml
[dependencies]
kiriminaja = { version = "0.1", features = ["blocking"] }
```

```rust
use kiriminaja::blocking::Client;
use kiriminaja::{Config, Env};

fn main() -> kiriminaja::Result<()> {
    let client = Client::new(Config {
        env: Env::Sandbox,
        api_key: std::env::var("KIRIMINAJA_API_KEY").unwrap_or_default(),
        ..Default::default()
    });

    let provinces = client.address.provinces()?; // no .await
    println!("{:#?}", provinces);
    Ok(())
}
```

The blocking client wraps the async client with a small internal Tokio runtime; the API surface is identical except methods are not `async`.

> ⚠️ **Do not call blocking methods from inside an async runtime.** The blocking facade calls `runtime.block_on(...)` internally, which Tokio rejects with: _"Cannot start a runtime from within a runtime."_ In async contexts (axum, actix-web, `#[tokio::main]`), use the async [`kiriminaja::Client`] directly.

---

## Services

All async methods return `kiriminaja::Result<T>`.

### Address

```rust
// List all provinces
client.address.provinces().await?;

// Cities in a province (provinsi_id)
client.address.cities(5).await?;

// Districts in a city (kabupaten_id)
client.address.districts(12).await?;

// Sub-districts in a district (kecamatan_id)
client.address.sub_districts(77).await?;

// Search districts by name
client.address.districts_by_name("jakarta").await?;
```

---

### Coverage Area & Pricing

```rust
use kiriminaja::types::{
    ExpressService, InstantService, InstantVehicle,
    PricingExpressPayload, PricingInstantLocationPayload, PricingInstantPayload,
};

// Express shipping rates
client.coverage_area.pricing_express(&PricingExpressPayload {
    origin: 1,
    destination: 2,
    weight: 1000, // grams
    item_value: 50_000,
    insurance: 0,
    courier: vec![ExpressService::Jne, ExpressService::Jnt],
}).await?;

// Instant (same-day) rates
client.coverage_area.pricing_instant(&PricingInstantPayload {
    service: vec![InstantService::Gosend],
    item_price: 10_000.0,
    origin: PricingInstantLocationPayload {
        lat: -6.2, long: 106.8, address: "Jl. Sudirman No.1".into(),
    },
    destination: PricingInstantLocationPayload {
        lat: -6.21, long: 106.81, address: "Jl. Thamrin No.5".into(),
    },
    weight: 1000,
    vehicle: InstantVehicle::Bike,
    timezone: "Asia/Jakarta".into(),
}).await?;
```

---

### Order — Express

```rust
use kiriminaja::types::{
    RequestPickupItem, RequestPickupItemMetadata,
    RequestPickupPackage, RequestPickupPayload,
};

// Track by order ID
client.order.express.track("ORDER123").await?;

// Cancel by AWB
client.order.express.cancel("AWB123456", "Customer request").await?;

// Request pickup
client.order.express.request_pickup(&RequestPickupPayload {
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
        weight: 520, width: 8, length: 8, height: 8,
        item_value: 275_000,
        shipping_cost: 65_000,
        service: "jne".into(),
        service_type: "REG23".into(),
        cod: 0,
        package_type_id: 7,
        item_name: "TEST Item name".into(),
        // `items` is optional. When provided, it lists the individual items
        // contained in the package. `item_value` is still required.
        items: vec![RequestPickupItem {
            name: "Kaos Polos".into(),
            price: 125_000,
            qty: 2,
            weight: 260,
            width: 4, length: 4, height: 4,
            metadata: Some(RequestPickupItemMetadata {
                sku: "KP-001".into(),
                variant_label: "Merah / L".into(),
            }),
        }],
        ..Default::default()
    }],
    ..Default::default()
}).await?;
```

---

### Order — Instant

```rust
use kiriminaja::types::{
    InstantPickupItem, InstantPickupPackage, InstantPickupPayload,
    InstantService, InstantVehicle,
};

// Create instant pickup
client.order.instant.create(&InstantPickupPayload {
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
}).await?;

// Find a new driver for an existing order
client.order.instant.find_new_driver("ORDER123").await?;

// Cancel instant order
client.order.instant.cancel("ORDER123").await?;

// Track instant order
client.order.instant.track("ORDER123").await?;
```

---

### Courier

```rust
// List available couriers
client.courier.list().await?;

// Courier groups
client.courier.group().await?;

// Courier service detail
client.courier.detail("jne").await?;

// Set whitelist services
client.courier.set_whitelist_services(&["jne_reg".into(), "jne_yes".into()]).await?;
```

---

### Pickup Schedules

```rust
client.pickup.schedules().await?;
```

---

### Payment

```rust
client.payment.get_payment("PAY123").await?;
```

---

### Credit

```rust
// Get the current KiriminAja credit balance
let balance = client.credit.balance().await?;
// balance.data.balance -> f64
```

---

### Utilities — Volumetric

Estimate the smallest bounding box (length × width × height) for a multi-item
package by trying vertical / horizontal / side-by-side stacking and returning
the arrangement with the smallest volume.

```rust
use kiriminaja::utils::volumetric::{self, Item};

let dim = volumetric::calculate(&[
    Item { qty: 2, length: 10.0, width: 10.0, height: 2.0 },
    Item { qty: 1, length: 5.0,  width: 5.0,  height: 5.0 },
]);
// dim.length, dim.width, dim.height
```

---

## Error handling

All methods return `kiriminaja::Result<T>` where `Error` distinguishes invalid
arguments, transport errors, decoding errors, and non-2xx API responses
(`Error::Api { status, body, .. }`).

---

## Development

```bash
cargo build
cargo test
cargo run --example basic
```

## License

MIT
