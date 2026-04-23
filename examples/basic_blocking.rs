//! Same as `basic.rs` but using the synchronous facade.
//! Run with: `cargo run --features blocking --example basic_blocking`

use kiriminaja::blocking::Client;
use kiriminaja::{Config, Env};

fn main() -> kiriminaja::Result<()> {
    let client = Client::new(Config {
        env: Env::Sandbox,
        api_key: std::env::var("KIRIMINAJA_API_KEY").unwrap_or_default(),
        ..Default::default()
    });

    let provinces = client.address.provinces()?;
    println!("Provinces: {:#?}", provinces);

    Ok(())
}
