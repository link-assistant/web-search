//! Generic, descriptor-driven search provider.
//!
//! A single provider implementation that can speak to any engine described by a
//! catalog [`EngineDescriptor`](super::engines::EngineDescriptor). This keeps
//! the fetch/normalize/error plumbing in one place while each engine only
//! declares its URL, request kind, and parser. Mirrors the JavaScript
//! `src/providers/generic.js` (issue #3 parity requirement).

use async_trait::async_trait;
use std::collections::BTreeMap;

use super::base::{SearchOptions, SearchProvider, SearchResult};
use super::engines::{EngineDescriptor, EngineKind, HttpMethod};
use crate::error::SearchError;
use crate::transport::{ReqwestTransport, SearchTransport, TransportRequest};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                          (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// Descriptor-driven provider.
pub struct GenericProvider {
    descriptor: EngineDescriptor,
    enabled: bool,
    weight: f64,
}

impl GenericProvider {
    /// Create a new provider for the given engine descriptor.
    pub fn new(descriptor: EngineDescriptor) -> Self {
        Self {
            descriptor,
            enabled: true,
            weight: 1.0,
        }
    }

    /// The engine descriptor backing this provider.
    pub fn descriptor(&self) -> &EngineDescriptor {
        &self.descriptor
    }

    fn build_headers(&self, options: &SearchOptions) -> BTreeMap<String, String> {
        let mut headers = BTreeMap::new();
        headers.insert("User-Agent".to_string(), USER_AGENT.to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9".to_string());
        let accept = match self.descriptor.kind {
            EngineKind::Json => "application/json",
            EngineKind::Text | EngineKind::Html => {
                "text/html,application/xhtml+xml,application/xml;q=0.9"
            }
        };
        headers.insert("Accept".to_string(), accept.to_string());

        if let Some(extra) = self.descriptor.headers {
            for (name, value) in extra(options) {
                headers.insert(name, value);
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
        self.search_with_transport(query, options, &ReqwestTransport::default())
            .await
    }

    async fn search_with_transport(
        &self,
        query: &str,
        options: &SearchOptions,
        transport: &dyn SearchTransport,
    ) -> Result<Vec<SearchResult>, SearchError> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let d = &self.descriptor;
        let limit = options.limit.unwrap_or(10);
        let url = (d.build_url)(query, options);
        let mut headers = self.build_headers(options);

        let method = match d.method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
        };
        let mut body = None;
        if let (HttpMethod::Post, Some(build_body)) = (d.method, d.build_body) {
            headers.insert(
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            );
            body = Some((build_body)(query, options).into_bytes());
        }
        let response = transport
            .execute(TransportRequest {
                method: method.to_string(),
                url,
                headers,
                body,
            })
            .await?;

        if !(200..300).contains(&response.status) {
            return Err(SearchError::ApiError {
                provider: d.id.to_string(),
                message: format!("HTTP {}", response.status),
            });
        }
        let body = String::from_utf8_lossy(&response.body);
        Ok((d.parse)(&body, limit, options))
    }
}
