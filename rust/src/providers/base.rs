//! Base provider trait and common types

use crate::error::SearchError;
use async_trait::async_trait;

pub use crate::SearchResult;

/// Options for search queries
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Maximum number of results to return
    pub limit: Option<usize>,
    /// Language code (e.g., "en", "de")
    pub language: Option<String>,
    /// Region code (e.g., "us", "de")
    pub region: Option<String>,
    /// Enable safe search filtering
    pub safe_search: Option<bool>,
}

/// Trait that all search providers must implement
#[async_trait]
pub trait SearchProvider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;

    /// Check if the provider is available/enabled
    fn is_available(&self) -> bool;

    /// Get the provider weight for reranking
    fn weight(&self) -> f64;

    /// Set the provider weight for reranking
    fn set_weight(&mut self, weight: f64);

    /// Enable or disable the provider
    fn set_enabled(&mut self, enabled: bool);

    /// Perform a search
    async fn search(
        &self,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError>;
}
