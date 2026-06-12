//! Integration coverage for the Rust web-capture component provider.

use web_search::providers::{
    SearchOptions, SearchProvider, WebCaptureProvider, SUPPORTED_PROVIDERS,
};

#[test]
fn web_capture_provider_contract_tracks_the_published_crate() {
    assert_eq!(SUPPORTED_PROVIDERS, web_capture::SEARCH_PROVIDERS);
}

#[tokio::test]
async fn web_capture_provider_handles_empty_queries_without_network() {
    let provider = WebCaptureProvider::default();

    assert_eq!(provider.name(), "wc:wikipedia");
    assert_eq!(provider.engine(), "wikipedia");
    assert!(provider.is_available());

    let results = provider
        .search("", &SearchOptions::default())
        .await
        .expect("empty queries should not fail");

    assert!(results.is_empty());
}

#[test]
fn web_capture_provider_adapts_normalized_items() {
    let provider = WebCaptureProvider::new("bing");
    let results = provider.adapt_items(vec![
        web_capture::SearchResultItem {
            rank: 1,
            title: "Web Capture".to_string(),
            url: "https://example.com/search".to_string(),
            snippet: "normalized".to_string(),
        },
        web_capture::SearchResultItem {
            rank: 0,
            title: String::new(),
            url: "https://example.com/fallback".to_string(),
            snippet: String::new(),
        },
        web_capture::SearchResultItem {
            rank: 3,
            title: "Ignored".to_string(),
            url: String::new(),
            snippet: "missing URL".to_string(),
        },
    ]);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "Web Capture");
    assert_eq!(results[0].source, "wc:bing");
    assert_eq!(results[0].rank, 1);
    assert_eq!(results[1].title, "Untitled");
    assert_eq!(results[1].rank, 2);
}
