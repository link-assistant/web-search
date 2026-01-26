//! DuckDuckGo search provider

use async_trait::async_trait;
use scraper::{Html, Selector};

use super::base::{SearchOptions, SearchProvider, SearchResult};
use crate::error::SearchError;

/// DuckDuckGo search provider
pub struct DuckDuckGoProvider {
    name: String,
    enabled: bool,
    weight: f64,
    client: reqwest::Client,
    base_url: String,
}

impl DuckDuckGoProvider {
    /// Create a new DuckDuckGo provider
    pub fn new() -> Self {
        Self {
            name: "duckduckgo".to_string(),
            enabled: true,
            weight: 1.0,
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .expect("Failed to create HTTP client"),
            base_url: "https://html.duckduckgo.com/html/".to_string(),
        }
    }

    fn parse_results(&self, html: &str, limit: usize) -> Vec<SearchResult> {
        let document = Html::parse_document(html);
        let mut results = Vec::new();

        let result_selector =
            Selector::parse(".result__a").unwrap_or_else(|_| Selector::parse("a").unwrap());
        let snippet_selector = Selector::parse(".result__snippet")
            .unwrap_or_else(|_| Selector::parse(".result__body").unwrap());

        let links: Vec<_> = document.select(&result_selector).collect();
        let snippets: Vec<_> = document.select(&snippet_selector).collect();

        for (i, link) in links.iter().enumerate() {
            if results.len() >= limit {
                break;
            }

            let url = link.value().attr("href").unwrap_or_default();
            if url.is_empty() || url.starts_with("//duckduckgo.com") || url.contains("ad_provider")
            {
                continue;
            }

            let decoded_url = urlencoding::decode(url).unwrap_or_else(|_| url.into());
            let title = link.text().collect::<String>().trim().to_string();
            let snippet = snippets
                .get(i)
                .map(|s| s.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            results.push(SearchResult {
                title: if title.is_empty() {
                    "Untitled".to_string()
                } else {
                    title
                },
                url: decoded_url.to_string(),
                snippet,
                source: self.name.clone(),
                rank: results.len() + 1,
                score: None,
                sources: None,
            });
        }

        results
    }
}

impl Default for DuckDuckGoProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SearchProvider for DuckDuckGoProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_available(&self) -> bool {
        self.enabled
    }

    fn weight(&self) -> f64 {
        self.weight
    }

    fn set_weight(&mut self, weight: f64) {
        self.weight = weight.clamp(0.0, 1.0);
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    async fn search(
        &self,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        if query.is_empty() {
            return Ok(Vec::new());
        }

        let limit = options.limit.unwrap_or(10);
        let mut params = vec![("q", query.to_string())];

        if let Some(ref region) = options.region {
            params.push(("kl", region.clone()));
        } else {
            params.push(("kl", "wt-wt".to_string()));
        }

        if let Some(safe) = options.safe_search {
            params.push(("kp", if safe { "1" } else { "-2" }.to_string()));
        }

        let response = self
            .client
            .post(&self.base_url)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SearchError::ApiError {
                provider: self.name.clone(),
                message: format!("HTTP {}", response.status()),
            });
        }

        let html = response.text().await?;
        Ok(self.parse_results(&html, limit))
    }
}
