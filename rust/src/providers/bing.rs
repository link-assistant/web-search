//! Bing search provider

use async_trait::async_trait;
use serde::Deserialize;

use super::base::{SearchOptions, SearchProvider, SearchResult};
use crate::error::SearchError;

/// Bing Web Search API response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BingApiResponse {
    web_pages: Option<BingWebPages>,
}

#[derive(Debug, Deserialize)]
struct BingWebPages {
    value: Vec<BingWebPage>,
}

#[derive(Debug, Deserialize)]
struct BingWebPage {
    name: String,
    url: String,
    snippet: Option<String>,
}

/// Configuration for Bing provider
#[derive(Debug, Clone, Default)]
pub struct BingConfig {
    /// Bing Search API key
    pub api_key: Option<String>,
}

/// Bing search provider
pub struct BingProvider {
    name: String,
    enabled: bool,
    weight: f64,
    client: reqwest::Client,
    config: BingConfig,
    api_url: String,
}

impl BingProvider {
    /// Create a new Bing provider with optional API credentials
    pub fn new(config: BingConfig) -> Self {
        Self {
            name: "bing".to_string(),
            enabled: true,
            weight: 1.0,
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .expect("Failed to create HTTP client"),
            config,
            api_url: "https://api.bing.microsoft.com/v7.0/search".to_string(),
        }
    }

    /// Create a new Bing provider from environment variables
    pub fn from_env() -> Self {
        Self::new(BingConfig {
            api_key: std::env::var("BING_API_KEY").ok(),
        })
    }

    /// Check if API credentials are configured
    pub fn has_api_credentials(&self) -> bool {
        self.config.api_key.is_some()
    }

    async fn search_with_api(
        &self,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let api_key = self.config.api_key.as_ref().unwrap();
        let limit = options.limit.unwrap_or(10).min(50);

        let mut url = format!(
            "{}?q={}&count={}&responseFilter=Webpages",
            self.api_url,
            urlencoding::encode(query),
            limit
        );

        if let Some(ref region) = options.region {
            let lang = options.language.as_deref().unwrap_or("en");
            url.push_str(&format!("&mkt={}-{}", lang, region.to_uppercase()));
        }

        let safe_search = match options.safe_search {
            Some(true) => "Strict",
            Some(false) => "Off",
            None => "Moderate",
        };
        url.push_str(&format!("&safeSearch={}", safe_search));

        let response = self
            .client
            .get(&url)
            .header("Ocp-Apim-Subscription-Key", api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SearchError::ApiError {
                provider: self.name.clone(),
                message: error_text,
            });
        }

        let api_response: BingApiResponse = response.json().await?;

        let results = api_response
            .web_pages
            .map(|wp| wp.value)
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(i, page)| SearchResult {
                title: page.name,
                url: page.url,
                snippet: page.snippet.unwrap_or_default(),
                source: self.name.clone(),
                rank: i + 1,
                score: None,
                sources: None,
            })
            .collect();

        Ok(results)
    }

    async fn search_with_scraping(
        &self,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let limit = options.limit.unwrap_or(10);
        let mut url = format!(
            "https://www.bing.com/search?q={}&count={}",
            urlencoding::encode(query),
            limit.min(30)
        );

        if let Some(ref region) = options.region {
            url.push_str(&format!("&cc={}", region.to_uppercase()));
        }

        let response = self
            .client
            .get(&url)
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            )
            .header("Accept-Language", "en-US,en;q=0.5")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SearchError::ApiError {
                provider: self.name.clone(),
                message: format!("HTTP {}", response.status()),
            });
        }

        let html = response.text().await?;
        Ok(self.parse_scraped_results(&html, limit))
    }

    fn parse_scraped_results(&self, html: &str, limit: usize) -> Vec<SearchResult> {
        use scraper::{Html, Selector};

        let document = Html::parse_document(html);
        let mut results = Vec::new();

        let algo_selector = Selector::parse(".b_algo").unwrap();
        let link_selector = Selector::parse("a").unwrap();
        let h2_selector = Selector::parse("h2").unwrap();
        let snippet_selector = Selector::parse("p").unwrap();

        for element in document.select(&algo_selector) {
            if results.len() >= limit {
                break;
            }

            let link = element.select(&link_selector).next();
            let h2 = element.select(&h2_selector).next();
            let snippet = element.select(&snippet_selector).next();

            if let Some(link_elem) = link {
                let url = link_elem.value().attr("href").unwrap_or_default();
                if url.is_empty() || url.contains("bing.com") || url.starts_with('/') {
                    continue;
                }

                let title = h2
                    .map(|h| h.text().collect::<String>())
                    .unwrap_or_else(|| link_elem.text().collect::<String>())
                    .trim()
                    .to_string();

                let snippet_text = snippet
                    .map(|s| s.text().collect::<String>())
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                if title.is_empty() {
                    continue;
                }

                results.push(SearchResult {
                    title,
                    url: url.to_string(),
                    snippet: snippet_text,
                    source: self.name.clone(),
                    rank: results.len() + 1,
                    score: None,
                    sources: None,
                });
            }
        }

        results
    }
}

impl Default for BingProvider {
    fn default() -> Self {
        Self::from_env()
    }
}

#[async_trait]
impl SearchProvider for BingProvider {
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

        if self.has_api_credentials() {
            match self.search_with_api(query, options).await {
                Ok(results) => return Ok(results),
                Err(e) => {
                    tracing::warn!("Bing API search failed, falling back to scraping: {}", e);
                }
            }
        }

        self.search_with_scraping(query, options).await
    }
}
