//! web-capture component-library provider.
//!
//! Issue #3 (R3/R4) asks `web-search` to use `link-assistant/web-capture` as a
//! component library so this project can focus on search aggregation rather than
//! re-implementing per-provider scraping. Mirrors the JavaScript
//! `src/providers/web-capture.js`.
//!
//! There is no published `web-capture` Rust crate yet, so this provider is an
//! optional integration point that degrades gracefully: it warns once and
//! returns no results, exactly like the JS provider does when the optional
//! `@link-assistant/web-capture` dependency is absent. The provider, its id
//! namespace (`wc:<engine>`), and registry metadata are kept in full parity so
//! that wiring a real crate later is a drop-in change.

use std::sync::atomic::{AtomicBool, Ordering};

use async_trait::async_trait;

use super::base::{SearchOptions, SearchProvider, SearchResult};
use crate::error::SearchError;

/// Providers exposed by web-capture's search contract.
pub const SUPPORTED_PROVIDERS: [&str; 5] = ["wikipedia", "duckduckgo", "google", "bing", "brave"];

static WARNED: AtomicBool = AtomicBool::new(false);

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
        _options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        if !WARNED.swap(true, Ordering::Relaxed) {
            tracing::warn!(
                "WebCaptureProvider: the web-capture component library is not wired in the Rust \
                 build yet; wc:* providers return no results until it is available."
            );
        }
        Ok(Vec::new())
    }
}
