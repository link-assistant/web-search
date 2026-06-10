//! Generic, descriptor-driven search provider.
//!
//! A single provider implementation that can speak to any engine described by a
//! catalog [`EngineDescriptor`](super::engines::EngineDescriptor). This keeps
//! the fetch/normalize/error plumbing in one place while each engine only
//! declares its URL, request kind, and parser. Mirrors the JavaScript
//! `src/providers/generic.js` (issue #3 parity requirement).

use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE};
use reqwest::Method;

use super::base::{SearchOptions, SearchProvider, SearchResult};
use super::engines::{EngineDescriptor, EngineKind, HttpMethod};
use crate::error::SearchError;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                          (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// Descriptor-driven provider.
pub struct GenericProvider {
    descriptor: EngineDescriptor,
    enabled: bool,
    weight: f64,
    client: reqwest::Client,
}

impl GenericProvider {
    /// Create a new provider for the given engine descriptor.
    pub fn new(descriptor: EngineDescriptor) -> Self {
        Self {
            descriptor,
            enabled: true,
            weight: 1.0,
            client: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// The engine descriptor backing this provider.
    pub fn descriptor(&self) -> &EngineDescriptor {
        &self.descriptor
    }

    fn build_headers(&self, options: &SearchOptions) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        let accept = match self.descriptor.kind {
            EngineKind::Json => "application/json",
            EngineKind::Text | EngineKind::Html => {
                "text/html,application/xhtml+xml,application/xml;q=0.9"
            }
        };
        headers.insert(ACCEPT, HeaderValue::from_static(accept));

        if let Some(extra) = self.descriptor.headers {
            for (name, value) in extra(options) {
                if let (Ok(name), Ok(value)) = (
                    HeaderName::from_bytes(name.as_bytes()),
                    HeaderValue::from_str(&value),
                ) {
                    headers.insert(name, value);
                }
            }
        }
        headers
    }
}

#[async_trait]
impl SearchProvider for GenericProvider {
    fn name(&self) -> &str {
        self.descriptor.id
    }

    fn is_available(&self) -> bool {
        self.enabled
    }

    fn weight(&self) -> f64 {
        self.weight
    }

    fn set_weight(&mut self, weight: f64) {
        self.weight = weight.clamp(0.0, 1.0);
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    async fn search(
        &self,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let d = &self.descriptor;
        let limit = options.limit.unwrap_or(10);
        let url = (d.build_url)(query, options);
        let mut headers = self.build_headers(options);

        let method = match d.method {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
        };

        let mut request = self.client.request(method, &url);
        if let (HttpMethod::Post, Some(build_body)) = (d.method, d.build_body) {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
            request = request.body((build_body)(query, options));
        }
        request = request.headers(headers);

        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                tracing::error!("{} search error: {}", d.id, error);
                return Ok(Vec::new());
            }
        };

        if !response.status().is_success() {
            tracing::error!("{} returned status {}", d.id, response.status());
            return Ok(Vec::new());
        }

        match response.text().await {
            Ok(body) => Ok((d.parse)(&body, limit, options)),
            Err(error) => {
                tracing::error!("{} body read error: {}", d.id, error);
                Ok(Vec::new())
            }
        }
    }
}
