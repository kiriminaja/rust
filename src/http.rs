use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use reqwest::{header, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::Config;
use crate::error::{Error, Result};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Internal HTTP client shared by all services. Mirrors `kahttp.Client` in Go.
#[derive(Debug, Clone)]
pub struct HttpClient {
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) http: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct RequestOptions<'a, B: Serialize + ?Sized> {
    pub method: Option<Method>,
    pub query: Option<HashMap<&'a str, String>>,
    pub body: Option<&'a B>,
    pub headers: Option<HashMap<&'a str, &'a str>>,
}

impl<'a, B: Serialize + ?Sized> Default for RequestOptions<'a, B> {
    fn default() -> Self {
        Self {
            method: None,
            query: None,
            body: None,
            headers: None,
        }
    }
}

impl HttpClient {
    pub(crate) fn from_config(cfg: &Config) -> Self {
        let base_url = cfg
            .base_url
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| cfg.env.base_url().to_string());

        let http = cfg.http_client.clone().unwrap_or_else(|| {
            reqwest::Client::builder()
                .timeout(cfg.timeout.unwrap_or(DEFAULT_TIMEOUT))
                .build()
                .expect("failed to build reqwest client")
        });

        HttpClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: cfg.api_key.clone(),
            http,
        }
    }

    fn build_url(&self, path: &str) -> String {
        if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        }
    }

    pub async fn request_raw<B: Serialize + ?Sized>(
        &self,
        path: &str,
        opts: RequestOptions<'_, B>,
    ) -> Result<Vec<u8>> {
        let method = opts.method.unwrap_or(Method::POST);
        let url = self.build_url(path);

        let mut req = self.http.request(method.clone(), &url);

        if let Some(query) = &opts.query {
            req = req.query(&query.iter().collect::<Vec<_>>());
        }

        let has_body = opts.body.is_some() && method != Method::GET && method != Method::DELETE;
        if has_body {
            req = req.json(opts.body.unwrap());
        } else if method != Method::GET && method != Method::DELETE {
            // Ensure POST/PUT/PATCH without a body still sends `Content-Length: 0`.
            // Some KiriminAja edge proxies reject body-less POSTs otherwise (HTTP 411).
            req = req
                .body(reqwest::Body::from(Vec::<u8>::new()))
                .header(header::CONTENT_LENGTH, "0");
        }

        req = req.header(header::ACCEPT, "application/json");
        if !self.api_key.is_empty() {
            req = req.bearer_auth(&self.api_key);
        }
        if let Some(headers) = opts.headers {
            for (k, v) in headers {
                req = req.header(k, v);
            }
        }

        let resp = req.send().await?;
        let status = resp.status();
        let bytes = resp.bytes().await?;

        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                status_text: status.canonical_reason().unwrap_or("").to_string(),
                body: String::from_utf8_lossy(&bytes).to_string(),
            });
        }

        Ok(bytes.to_vec())
    }

    pub async fn request_json<T, B>(&self, path: &str, opts: RequestOptions<'_, B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let bytes = self.request_raw(path, opts).await?;
        let parsed = serde_json::from_slice::<T>(&bytes)?;
        Ok(parsed)
    }

    pub async fn post_json<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.request_json(
            path,
            RequestOptions {
                method: Some(Method::POST),
                body: Some(body),
                ..Default::default()
            },
        )
        .await
    }

    pub async fn post_empty<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json::<T, ()>(
            path,
            RequestOptions {
                method: Some(Method::POST),
                ..Default::default()
            },
        )
        .await
    }

    pub async fn get_json<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json::<T, ()>(
            path,
            RequestOptions {
                method: Some(Method::GET),
                ..Default::default()
            },
        )
        .await
    }

    pub async fn delete_json<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json::<T, ()>(
            path,
            RequestOptions {
                method: Some(Method::DELETE),
                ..Default::default()
            },
        )
        .await
    }
}

/// Convenience type alias used by services to share the HTTP client.
pub type SharedHttp = Arc<HttpClient>;
