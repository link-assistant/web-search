//! Search provider implementations

mod base;
mod bing;
mod duckduckgo;
mod google;

pub use base::{SearchOptions, SearchProvider, SearchResult};
pub use bing::{BingConfig, BingProvider};
pub use duckduckgo::DuckDuckGoProvider;
pub use google::{GoogleConfig, GoogleProvider};
