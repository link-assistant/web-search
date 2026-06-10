//! Search provider implementations

mod base;
mod bing;
mod duckduckgo;
mod engines;
mod generic;
mod google;
mod html_utils;
mod registry;
mod web_capture;

pub use base::{SearchOptions, SearchProvider, SearchResult};
pub use bing::{BingConfig, BingProvider};
pub use duckduckgo::DuckDuckGoProvider;
pub use engines::{
    access_for, all_descriptor_engines, api_engines, html_engines, EngineDescriptor, EngineKind,
    HttpMethod,
};
pub use generic::GenericProvider;
pub use google::{GoogleConfig, GoogleProvider};
pub use html_utils::{
    clean_text, decode_html_entities, parse_anchor_list, strip_html, AnchorConfig,
};
pub use registry::{
    build_providers, get_default_provider_ids, get_provider_ids, get_registry, is_known_category,
    BuildConfig, RegistryEntry, CATEGORIES,
};
pub use web_capture::{WebCaptureProvider, SUPPORTED_PROVIDERS};
