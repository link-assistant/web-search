//! Web Search Engine - main entry point

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::error::SearchError;
use crate::merger::{merge_results, MergeOptions, MergeStrategy};
use crate::providers::{
    BingConfig, BingProvider, DuckDuckGoProvider, GoogleConfig, GoogleProvider, SearchOptions,
    SearchProvider, SearchResult,
};

/// Configuration for the web search engine
#[derive(Debug, Clone, Default)]
pub struct WebSearchConfig {
    /// Providers to use by default
    pub providers: Vec<String>,
    /// Google API key
    pub google_api_key: Option<String>,
    /// Google Custom Search Engine ID
    pub google_cx: Option<String>,
    /// Bing API key
    pub bing_api_key: Option<String>,
    /// Default weights for providers
    pub weights: HashMap<String, f64>,
    /// Default merge strategy
    pub merge_strategy: MergeStrategy,
}

impl WebSearchConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        Self {
            providers: vec![
                "duckduckgo".to_string(),
                "google".to_string(),
                "bing".to_string(),
            ],
            google_api_key: std::env::var("GOOGLE_API_KEY").ok(),
            google_cx: std::env::var("GOOGLE_CX").ok(),
            bing_api_key: std::env::var("BING_API_KEY").ok(),
            weights: HashMap::new(),
            merge_strategy: MergeStrategy::Rrf,
        }
    }
}

/// Web Search Engine
pub struct WebSearchEngine {
    providers: HashMap<String, Arc<RwLock<Box<dyn SearchProvider>>>>,
    default_providers: Vec<String>,
    default_weights: HashMap<String, f64>,
    default_strategy: MergeStrategy,
}

impl WebSearchEngine {
    /// Create a new web search engine with default configuration
    pub fn new() -> Self {
        Self::with_config(WebSearchConfig::from_env())
    }

    /// Create a new web search engine with custom configuration
    pub fn with_config(config: WebSearchConfig) -> Self {
        let mut providers: HashMap<String, Arc<RwLock<Box<dyn SearchProvider>>>> = HashMap::new();

        providers.insert(
            "duckduckgo".to_string(),
            Arc::new(RwLock::new(Box::new(DuckDuckGoProvider::new()))),
        );

        providers.insert(
            "google".to_string(),
            Arc::new(RwLock::new(Box::new(GoogleProvider::new(GoogleConfig {
                api_key: config.google_api_key,
                search_engine_id: config.google_cx,
            })))),
        );

        providers.insert(
            "bing".to_string(),
            Arc::new(RwLock::new(Box::new(BingProvider::new(BingConfig {
                api_key: config.bing_api_key,
            })))),
        );

        Self {
            providers,
            default_providers: config.providers,
            default_weights: config.weights,
            default_strategy: config.merge_strategy,
        }
    }

    /// Search across multiple providers
    pub async fn search(
        &self,
        query: &str,
        options: SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        self.search_with_options(query, options, None, None).await
    }

    /// Search with additional merge options
    pub async fn search_with_options(
        &self,
        query: &str,
        options: SearchOptions,
        providers: Option<Vec<String>>,
        merge_options: Option<MergeOptions>,
    ) -> Result<Vec<SearchResult>, SearchError> {
        if query.is_empty() {
            return Ok(Vec::new());
        }

        let providers_to_use = providers.unwrap_or_else(|| self.default_providers.clone());
        let merge_opts = merge_options.unwrap_or_else(|| MergeOptions {
            strategy: self.default_strategy,
            weights: self.default_weights.clone(),
            rrf_k: None,
            remove_duplicates: true,
        });

        let mut handles = Vec::new();

        for provider_name in &providers_to_use {
            let provider = self.providers.get(provider_name).cloned();
            if provider.is_none() {
                continue;
            }

            let provider = provider.unwrap();
            let query = query.to_string();
            let opts = options.clone();
            let name = provider_name.clone();

            handles.push(tokio::spawn(async move {
                let provider = provider.read().await;
                if !provider.is_available() {
                    return (name, Vec::new());
                }
                match provider.search(&query, &opts).await {
                    Ok(results) => (name, results),
                    Err(e) => {
                        tracing::error!("Provider {} failed: {}", name, e);
                        (name, Vec::new())
                    }
                }
            }));
        }

        let mut results_by_provider = HashMap::new();

        for handle in handles {
            if let Ok((name, results)) = handle.await {
                results_by_provider.insert(name, results);
            }
        }

        Ok(merge_results(&results_by_provider, &merge_opts))
    }

    /// Search with a single provider
    pub async fn search_single(
        &self,
        query: &str,
        provider_name: &str,
        options: SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| SearchError::UnknownProvider(provider_name.to_string()))?;

        let provider = provider.read().await;

        if !provider.is_available() {
            return Err(SearchError::ProviderDisabled(provider_name.to_string()));
        }

        provider.search(query, &options).await
    }

    /// Get available provider names
    pub fn get_available_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    /// Get provider status
    pub async fn get_provider_status(&self) -> HashMap<String, ProviderStatus> {
        let mut status = HashMap::new();

        for (name, provider) in &self.providers {
            let p = provider.read().await;
            status.insert(
                name.clone(),
                ProviderStatus {
                    enabled: p.is_available(),
                    weight: p.weight(),
                },
            );
        }

        status
    }

    /// Set provider weight
    pub async fn set_provider_weight(&self, name: &str, weight: f64) -> Result<(), SearchError> {
        let provider = self
            .providers
            .get(name)
            .ok_or_else(|| SearchError::UnknownProvider(name.to_string()))?;

        provider.write().await.set_weight(weight);
        Ok(())
    }

    /// Enable or disable a provider
    pub async fn set_provider_enabled(&self, name: &str, enabled: bool) -> Result<(), SearchError> {
        let provider = self
            .providers
            .get(name)
            .ok_or_else(|| SearchError::UnknownProvider(name.to_string()))?;

        provider.write().await.set_enabled(enabled);
        Ok(())
    }
}

impl Default for WebSearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Provider status information
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProviderStatus {
    /// Whether the provider is enabled
    pub enabled: bool,
    /// Provider weight for reranking
    pub weight: f64,
}
