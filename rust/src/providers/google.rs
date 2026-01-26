//! Google search provider

use async_trait::async_trait;
use serde::Deserialize;

use super::base::{SearchOptions, SearchProvider, SearchResult};
use crate::error::SearchError;

/// Google Custom Search API response
#[derive(Debug, Deserialize)]
struct GoogleApiResponse {
    items: Option<Vec<GoogleApiItem>>,
}

#[derive(Debug, Deserialize)]
struct GoogleApiItem {
    title: String,
    link: String,
    snippet: Option<String>,
}

/// Configuration for Google provider
#[derive(Debug, Clone, Default)]
pub struct GoogleConfig {
    /// Google Custom Search API key
    pub api_key: Option<String>,
    /// Google Custom Search Engine ID
    pub search_engine_id: Option<String>,
}

/// Google search provider
pub struct GoogleProvider {
    name: String,
    enabled: bool,
    weight: f64,
    client: reqwest::Client,
    config: GoogleConfig,
    api_url: String,
}

impl GoogleProvider {
    /// Create a new Google provider with optional API credentials
    pub fn new(config: GoogleConfig) -> Self {
        Self {
            name: "google".to_string(),
            enabled: true,
            weight: 1.0,
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .expect("Failed to create HTTP client"),
            config,
            api_url: "https://www.googleapis.com/customsearch/v1".to_string(),
        }
    }

    /// Create a new Google provider from environment variables
    pub fn from_env() -> Self {
        Self::new(GoogleConfig {
            api_key: std::env::var("GOOGLE_API_KEY").ok(),
            search_engine_id: std::env::var("GOOGLE_CX").ok(),
        })
    }

    /// Check if API credentials are configured
    pub fn has_api_credentials(&self) -> bool {
        self.config.api_key.is_some() && self.config.search_engine_id.is_some()
    }

    async fn search_with_api(
        &self,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let api_key = self.config.api_key.as_ref().unwrap();
        let cx = self.config.search_engine_id.as_ref().unwrap();
        let limit = options.limit.unwrap_or(10).min(10);

        let mut url = format!(
            "{}?key={}&cx={}&q={}&num={}",
            self.api_url,
            api_key,
            cx,
            urlencoding::encode(query),
            limit
        );

        if let Some(ref lang) = options.language {
            url.push_str(&format!("&lr=lang_{}", lang));
        }

        if let Some(ref region) = options.region {
            url.push_str(&format!("&gl={}", region));
        }

        if let Some(safe) = options.safe_search {
            url.push_str(&format!("&safe={}", if safe { "active" } else { "off" }));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SearchError::ApiError {
                provider: self.name.clone(),
                message: error_text,
            });
        }

        let api_response: GoogleApiResponse = response.json().await?;

        let results = api_response
            .items
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(i, item)| SearchResult {
                title: item.title,
                url: item.link,
                snippet: item.snippet.unwrap_or_default(),
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
            "https://www.google.com/search?q={}&num={}",
            urlencoding::encode(query),
            limit.min(20)
        );

        if let Some(ref lang) = options.language {
            url.push_str(&format!("&hl={}", lang));
        }

        if let Some(ref region) = options.region {
            url.push_str(&format!("&gl={}", region));
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
        let mut seen_urls = std::collections::HashSet::new();

        let link_selector = Selector::parse("a").unwrap();
        let h3_selector = Selector::parse("h3").unwrap();

        for element in document.select(&link_selector) {
            if results.len() >= limit {
                break;
            }

            let href = element.value().attr("href").unwrap_or_default();
            let url = if href.starts_with("/url?q=") {
                href.strip_prefix("/url?q=")
                    .and_then(|u| u.split('&').next())
                    .map(|u| {
                        urlencoding::decode(u)
                            .unwrap_or_else(|_| u.into())
                            .to_string()
                    })
            } else if href.starts_with("http") && !href.contains("google.com") {
                Some(href.to_string())
            } else {
                None
            };

            if let Some(url) = url {
                if seen_urls.contains(&url) || url.contains("google.com") {
                    continue;
                }

                let title = element
                    .select(&h3_selector)
                    .next()
                    .map(|h3| h3.text().collect::<String>())
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                if title.is_empty() {
                    continue;
                }

                seen_urls.insert(url.clone());
                results.push(SearchResult {
                    title,
                    url,
                    snippet: String::new(),
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

impl Default for GoogleProvider {
    fn default() -> Self {
        Self::from_env()
    }
}

#[async_trait]
impl SearchProvider for GoogleProvider {
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
                    tracing::warn!("Google API search failed, falling back to scraping: {}", e);
                }
            }
        }

        self.search_with_scraping(query, options).await
    }
}
