//! Web Search - Multi-provider web search aggregator
//!
//! A library for aggregating search results from multiple search engines
//! (Google, DuckDuckGo, Bing) with support for result merging and reranking.
//!
//! # Example
//!
//! ```no_run
//! use web_search::{WebSearchEngine, SearchOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let engine = WebSearchEngine::new();
//!     let results = engine.search("rust programming", SearchOptions::default()).await?;
//!
//!     for result in results {
//!         println!("{}: {}", result.title, result.url);
//!     }
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod merger;
pub mod providers;
pub mod search;

pub use error::SearchError;
pub use merger::{MergeOptions, MergeStrategy};
pub use providers::{
    get_default_provider_ids, get_provider_ids, get_registry, is_known_category, RegistryEntry,
    SearchOptions, SearchResult, CATEGORIES,
};
pub use search::{WebSearchConfig, WebSearchEngine};
