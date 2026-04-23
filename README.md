# KiriminAja Rust SDK

Async Rust SDK for the [KiriminAja](https://kiriminaja.com)
that examples and call patterns translate one-to-one between the two.

## Requirements

- Rust 1.75+
- Tokio (or any async runtime compatible with `reqwest`)

## Installation
Instalation
```
cargo add kiriminaja --git https://github.com/kiriminaja/rust
```

```toml
[dependencies]
kiriminaja = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use kiriminaja::{Client, Config, Env};

#[tokio::main]
async fn main() -> kiriminaja::Result<()> {
    let client = Client::new(Config {
        env: Env::Sandbox, // or Env::Production
        api_key: std::env::var("KIRIMINAJA_API_KEY").unwrap_or_default(),
        ..Default::default()
    });

    let provinces = client.address.provinces().await?;
    println!("{:#?}", provinces);
    Ok(())
}
```

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

The blocking client wraps the async client with a small internal Tokio
runtime; the API surface is identical except methods are not `async`.

> ⚠️ **Caveat — do not call blocking methods from inside an async runtime.**
>
> The blocking facade calls `runtime.block_on(...)` internally. If you invoke
> it from within an existing Tokio runtime (for example an `axum`, `actix-web`,
> or `tokio::main` handler), Tokio will panic with:
>
> ```text
> Cannot start a runtime from within a runtime. This happens because a
> function (like `block_on`) attempted to block the current thread while
> the thread is being used to drive asynchronous tasks.
> ```
>
> In async contexts, always use the async [`kiriminaja::Client`] directly.
> The `blocking` facade is only meant for plain `fn main()` programs, CLIs,
> tests, scripts, and synchronous codebases that do not have an async runtime
> of their own.

## Config Options

| Field         | Type                      | Default              | Description                                  |
| ------------- | ------------------------- | -------------------- | -------------------------------------------- |
| `env`         | `Env`                     | `Env::Sandbox`       | Target environment.                          |
| `api_key`     | `String`                  | _required_           | Your KiriminAja API key.                     |
| `base_url`    | `Option<String>`          | derived from `env`   | Override the base URL.                       |
| `timeout`     | `Option<Duration>`        | 30s                  | Request timeout (used when no `http_client`).|
| `http_client` | `Option<reqwest::Client>` | new client           | Bring-your-own `reqwest` client.             |

## Services

API methods correspond directly to the Go SDK. All methods are `async` and
return `kiriminaja::Result<T>`.

### Address

```rust
client.address.provinces().await?;
client.address.cities(5).await?;            // provinsi_id
client.address.districts(12).await?;        // kabupaten_id
client.address.sub_districts(77).await?;    // kecamatan_id
client.address.districts_by_name("jakarta").await?;
```

### Coverage Area & Pricing

```rust
use kiriminaja::types::{
    ExpressService, InstantService, InstantVehicle,
    PricingExpressPayload, PricingInstantLocationPayload, PricingInstantPayload,
};

client.coverage_area.pricing_express(&PricingExpressPayload {
    origin: 1,
    destination: 2,
    weight: 1000,
    item_value: 50_000,
    insurance: 0,
    courier: vec![ExpressService::Jne, ExpressService::Jnt],
}).await?;

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

### Order — Express

```rust
use kiriminaja::types::{RequestPickupItem, RequestPickupItemMetadata, RequestPickupPackage, RequestPickupPayload};

client.order.express.track("ORDER123").await?;
client.order.express.cancel("AWB123456", "Customer request").await?;

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

### Order — Instant

```rust
use kiriminaja::types::{
    InstantPickupItem, InstantPickupPackage, InstantPickupPayload,
    InstantService, InstantVehicle,
};

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

client.order.instant.find_new_driver("ORDER123").await?;
client.order.instant.cancel("ORDER123").await?;
client.order.instant.track("ORDER123").await?;
```

### Courier

```rust
client.courier.list().await?;
client.courier.group().await?;
client.courier.detail("jne").await?;
client.courier.set_whitelist_services(&["jne_reg".into(), "jne_yes".into()]).await?;
```

### Pickup Schedules

```rust
client.pickup.schedules().await?;
```

### Payment

```rust
client.payment.get_payment("PAY123").await?;
```

### Credit

```rust
// Get the current KiriminAja credit balance
let balance = client.credit.balance().await?;
// balance.data.balance -> f64
```

## Error handling

All methods return `kiriminaja::Result<T>` where `Error` distinguishes invalid
arguments, transport errors, decoding errors, and non-2xx API responses
(`Error::Api { status, body, .. }`).

## Development

```bash
cargo build
cargo test
cargo run --example basic
```

## License

MIT
