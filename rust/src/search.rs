//! Web Search Engine - main entry point

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future::join_all;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::error::SearchError;
use crate::merger::{merge_results, MergeOptions, MergeStrategy};
use crate::providers::{
    build_providers, get_default_provider_ids, get_registry, BuildConfig, RegistryEntry,
    SearchOptions, SearchProvider, SearchResult,
};
use crate::transport::{ReqwestTransport, SearchTransport, TransportRequest, TransportResponse};

struct RecordingTransport {
    inner: Arc<dyn SearchTransport>,
    responses: Arc<std::sync::Mutex<Vec<TransportResponse>>>,
}

#[async_trait]
impl SearchTransport for RecordingTransport {
    async fn execute(&self, request: TransportRequest) -> Result<TransportResponse, SearchError> {
        let response = self.inner.execute(request).await?;
        self.responses
            .lock()
            .expect("response capture mutex poisoned")
            .push(response.clone());
        Ok(response)
    }
}

/// Status of one provider in a detailed search.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderOutcomeStatus {
    /// Provider returned a valid result list (which may be empty).
    Success,
    /// Provider returned an error.
    Error,
    /// Provider is registered but disabled.
    Unavailable,
}

/// Serializable provider error that does not erase its broad category.
#[derive(Debug, Clone, Serialize)]
pub struct ProviderError {
    /// Stable broad error category.
    pub kind: String,
    /// Human-readable error detail.
    pub message: String,
}

impl From<&SearchError> for ProviderError {
    fn from(error: &SearchError) -> Self {
        let kind = match error {
            SearchError::RequestError(_) => "request",
            SearchError::Transport(_) => "transport",
            SearchError::ParseError(_) | SearchError::JsonError(_) => "parse",
            SearchError::UrlError(_) => "url",
            SearchError::UnknownProvider(_) => "unknown_provider",
            SearchError::ProviderDisabled(_) => "provider_disabled",
            SearchError::ApiError { .. } => "api",
            SearchError::ConfigError(_) => "configuration",
        };
        Self {
            kind: kind.to_string(),
            message: error.to_string(),
        }
    }
}

/// Results, diagnostics, and exact captures for one provider.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderOutcome {
    /// Requested provider id.
    pub provider: String,
    /// Success/error/unavailable status.
    pub status: ProviderOutcomeStatus,
    /// Unmerged results from this provider.
    pub results: Vec<SearchResult>,
    /// Every exact HTTP response observed for this provider.
    pub responses: Vec<TransportResponse>,
    /// Structured error, when status is `Error`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProviderError>,
}

/// Fused results alongside every provider outcome.
#[derive(Debug, Clone, Serialize)]
pub struct DetailedSearchResult {
    /// Fused successful provider results.
    pub results: Vec<SearchResult>,
    /// Per-provider results, errors, and response captures.
    pub outcomes: Vec<ProviderOutcome>,
}

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
            providers: get_default_provider_ids(),
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
    registry: Vec<RegistryEntry>,
    default_providers: Vec<String>,
    default_weights: HashMap<String, f64>,
    default_strategy: MergeStrategy,
}

impl WebSearchEngine {
    /// Create a new web search engine with default configuration
    pub fn new() -> Self {
        Self::with_config(WebSearchConfig::from_env())
    }

    /// Create a new web search engine with custom configuration.
    ///
    /// Providers are instantiated from the typed registry (the single source of
    /// truth), so every catalogued engine — class-based, descriptor-driven, and
    /// web-capture-backed — is available for selection.
    pub fn with_config(config: WebSearchConfig) -> Self {
        let mut providers: HashMap<String, Arc<RwLock<Box<dyn SearchProvider>>>> = HashMap::new();

        let build_config = BuildConfig {
            google_api_key: config.google_api_key,
            google_cx: config.google_cx,
            bing_api_key: config.bing_api_key,
        };

        for (id, provider) in build_providers(&build_config) {
            providers.insert(id, Arc::new(RwLock::new(provider)));
        }

        Self {
            providers,
            registry: get_registry(),
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
        let detailed = self
            .search_detailed_with_options(
                query,
                options,
                providers,
                merge_options,
                Arc::new(ReqwestTransport::default()),
            )
            .await;
        for outcome in &detailed.outcomes {
            if let Some(error) = &outcome.error {
                tracing::error!("Provider {} failed: {}", outcome.provider, error.message);
            }
        }
        Ok(detailed.results)
    }

    /// Search through a caller-owned transport and retain per-provider errors
    /// and exact response bytes. These provider futures are not spawned: when
    /// the returned future is dropped, all in-flight work is dropped with it.
    pub async fn search_detailed_with_options(
        &self,
        query: &str,
        options: SearchOptions,
        providers: Option<Vec<String>>,
        merge_options: Option<MergeOptions>,
        transport: Arc<dyn SearchTransport>,
    ) -> DetailedSearchResult {
        if query.is_empty() {
            return DetailedSearchResult {
                results: Vec::new(),
                outcomes: Vec::new(),
            };
        }

        let providers_to_use = providers.unwrap_or_else(|| self.default_providers.clone());
        let merge_opts = merge_options.unwrap_or_else(|| MergeOptions {
            strategy: self.default_strategy,
            weights: self.default_weights.clone(),
            rrf_k: None,
            remove_duplicates: true,
        });
        let futures = providers_to_use.into_iter().map(|name| {
            let provider = self.providers.get(&name).cloned();
            let options = options.clone();
            let transport = transport.clone();
            async move {
                let Some(provider) = provider else {
                    let error = SearchError::UnknownProvider(name.clone());
                    return ProviderOutcome {
                        provider: name,
                        status: ProviderOutcomeStatus::Error,
                        results: Vec::new(),
                        responses: Vec::new(),
                        error: Some(ProviderError::from(&error)),
                    };
                };
                let provider = provider.read().await;
                if !provider.is_available() {
                    return ProviderOutcome {
                        provider: name,
                        status: ProviderOutcomeStatus::Unavailable,
                        results: Vec::new(),
                        responses: Vec::new(),
                        error: None,
                    };
                }
                let responses = Arc::new(std::sync::Mutex::new(Vec::new()));
                let recording = RecordingTransport {
                    inner: transport,
                    responses: responses.clone(),
                };
                let result = provider
                    .search_with_transport(query, &options, &recording)
                    .await;
                let captures = responses
                    .lock()
                    .expect("response capture mutex poisoned")
                    .clone();
                match result {
                    Ok(results) => ProviderOutcome {
                        provider: name,
                        status: ProviderOutcomeStatus::Success,
                        results,
                        responses: captures,
                        error: None,
                    },
                    Err(error) => ProviderOutcome {
                        provider: name,
                        status: ProviderOutcomeStatus::Error,
                        results: Vec::new(),
                        responses: captures,
                        error: Some(ProviderError::from(&error)),
                    },
                }
            }
        });
        let outcomes = join_all(futures).await;
        let results_by_provider = outcomes
            .iter()
            .filter(|outcome| outcome.status == ProviderOutcomeStatus::Success)
            .map(|outcome| (outcome.provider.clone(), outcome.results.clone()))
            .collect();
        DetailedSearchResult {
            results: merge_results(&results_by_provider, &merge_opts),
            outcomes,
        }
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

    /// Search one provider through a caller-owned transport.
    pub async fn search_single_with_transport(
        &self,
        query: &str,
        provider_name: &str,
        options: SearchOptions,
        transport: &dyn SearchTransport,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| SearchError::UnknownProvider(provider_name.to_string()))?;
        let provider = provider.read().await;
        if !provider.is_available() {
            return Err(SearchError::ProviderDisabled(provider_name.to_string()));
        }
        provider
            .search_with_transport(query, &options, transport)
            .await
    }

    /// Get available provider names
    pub fn get_available_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    /// Get the full provider registry (metadata for every known provider).
    pub fn get_registry(&self) -> &[RegistryEntry] {
        &self.registry
    }

    /// Get provider status, enriched with registry metadata (category, label,
    /// CORS readability, access mechanism) so callers see the same shape the
    /// JavaScript implementation exposes.
    pub async fn get_provider_status(&self) -> HashMap<String, ProviderStatus> {
        let mut status = HashMap::new();

        for (name, provider) in &self.providers {
            let p = provider.read().await;
            let meta = self.registry.iter().find(|e| &e.id == name);
            status.insert(
                name.clone(),
                ProviderStatus {
                    enabled: p.is_available(),
                    weight: p.weight(),
                    category: meta.map(|m| m.category.clone()),
                    label: meta.map(|m| m.label.clone()),
                    cors_readable: meta.map(|m| m.cors_readable),
                    access: meta.map(|m| m.access.clone()),
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

/// Provider status information, enriched with registry metadata.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStatus {
    /// Whether the provider is enabled
    pub enabled: bool,
    /// Provider weight for reranking
    pub weight: f64,
    /// Provider category (one of the registry categories)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Human-readable label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Whether the endpoint is browser-CORS readable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_readable: Option<bool>,
    /// How results are obtained (api, html, hybrid, component, ...)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
}
