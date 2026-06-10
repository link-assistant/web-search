//! Integration tests for the typed provider registry (Rust parity with
//! `tests/registry.test.js`).

use web_search::providers::{
    build_providers, get_default_provider_ids, get_provider_ids, get_registry, is_known_category,
    BuildConfig,
};
use web_search::CATEGORIES;

#[test]
fn categories_are_in_canonical_order() {
    assert_eq!(CATEGORIES, ["search", "knowledge", "papers", "code"]);
}

#[test]
fn registry_lists_every_catalogued_provider() {
    let registry = get_registry();
    // 3 class engines + 8 API + 6 HTML + 5 web-capture = 22.
    assert_eq!(registry.len(), 22);

    let ids: Vec<&str> = registry.iter().map(|e| e.id.as_str()).collect();
    for expected in [
        "google",
        "bing",
        "duckduckgo",
        "wikipedia",
        "wikidata",
        "searx",
        "crossref",
        "openalex",
        "github",
        "hackernews",
        "arxiv",
        "brave",
        "mojeek",
        "ecosia",
        "startpage",
        "yahoo",
        "lite",
        "wc:wikipedia",
        "wc:brave",
    ] {
        assert!(ids.contains(&expected), "missing provider id: {expected}");
    }
}

#[test]
fn every_entry_has_a_known_category() {
    for entry in get_registry() {
        assert!(
            is_known_category(&entry.category),
            "unknown category {} for {}",
            entry.category,
            entry.id
        );
    }
}

#[test]
fn provider_ids_filter_by_category() {
    assert_eq!(get_provider_ids(None).len(), 22);
    assert_eq!(get_provider_ids(Some("code")), ["github", "hackernews"]);
    assert_eq!(
        get_provider_ids(Some("papers")),
        ["crossref", "openalex", "arxiv"]
    );
    assert_eq!(
        get_provider_ids(Some("knowledge")),
        ["wikipedia", "wikidata"]
    );
    // Every search-category provider, including the web-capture namespace.
    assert!(get_provider_ids(Some("search")).len() >= 10);
}

#[test]
fn defaults_cover_one_provider_per_category_intent() {
    assert_eq!(
        get_default_provider_ids(),
        ["duckduckgo", "google", "bing", "wikipedia"]
    );
}

#[test]
fn default_for_category_flags_exactly_one_default_per_category() {
    let registry = get_registry();
    for category in CATEGORIES {
        let defaults: Vec<&str> = registry
            .iter()
            .filter(|e| e.category == category && e.default_for_category)
            .map(|e| e.id.as_str())
            .collect();
        assert_eq!(
            defaults.len(),
            1,
            "category {category} should have exactly one default, got {defaults:?}"
        );
    }
}

#[test]
fn build_providers_instantiates_the_whole_catalog() {
    let providers = build_providers(&BuildConfig::default());
    assert_eq!(providers.len(), 22);

    let names: Vec<&str> = providers.iter().map(|(id, _)| id.as_str()).collect();
    assert!(names.contains(&"github"));
    assert!(names.contains(&"wc:wikipedia"));

    // The instantiated provider reports the registry id as its name.
    let (id, provider) = providers
        .iter()
        .find(|(id, _)| id == "wikipedia")
        .expect("wikipedia provider");
    assert_eq!(provider.name(), id);
}

#[test]
fn web_capture_entries_use_the_component_access_label() {
    let wc: Vec<_> = get_registry()
        .into_iter()
        .filter(|e| e.id.starts_with("wc:"))
        .collect();
    assert_eq!(wc.len(), 5);
    for entry in &wc {
        assert_eq!(entry.access, "component");
        assert_eq!(entry.category, "search");
    }
    // Only the Wikipedia-backed web-capture engine is CORS readable.
    let wiki = wc.iter().find(|e| e.id == "wc:wikipedia").unwrap();
    assert!(wiki.cors_readable);
    let google = wc.iter().find(|e| e.id == "wc:google").unwrap();
    assert!(!google.cors_readable);
}
