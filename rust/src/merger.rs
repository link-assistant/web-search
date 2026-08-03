//! Search result merger with reranking support

use std::collections::{HashMap, HashSet};

use crate::SearchResult;

/// Merge strategy for combining results
#[derive(Debug, Clone, Copy, Default)]
pub enum MergeStrategy {
    /// Reciprocal Rank Fusion (default)
    #[default]
    Rrf,
    /// Weighted scoring
    Weighted,
    /// Interleaved (round-robin)
    Interleave,
}

/// Options for merging search results
#[derive(Debug, Clone, Default)]
pub struct MergeOptions {
    /// Merge strategy to use
    pub strategy: MergeStrategy,
    /// Weights for each provider (provider name -> weight)
    pub weights: HashMap<String, f64>,
    /// RRF k parameter (default: 60)
    pub rrf_k: Option<f64>,
    /// Whether to remove duplicate URLs (default: true)
    pub remove_duplicates: bool,
}

impl MergeOptions {
    /// Create new merge options with default values
    pub fn new() -> Self {
        Self {
            strategy: MergeStrategy::Rrf,
            weights: HashMap::new(),
            rrf_k: None,
            remove_duplicates: true,
        }
    }

    /// Set the merge strategy
    pub fn with_strategy(mut self, strategy: MergeStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set provider weights
    pub fn with_weights(mut self, weights: HashMap<String, f64>) -> Self {
        self.weights = weights;
        self
    }

    /// Set the RRF k parameter
    pub fn with_rrf_k(mut self, k: f64) -> Self {
        self.rrf_k = Some(k);
        self
    }
}

/// Normalize URL for deduplication
fn normalize_url(url: &str) -> String {
    match url::Url::parse(url) {
        Ok(parsed) => {
            let mut normalized = format!("{}{}", parsed.host_str().unwrap_or(""), parsed.path());
            normalized = normalized.trim_end_matches('/').to_lowercase();
            normalized
        }
        Err(_) => url.to_lowercase(),
    }
}

/// Calculate RRF score
fn rrf_score(rank: usize, k: f64) -> f64 {
    1.0 / (k + rank as f64)
}

/// Merge results using Reciprocal Rank Fusion
pub fn merge_with_rrf(
    results_by_provider: &HashMap<String, Vec<SearchResult>>,
    options: &MergeOptions,
) -> Vec<SearchResult> {
    let k = options.rrf_k.unwrap_or(60.0);
    let mut scores_by_url: HashMap<String, f64> = HashMap::new();
    let mut results_by_url: HashMap<String, SearchResult> = HashMap::new();
    let mut sources_by_url: HashMap<String, HashSet<String>> = HashMap::new();

    for (provider, results) in results_by_provider {
        let weight = options.weights.get(provider).copied().unwrap_or(1.0);

        for result in results {
            let normalized_url = normalize_url(&result.url);
            let score = rrf_score(result.rank, k) * weight;

            *scores_by_url.entry(normalized_url.clone()).or_insert(0.0) += score;

            sources_by_url
                .entry(normalized_url.clone())
                .or_default()
                .insert(result.source.clone());

            results_by_url
                .entry(normalized_url)
                .or_insert_with(|| result.clone());
        }
    }

    let mut merged: Vec<_> = scores_by_url
        .into_iter()
        .map(|(url, score)| {
            let mut result = results_by_url.remove(&url).unwrap();
            result.score = Some(score);
            let sources: Vec<_> = sources_by_url
                .get(&url)
                .map(|s| s.iter().cloned().collect())
                .unwrap_or_default();
            if sources.len() > 1 {
                result.sources = Some(sources);
            }
            (score, result)
        })
        .collect();

    merged.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    merged
        .into_iter()
        .enumerate()
        .map(|(i, (_, mut result))| {
            result.rank = i + 1;
            result
        })
        .collect()
}

/// Merge results using weighted scoring
pub fn merge_with_weights(
    results_by_provider: &HashMap<String, Vec<SearchResult>>,
    options: &MergeOptions,
) -> Vec<SearchResult> {
    let max_rank = 100.0;
    let mut scores_by_url: HashMap<String, f64> = HashMap::new();
    let mut results_by_url: HashMap<String, SearchResult> = HashMap::new();
    let mut sources_by_url: HashMap<String, HashSet<String>> = HashMap::new();

    for (provider, results) in results_by_provider {
        let weight = options.weights.get(provider).copied().unwrap_or(1.0);

        for result in results {
            let normalized_url = normalize_url(&result.url);
            let score = ((max_rank - result.rank as f64 + 1.0) / max_rank) * weight;

            *scores_by_url.entry(normalized_url.clone()).or_insert(0.0) += score;

            sources_by_url
                .entry(normalized_url.clone())
                .or_default()
                .insert(result.source.clone());

            results_by_url
                .entry(normalized_url)
                .or_insert_with(|| result.clone());
        }
    }

    let mut merged: Vec<_> = scores_by_url
        .into_iter()
        .map(|(url, score)| {
            let mut result = results_by_url.remove(&url).unwrap();
            result.score = Some(score);
            let sources: Vec<_> = sources_by_url
                .get(&url)
                .map(|s| s.iter().cloned().collect())
                .unwrap_or_default();
            if sources.len() > 1 {
                result.sources = Some(sources);
            }
            (score, result)
        })
        .collect();

    merged.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    merged
        .into_iter()
        .enumerate()
        .map(|(i, (_, mut result))| {
            result.rank = i + 1;
            result
        })
        .collect()
}

/// Merge results using interleaving (round-robin)
pub fn merge_with_interleave(
    results_by_provider: &HashMap<String, Vec<SearchResult>>,
    options: &MergeOptions,
) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let mut seen_urls: HashSet<String> = HashSet::new();

    let providers: Vec<_> = results_by_provider.keys().collect();
    let max_len = results_by_provider
        .values()
        .map(|v| v.len())
        .max()
        .unwrap_or(0);

    for i in 0..max_len {
        for provider in &providers {
            if let Some(provider_results) = results_by_provider.get(*provider) {
                if i < provider_results.len() {
                    let result = &provider_results[i];

                    if options.remove_duplicates {
                        let normalized = normalize_url(&result.url);
                        if seen_urls.contains(&normalized) {
                            continue;
                        }
                        seen_urls.insert(normalized);
                    }

                    let mut new_result = result.clone();
                    new_result.rank = results.len() + 1;
                    results.push(new_result);
                }
            }
        }
    }

    results
}

/// Merge search results using the specified strategy
pub fn merge_results(
    results_by_provider: &HashMap<String, Vec<SearchResult>>,
    options: &MergeOptions,
) -> Vec<SearchResult> {
    match options.strategy {
        MergeStrategy::Rrf => merge_with_rrf(results_by_provider, options),
        MergeStrategy::Weighted => merge_with_weights(results_by_provider, options),
        MergeStrategy::Interleave => merge_with_interleave(results_by_provider, options),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_result(url: &str, title: &str, source: &str, rank: usize) -> SearchResult {
        SearchResult {
            title: title.to_string(),
            url: url.to_string(),
            snippet: String::new(),
            source: source.to_string(),
            rank,
            score: None,
            sources: None,
        }
    }

    #[test]
    fn test_rrf_merge() {
        let mut results_by_provider = HashMap::new();

        results_by_provider.insert(
            "google".to_string(),
            vec![
                create_test_result("https://example.com/1", "Result 1", "google", 1),
                create_test_result("https://example.com/2", "Result 2", "google", 2),
            ],
        );

        results_by_provider.insert(
            "bing".to_string(),
            vec![
                create_test_result("https://example.com/2", "Result 2", "bing", 1),
                create_test_result("https://example.com/3", "Result 3", "bing", 2),
            ],
        );

        let options = MergeOptions::new();
        let merged = merge_with_rrf(&results_by_provider, &options);

        assert_eq!(merged.len(), 3);
        assert!(merged[0].url.contains("example.com/2"));
    }

    #[test]
    fn test_url_normalization() {
        assert_eq!(
            normalize_url("https://example.com/path/"),
            normalize_url("https://example.com/path")
        );
        assert_eq!(
            normalize_url("https://Example.COM/Path"),
            normalize_url("https://example.com/path")
        );
    }
}
