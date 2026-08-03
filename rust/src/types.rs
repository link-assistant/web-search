//! Core data types shared by merging and network-backed search.

use serde::{Deserialize, Serialize};

/// A single search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The title of the search result.
    pub title: String,
    /// The URL of the search result.
    pub url: String,
    /// The description/snippet of the search result.
    pub snippet: String,
    /// The search provider that returned this result.
    pub source: String,
    /// The rank position in the original results (1-based).
    pub rank: usize,
    /// Computed score after merging (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// Sources that returned this result (after deduplication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
}
