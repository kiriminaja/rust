use std::time::Duration;

/// Target environment for the KiriminAja API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Env {
    #[default]
    Sandbox,
    Production,
}

impl Env {
    pub fn base_url(&self) -> &'static str {
        match self {
            Env::Sandbox => "https://tdev.kiriminaja.com",
            Env::Production => "https://client.kiriminaja.com",
        }
    }
}

/// SDK configuration. Mirrors `kiriminaja.Config` in the Go SDK.
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub env: Env,
    /// Override the base URL. When empty, derived from `env`.
    pub base_url: Option<String>,
    pub api_key: String,
    /// Request timeout. Defaults to 30 seconds when `None`.
    pub timeout: Option<Duration>,
    /// Pre-built reqwest client. When `Some`, `timeout` is ignored.
    pub http_client: Option<reqwest::Client>,
}
