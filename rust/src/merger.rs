//! Search result merger with reranking support

use alloc::{
    collections::{BTreeMap, BTreeSet},
    format,
    string::String,
    vec::Vec,
};
use core::cmp::Ordering;

/// Provider weights retain the server HashMap API and use alloc in core builds.
#[cfg(feature = "server")]
pub type ProviderWeights = std::collections::HashMap<String, f64>;
#[cfg(not(feature = "server"))]
pub type ProviderWeights = BTreeMap<String, f64>;

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
    pub weights: ProviderWeights,
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
            weights: ProviderWeights::new(),
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
    pub fn with_weights(mut self, weights: impl IntoIterator<Item = (String, f64)>) -> Self {
        self.weights = weights.into_iter().collect();
        self
    }

    /// Set the RRF k parameter
    pub fn with_rrf_k(mut self, k: f64) -> Self {
        self.rrf_k = Some(k);
        self
    }
}

/// Normalize a URL using the same host/path deduplication rules as the server.
/// Scheme, port, query, fragment and trailing slashes are ignored; invalid URLs
/// fall back to lowercase text. These are ranking keys, not request URLs.
pub fn normalize_url(url: &str) -> String {
    match url::Url::parse(url) {
        Ok(parsed) => format!("{}{}", parsed.host_str().unwrap_or(""), parsed.path())
            .trim_end_matches('/')
            .to_lowercase(),
        Err(_) => url.to_lowercase(),
    }
}

fn ordered_lists<'a>(
    lists: impl IntoIterator<Item = (&'a String, &'a Vec<SearchResult>)>,
) -> Vec<(&'a String, &'a Vec<SearchResult>)> {
    let mut lists: Vec<_> = lists.into_iter().collect();
    lists.sort_by(|a, b| a.0.cmp(b.0));
    lists
}

fn merge_scored<'a>(
    lists: impl IntoIterator<Item = (&'a String, &'a Vec<SearchResult>)>,
    options: &MergeOptions,
    score: impl Fn(usize) -> f64,
) -> Vec<SearchResult> {
    // Ordered providers, URL keys and sources give identical output regardless
    // of the caller's collection type or randomized HashMap iteration order.
    let mut by_url: BTreeMap<String, (SearchResult, f64, BTreeSet<String>)> = BTreeMap::new();
    for (provider, results) in ordered_lists(lists) {
        let weight = options.weights.get(provider).copied().unwrap_or(1.0);
        for result in results {
            let entry = by_url
                .entry(normalize_url(&result.url))
                .or_insert_with(|| (result.clone(), 0.0, BTreeSet::new()));
            entry.1 += score(result.rank) * weight;
            entry.2.insert(result.source.clone());
        }
    }
    let mut merged: Vec<_> = by_url
        .into_values()
        .map(|(mut result, score, sources)| {
            result.score = Some(score);
            if sources.len() > 1 {
                result.sources = Some(sources.into_iter().collect());
            }
            result
        })
        .collect();
    // Stable sort preserves URL order on ties, including incomparable scores.
    merged.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
    for (index, result) in merged.iter_mut().enumerate() {
        result.rank = index + 1;
    }
    merged
}

/// Reciprocal Rank Fusion: sum each provider's weight / (k + original rank).
/// Accepts both std HashMap and alloc BTreeMap provider collections.
pub fn merge_with_rrf<'a>(
    lists: impl IntoIterator<Item = (&'a String, &'a Vec<SearchResult>)>,
    options: &MergeOptions,
) -> Vec<SearchResult> {
    let k = options.rrf_k.unwrap_or(60.0);
    merge_scored(lists, options, |rank| 1.0 / (k + rank as f64))
}

/// Merge using the server's weighted rank score.
pub fn merge_with_weights<'a>(
    lists: impl IntoIterator<Item = (&'a String, &'a Vec<SearchResult>)>,
    options: &MergeOptions,
) -> Vec<SearchResult> {
    merge_scored(lists, options, |rank| (100.0 - rank as f64 + 1.0) / 100.0)
}

/// Round-robin provider results in provider-id order, optionally deduplicated.
pub fn merge_with_interleave<'a>(
    lists: impl IntoIterator<Item = (&'a String, &'a Vec<SearchResult>)>,
    options: &MergeOptions,
) -> Vec<SearchResult> {
    let lists = ordered_lists(lists);
    let max_len = lists
        .iter()
        .map(|(_, results)| results.len())
        .max()
        .unwrap_or(0);
    let mut merged = Vec::new();
    let mut seen = BTreeSet::new();
    for index in 0..max_len {
        for (_, results) in &lists {
            if let Some(result) = results.get(index) {
                if options.remove_duplicates && !seen.insert(normalize_url(&result.url)) {
                    continue;
                }
                let mut result = result.clone();
                result.rank = merged.len() + 1;
                merged.push(result);
            }
        }
    }
    merged
}

/// Merge search results using the specified strategy.
pub fn merge_results<'a>(
    lists: impl IntoIterator<Item = (&'a String, &'a Vec<SearchResult>)>,
    options: &MergeOptions,
) -> Vec<SearchResult> {
    match options.strategy {
        MergeStrategy::Rrf => merge_with_rrf(lists, options),
        MergeStrategy::Weighted => merge_with_weights(lists, options),
        MergeStrategy::Interleave => merge_with_interleave(lists, options),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{string::ToString, vec};

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
        let mut results_by_provider = BTreeMap::new();

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
