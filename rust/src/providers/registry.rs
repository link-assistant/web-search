//! Server provider factories. Discovery metadata lives in crate::registry.

pub use crate::registry::{
    get_default_provider_ids, get_provider_ids, get_registry, is_known_category, RegistryEntry,
    CATEGORIES,
};

use super::base::SearchProvider;
use super::bing::{BingConfig, BingProvider};
use super::duckduckgo::DuckDuckGoProvider;
use super::engines::all_descriptor_engines;
use super::generic::GenericProvider;
use super::google::{GoogleConfig, GoogleProvider};
use super::web_capture::{WebCaptureProvider, SUPPORTED_PROVIDERS};

/// Engine configuration used to instantiate providers.
#[derive(Debug, Clone, Default)]
pub struct BuildConfig {
    /// Google Custom Search API key.
    pub google_api_key: Option<String>,
    /// Google Custom Search Engine ID.
    pub google_cx: Option<String>,
    /// Bing Search API key.
    pub bing_api_key: Option<String>,
}

/// Instantiate every registered provider, keyed by id, in catalog order.
pub fn build_providers(config: &BuildConfig) -> Vec<(String, Box<dyn SearchProvider>)> {
    let mut providers: Vec<(String, Box<dyn SearchProvider>)> = Vec::new();

    providers.push((
        "google".to_string(),
        Box::new(GoogleProvider::new(GoogleConfig {
            api_key: config.google_api_key.clone(),
            search_engine_id: config.google_cx.clone(),
        })),
    ));
    providers.push((
        "bing".to_string(),
        Box::new(BingProvider::new(BingConfig {
            api_key: config.bing_api_key.clone(),
        })),
    ));
    providers.push((
        "duckduckgo".to_string(),
        Box::new(DuckDuckGoProvider::new()),
    ));

    for d in all_descriptor_engines() {
        providers.push((d.id.to_string(), Box::new(GenericProvider::new(d))));
    }

    for engine in SUPPORTED_PROVIDERS {
        providers.push((
            format!("wc:{engine}"),
            Box::new(WebCaptureProvider::new(engine)),
        ));
    }

    providers
}
