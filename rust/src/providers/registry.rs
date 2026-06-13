//! Typed provider registry.
//!
//! A single source of truth describing every search provider this library can
//! use, grouped into the four categories that `formal-ai` consumes (`search`,
//! `knowledge`, `papers`, `code`). The registry powers provider discovery
//! (CLI/server/`/providers`) and is the factory that instantiates the correct
//! provider implementation for each id. Mirrors the JavaScript
//! `src/providers/registry.js` (issue #3 parity requirement).

use serde::Serialize;

use super::base::SearchProvider;
use super::bing::{BingConfig, BingProvider};
use super::duckduckgo::DuckDuckGoProvider;
use super::engines::{access_for, all_descriptor_engines, EngineDescriptor};
use super::generic::GenericProvider;
use super::google::{GoogleConfig, GoogleProvider};
use super::web_capture::{WebCaptureProvider, SUPPORTED_PROVIDERS};

/// Provider categories, mirroring `formal-ai`'s `web_search_core` registry.
pub const CATEGORIES: [&str; 4] = ["search", "knowledge", "papers", "code"];

/// Public metadata describing a single registered provider.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryEntry {
    /// Stable provider id.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Provider category (one of [`CATEGORIES`]).
    pub category: String,
    /// Whether the endpoint is browser-CORS readable.
    pub cors_readable: bool,
    /// Whether this is its category's default provider.
    pub default_for_category: bool,
    /// How results are obtained (`api`, `html`, `hybrid`, `component`, ...).
    pub access: String,
}

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

/// Metadata for a class-based provider (google/bing/duckduckgo) that predates
/// the descriptor catalog and keeps its dedicated API + scraping logic.
struct ClassEngine {
    id: &'static str,
    label: &'static str,
    category: &'static str,
    cors_readable: bool,
    default_for_category: bool,
    access: &'static str,
}

const CLASS_ENGINES: [ClassEngine; 3] = [
    ClassEngine {
        id: "google",
        label: "Google",
        category: "search",
        cors_readable: false,
        default_for_category: false,
        access: "hybrid",
    },
    ClassEngine {
        id: "bing",
        label: "Bing",
        category: "search",
        cors_readable: false,
        default_for_category: false,
        access: "hybrid",
    },
    ClassEngine {
        id: "duckduckgo",
        label: "DuckDuckGo",
        category: "search",
        cors_readable: false,
        default_for_category: true,
        access: "html",
    },
];

fn descriptor_entry(d: &EngineDescriptor) -> RegistryEntry {
    RegistryEntry {
        id: d.id.to_string(),
        label: d.label.to_string(),
        category: d.category.to_string(),
        cors_readable: d.cors_readable,
        default_for_category: d.default_for_category,
        access: access_for(d.kind).to_string(),
    }
}

/// Build the full registry of provider entries, in catalog order
/// (class engines, descriptor engines, then web-capture engines).
pub fn get_registry() -> Vec<RegistryEntry> {
    let mut entries = Vec::new();

    for e in &CLASS_ENGINES {
        entries.push(RegistryEntry {
            id: e.id.to_string(),
            label: e.label.to_string(),
            category: e.category.to_string(),
            cors_readable: e.cors_readable,
            default_for_category: e.default_for_category,
            access: e.access.to_string(),
        });
    }
    for d in all_descriptor_engines() {
        entries.push(descriptor_entry(&d));
    }
    for engine in SUPPORTED_PROVIDERS {
        entries.push(RegistryEntry {
            id: format!("wc:{engine}"),
            label: format!("web-capture ({engine})"),
            category: "search".to_string(),
            cors_readable: engine == "wikipedia",
            default_for_category: false,
            access: "component".to_string(),
        });
    }

    entries
}

/// Get all provider ids, optionally filtered by category.
pub fn get_provider_ids(category: Option<&str>) -> Vec<String> {
    get_registry()
        .into_iter()
        .filter(|e| category.is_none_or(|c| e.category == c))
        .map(|e| e.id)
        .collect()
}

/// Get the default provider ids used when the caller does not specify providers.
///
/// Mirrors FormalAI's live default plan (`WEB_SEARCH_PROVIDERS`): a
/// DuckDuckGo-first, CORS-readable knowledge sweep across the Wikimedia family
/// and the Internet Archive (issue #5 parity requirement).
pub fn get_default_provider_ids() -> Vec<String> {
    [
        "duckduckgo",
        "internet-archive",
        "wikipedia",
        "wikidata",
        "wiktionary",
        "wikinews",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

/// Whether `category` is a known category.
pub fn is_known_category(category: &str) -> bool {
    CATEGORIES.contains(&category)
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
