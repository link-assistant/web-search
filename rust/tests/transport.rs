use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use web_search::{
    ProviderOutcomeStatus, SearchError, SearchOptions, SearchTransport, TransportRequest,
    TransportResponse, WebSearchEngine,
};

struct RoutedTransport;

#[async_trait]
impl SearchTransport for RoutedTransport {
    async fn execute(&self, request: TransportRequest) -> Result<TransportResponse, SearchError> {
        if request.url.contains("wikipedia.org") {
            let body = br#"{"pages":[{"key":"Cat","title":"Cat","excerpt":"A cat"}]}"#.to_vec();
            return Ok(TransportResponse {
                url: request.url,
                status: 200,
                headers: BTreeMap::new(),
                body,
                receipt: Some("cache:sha256:cat".to_string()),
            });
        }
        Err(SearchError::Transport("offline".to_string()))
    }
}

#[tokio::test]
async fn detailed_search_preserves_captures_and_provider_errors() {
    let engine = WebSearchEngine::new();
    let detailed = engine
        .search_detailed_with_options(
            "cat",
            SearchOptions::default(),
            Some(vec!["wikipedia".to_string(), "github".to_string()]),
            None,
            Arc::new(RoutedTransport),
        )
        .await;

    assert_eq!(detailed.results.len(), 1);
    assert_eq!(detailed.outcomes[0].status, ProviderOutcomeStatus::Success);
    assert_eq!(detailed.outcomes[0].responses.len(), 1);
    assert_eq!(
        detailed.outcomes[0].responses[0].receipt.as_deref(),
        Some("cache:sha256:cat")
    );
    assert!(String::from_utf8_lossy(&detailed.outcomes[0].responses[0].body).contains("Cat"));
    assert_eq!(detailed.outcomes[1].status, ProviderOutcomeStatus::Error);
    assert_eq!(
        detailed.outcomes[1].error.as_ref().unwrap().kind,
        "transport"
    );
}

#[tokio::test]
async fn web_capture_provider_uses_the_caller_transport() {
    let engine = WebSearchEngine::new();
    let results = engine
        .search_single_with_transport(
            "cat",
            "wc:wikipedia",
            SearchOptions::default(),
            &RoutedTransport,
        )
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].url, "https://en.wikipedia.org/wiki/Cat");
}

struct DropMarker(Arc<AtomicBool>);

impl Drop for DropMarker {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

struct HangingTransport(Arc<AtomicBool>);

#[async_trait]
impl SearchTransport for HangingTransport {
    async fn execute(&self, _request: TransportRequest) -> Result<TransportResponse, SearchError> {
        let _marker = DropMarker(self.0.clone());
        std::future::pending().await
    }
}

#[tokio::test]
async fn dropping_search_future_drops_in_flight_provider_work() {
    let dropped = Arc::new(AtomicBool::new(false));
    let engine = WebSearchEngine::new();
    let search = engine.search_detailed_with_options(
        "cat",
        SearchOptions::default(),
        Some(vec!["wikipedia".to_string()]),
        None,
        Arc::new(HangingTransport(dropped.clone())),
    );

    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(20), search)
            .await
            .is_err()
    );
    assert!(dropped.load(Ordering::SeqCst));
}
