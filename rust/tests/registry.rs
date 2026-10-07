//! Integration tests for the typed provider registry (Rust parity with
//! `tests/registry.test.js`).

#[cfg(feature = "server")]
use web_search::providers::{build_providers, BuildConfig};
use web_search::{
    get_default_provider_ids, get_provider_ids, get_registry, is_known_category, CATEGORIES,
};

#[test]
fn categories_are_in_canonical_order() {
    assert_eq!(CATEGORIES, ["search", "knowledge", "papers", "code"]);
}

#[test]
fn registry_lists_every_catalogued_provider() {
    let registry = get_registry();
    // 3 class engines + 21 API + 11 HTML + 5 web-capture = 40.
    assert_eq!(registry.len(), 40);

    let ids: Vec<&str> = registry.iter().map(|e| e.id.as_str()).collect();
    for expected in [
        "google",
        "bing",
        "duckduckgo",
        "wikipedia",
        "wikidata",
        "wiktionary",
        "wikinews",
        "internet-archive",
        "dbpedia",
        "openlibrary",
        "semantic-scholar",
        "openalex",
        "crossref",
        "searx",
        "arxiv",
        "europepmc",
        "doaj",
        "github",
        "hackernews",
        "gitlab",
        "codeberg",
        "gitee",
        "bitbucket",
        "gitflic",
        "brave",
        "mojeek",
        "ecosia",
        "startpage",
        "yahoo",
        "yandex",
        "cambridge-dictionary",
        "merriam-webster",
        "dictionary-com",
        "collins-dictionary",
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
    assert_eq!(get_provider_ids(None).len(), 40);
    assert_eq!(
        get_provider_ids(Some("code")),
        [
            "github",
            "hackernews",
            "gitlab",
            "codeberg",
            "gitee",
            "bitbucket",
            "gitflic"
        ]
    );
    assert_eq!(
        get_provider_ids(Some("papers")),
        ["arxiv", "europepmc", "doaj"]
    );
    assert_eq!(
        get_provider_ids(Some("knowledge")),
        [
            "wikipedia",
            "wikidata",
            "wiktionary",
            "wikinews",
            "internet-archive",
            "dbpedia",
            "openlibrary",
            "semantic-scholar",
            "openalex",
            "crossref",
            "cambridge-dictionary",
            "merriam-webster",
            "dictionary-com",
            "collins-dictionary"
        ]
    );
    // Every search-category provider, including the web-capture namespace.
    assert!(get_provider_ids(Some("search")).len() >= 10);
}

#[test]
fn defaults_cover_one_provider_per_category_intent() {
    assert_eq!(
        get_default_provider_ids(),
        [
            "duckduckgo",
            "internet-archive",
            "wikipedia",
            "wikidata",
            "wiktionary",
            "wikinews"
        ]
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

#[cfg(feature = "server")]
#[test]
fn build_providers_instantiates_the_whole_catalog() {
    let providers = build_providers(&BuildConfig::default());
    assert_eq!(providers.len(), 40);

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

#[cfg(feature = "server")]
#[test]
fn core_metadata_matches_every_server_descriptor_and_endpoint() {
    use web_search::providers::{access_for, all_descriptor_engines, HttpMethod, SearchOptions};
    use web_search::registry::provider_metadata;

    for descriptor in all_descriptor_engines() {
        let entry = provider_metadata(descriptor.id).unwrap();
        assert_eq!(entry.label, descriptor.label);
        assert_eq!(entry.category, descriptor.category);
        assert_eq!(entry.cors_readable, descriptor.cors_readable);
        assert_eq!(entry.default_for_category, descriptor.default_for_category);
        assert_eq!(entry.access, access_for(descriptor.kind));
        let query = "rust & web";
        let encoded = urlencoding::encode(query);
        let endpoint = entry
            .endpoint_template
            .replace("{query}", &encoded)
            .replace("{language}", "en")
            .replace("{limit}", "10");
        assert_eq!(
            endpoint,
            (descriptor.build_url)(query, &SearchOptions::default()),
            "endpoint mismatch for {}",
            descriptor.id
        );
        assert_eq!(
            entry.capabilities.method.as_str(),
            match descriptor.method {
                HttpMethod::Get => "GET",
                HttpMethod::Post => "POST",
            }
        );
        assert_eq!(
            entry.body_template.is_some(),
            descriptor.build_body.is_some()
        );
    }
    let catalog = get_registry();
    let mut ids: Vec<_> = catalog.iter().map(|e| e.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), catalog.len());
    assert_eq!(
        web_search::providers::SUPPORTED_PROVIDERS,
        ["wikipedia", "duckduckgo", "google", "bing", "brave"]
    );
}

#[cfg(feature = "server")]
#[test]
fn discovery_metadata_serializes_for_wasm_consumers() {
    let entries = get_registry();
    let json = serde_json::to_value(&entries).unwrap();
    assert_eq!(
        json[0]["endpointTemplate"],
        "https://www.google.com/search?q={query}"
    );
    assert_eq!(json[0]["capabilities"]["method"], "GET");
    assert_eq!(json[0]["capabilities"]["optionalCredentials"], true);
    let roundtrip: Vec<web_search::RegistryEntry> = serde_json::from_value(json).unwrap();
    assert_eq!(roundtrip.len(), entries.len());
}
