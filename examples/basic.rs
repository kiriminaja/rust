use kiriminaja::{Client, Config, Env};

#[tokio::main]
async fn main() -> kiriminaja::Result<()> {
    let client = Client::new(Config {
        env: Env::Sandbox,
        api_key: std::env::var("KIRIMINAJA_API_KEY").unwrap_or_default(),
        ..Default::default()
    });

    let provinces = client.address.provinces().await?;
    println!("Provinces: {:#?}", provinces);

    Ok(())
}
