//! Caller-owned HTTP transport contract and exact response captures.

use std::collections::BTreeMap;

use async_trait::async_trait;
use serde::Serialize;

use crate::error::SearchError;

/// Transport-neutral HTTP request produced by a search provider.
#[derive(Debug, Clone)]
pub struct TransportRequest {
    /// HTTP method.
    pub method: String,
    /// Fully resolved request URL.
    pub url: String,
    /// Request headers.
    pub headers: BTreeMap<String, String>,
    /// Optional exact request body.
    pub body: Option<Vec<u8>>,
}

/// Exact transport response retained for provenance and parsing.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportResponse {
    /// Final response URL.
    pub url: String,
    /// HTTP status code.
    pub status: u16,
    /// Response headers.
    pub headers: BTreeMap<String, String>,
    /// Exact response bytes.
    pub body: Vec<u8>,
    /// Optional opaque receipt supplied by a caching transport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
}

/// Async transport implemented by callers that own caching, retries, or I/O.
#[async_trait]
pub trait SearchTransport: Send + Sync {
    /// Execute one provider request. Dropping this future must cancel its work.
    async fn execute(&self, request: TransportRequest) -> Result<TransportResponse, SearchError>;
}

/// Default reqwest-backed transport.
#[derive(Debug, Clone, Default)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    /// Wrap a caller-configured reqwest client.
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchTransport for ReqwestTransport {
    async fn execute(&self, request: TransportRequest) -> Result<TransportResponse, SearchError> {
        let method = reqwest::Method::from_bytes(request.method.as_bytes())
            .map_err(|error| SearchError::Transport(error.to_string()))?;
        let mut builder = self.client.request(method, &request.url);
        for (name, value) in request.headers {
            builder = builder.header(name, value);
        }
        if let Some(body) = request.body {
            builder = builder.body(body);
        }
        let response = builder.send().await?;
        let status = response.status().as_u16();
        let url = response.url().to_string();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|value| (name.to_string(), value.to_string()))
            })
            .collect();
        let body = response.bytes().await?.to_vec();
        Ok(TransportResponse {
            url,
            status,
            headers,
            body,
            receipt: None,
        })
    }
}
