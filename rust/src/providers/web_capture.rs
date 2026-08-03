//! web-capture component-library provider.
//!
//! Issue #3 (R3/R4) asks `web-search` to use `link-assistant/web-capture` as a
//! component library so this project can focus on search aggregation rather than
//! re-implementing per-provider scraping. Mirrors the JavaScript
//! `src/providers/web-capture.js`.
//!
//! The Rust provider delegates to the published `web-capture` crate and keeps
//! the same graceful empty-result behavior the JavaScript provider uses for
//! component errors.

use async_trait::async_trait;
use std::collections::BTreeMap;

use super::base::{SearchOptions, SearchProvider, SearchResult};
use crate::error::SearchError;
use crate::transport::{ReqwestTransport, SearchTransport, TransportRequest};

/// Providers exposed by web-capture's search contract.
pub const SUPPORTED_PROVIDERS: [&str; 5] = web_capture::SEARCH_PROVIDERS;

/// Provider that delegates to the web-capture component library.
pub struct WebCaptureProvider {
    name: String,
    engine: String,
    enabled: bool,
    weight: f64,
}

impl WebCaptureProvider {
    /// Create a provider bound to a web-capture engine (default `wikipedia`).
    pub fn new(engine: impl Into<String>) -> Self {
        let engine = engine.into();
        Self {
            name: format!("wc:{engine}"),
            engine,
            enabled: true,
            weight: 1.0,
        }
    }

    /// The web-capture engine this provider delegates to.
    pub fn engine(&self) -> &str {
        &self.engine
    }

    /// Adapt normalized web-capture items into the web-search result contract.
    pub fn adapt_items(&self, items: Vec<web_capture::SearchResultItem>) -> Vec<SearchResult> {
        items
            .into_iter()
            .enumerate()
            .filter_map(|(index, item)| {
                if item.url.trim().is_empty() {
                    return None;
                }

                Some(SearchResult {
                    title: if item.title.trim().is_empty() {
                        "Untitled".to_string()
                    } else {
                        item.title
                    },
                    url: item.url,
                    snippet: item.snippet,
                    source: self.name.clone(),
                    rank: if item.rank == 0 { index + 1 } else { item.rank },
                    score: None,
                    sources: None,
                })
            })
            .collect()
    }
}

impl Default for WebCaptureProvider {
    fn default() -> Self {
        Self::new("wikipedia")
    }
}

#[async_trait]
impl SearchProvider for WebCaptureProvider {
    fn name(&self) -> &str {
        &self.name
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

        let limit = options.limit.unwrap_or(web_capture::DEFAULT_LIMIT);

        let url = web_capture::search::build_search_url(&self.engine, query, limit).map_err(
            |message| SearchError::ApiError {
                provider: self.name.clone(),
                message,
            },
        )?;
        let response = transport
            .execute(TransportRequest {
                method: "GET".to_string(),
                url,
                headers: BTreeMap::from([(
                    "User-Agent".to_string(),
                    "Mozilla/5.0 (compatible; web-search/0.3)".to_string(),
                )]),
                body: None,
            })
            .await?;
        if !(200..300).contains(&response.status) {
            return Err(SearchError::ApiError {
                provider: self.name.clone(),
                message: format!("HTTP {}", response.status),
            });
        }
        let body = String::from_utf8_lossy(&response.body);
        let (items, blocked) =
            web_capture::search::parse_search_results(&self.engine, &body, limit);
        if blocked {
            return Err(SearchError::ApiError {
                provider: self.name.clone(),
                message: "provider returned a CAPTCHA page".to_string(),
            });
        }
        Ok(self.adapt_items(items))
    }
}
