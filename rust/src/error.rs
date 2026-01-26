//! Error types for web search operations

use thiserror::Error;

/// Errors that can occur during web search operations
#[derive(Error, Debug)]
pub enum SearchError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Failed to parse HTML response
    #[error("Failed to parse HTML: {0}")]
    ParseError(String),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),

    /// Provider not found
    #[error("Unknown provider: {0}")]
    UnknownProvider(String),

    /// Provider is disabled
    #[error("Provider {0} is disabled")]
    ProviderDisabled(String),

    /// API error from search provider
    #[error("API error from {provider}: {message}")]
    ApiError { provider: String, message: String },

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Result type alias for search operations
pub type SearchResult<T> = std::result::Result<T, SearchError>;
