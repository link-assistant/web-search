//! Web Search - Multi-provider web search aggregator
//!
//! Deterministic result merging is always available. Network providers, the
//! search engine, CLI, and HTTP service are enabled by the default `server`
//! feature.
//!
//! # Example
//!
//! ```
//! use std::collections::HashMap;
//! use web_search::{merger::merge_results, MergeOptions, SearchResult};
//!
//! let results = HashMap::<String, Vec<SearchResult>>::new();
//! assert!(merge_results(&results, &MergeOptions::new()).is_empty());
//! ```

#![cfg_attr(not(feature = "server"), no_std)]

extern crate alloc;

#[cfg(feature = "server")]
pub mod error;
pub mod merger;
#[cfg(feature = "server")]
pub mod providers;
pub mod registry;
#[cfg(feature = "server")]
pub mod search;
#[cfg(feature = "server")]
pub mod transport;
mod types;

#[cfg(feature = "server")]
pub use error::SearchError;
pub use merger::{MergeOptions, MergeStrategy};
#[cfg(feature = "server")]
pub use providers::SearchOptions;
pub use registry::{
    get_default_provider_ids, get_provider_ids, get_registry, is_known_category,
    ProviderCapabilities, ProviderMetadata, RegistryEntry, CATEGORIES, PROVIDER_REGISTRY,
};
#[cfg(feature = "server")]
pub use search::{
    DetailedSearchResult, ProviderError, ProviderOutcome, ProviderOutcomeStatus, WebSearchConfig,
    WebSearchEngine,
};
#[cfg(feature = "server")]
pub use transport::{ReqwestTransport, SearchTransport, TransportRequest, TransportResponse};
pub use types::SearchResult;
