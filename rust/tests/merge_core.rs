//! Shared server/no_std ranking contract, using three independently ranked lists.
use std::collections::BTreeMap;
use web_search::{get_provider_ids, get_registry, merger::*, SearchResult};

fn result(provider: &str, path: &str, rank: usize) -> SearchResult {
    SearchResult {
        title: format!("{provider}:{path}"),
        url: format!("https://example.com/{path}"),
        snippet: String::new(),
        source: provider.into(),
        rank,
        score: None,
        sources: None,
    }
}

fn lists() -> BTreeMap<String, Vec<SearchResult>> {
    BTreeMap::from([
        (
            "alpha".into(),
            vec![result("alpha", "shared/", 1), result("alpha", "a", 2)],
        ),
        (
            "beta".into(),
            vec![result("beta", "b", 1), result("beta", "SHARED", 2)],
        ),
        (
            "gamma".into(),
            vec![result("gamma", "shared", 1), result("gamma", "c", 2)],
        ),
    ])
}

#[test]
fn three_provider_rrf_has_known_scores_and_stable_ties() {
    let merged = merge_results(&lists(), &MergeOptions::new());
    assert_eq!(merged.len(), 4);
    assert_eq!(normalize_url(&merged[0].url), "example.com/shared");
    assert!((merged[0].score.unwrap() - (2.0 / 61.0 + 1.0 / 62.0)).abs() < 1e-12);
    assert_eq!(
        merged[0].sources.as_ref().unwrap(),
        &["alpha", "beta", "gamma"]
    );
    assert_eq!(
        merged.iter().map(|r| r.rank).collect::<Vec<_>>(),
        [1, 2, 3, 4]
    );
    assert_eq!(
        merged
            .iter()
            .skip(1)
            .map(|r| normalize_url(&r.url))
            .collect::<Vec<_>>(),
        ["example.com/b", "example.com/a", "example.com/c"]
    );
}

#[test]
fn configurable_k_and_weights_apply_to_each_provider() {
    let options = MergeOptions::new()
        .with_rrf_k(10.0)
        .with_weights(BTreeMap::from([("beta".into(), 3.0)]));
    let merged = merge_results(&lists(), &options);
    assert!((merged[0].score.unwrap() - (2.0 / 11.0 + 3.0 / 12.0)).abs() < 1e-12);
    let weighted = merge_results(&lists(), &options.with_strategy(MergeStrategy::Weighted));
    assert!((weighted[0].score.unwrap() - 4.97).abs() < 1e-12);
}

#[test]
fn interleave_preserves_rounds_and_duplicate_option() {
    let options = MergeOptions::new().with_strategy(MergeStrategy::Interleave);
    let merged = merge_results(&lists(), &options);
    assert_eq!(
        merged
            .iter()
            .map(|r| normalize_url(&r.url))
            .collect::<Vec<_>>(),
        [
            "example.com/shared",
            "example.com/b",
            "example.com/a",
            "example.com/c"
        ]
    );
    let options = MergeOptions {
        remove_duplicates: false,
        ..options
    };
    assert_eq!(merge_results(&lists(), &options).len(), 6);
}

#[test]
fn normalization_keeps_existing_server_semantics() {
    assert_eq!(
        normalize_url("http://EXAMPLE.com/Path/?q=1#frag"),
        "example.com/path"
    );
    assert_eq!(normalize_url("Not a URL"), "not a url");
    assert_eq!(normalize_url("https://[::1]:443/path"), "[::1]/path");
    assert_eq!(
        normalize_url("https://bücher.example/path"),
        "xn--bcher-kva.example/path"
    );
}

#[test]
fn core_registry_is_complete_and_selectable() {
    let registry = get_registry();
    assert_eq!(registry.len(), 40);
    assert_eq!(
        get_provider_ids(Some("papers")),
        ["arxiv", "europepmc", "doaj"]
    );
    for entry in registry {
        assert!(
            !entry.endpoint_template.is_empty(),
            "{} lacks an endpoint",
            entry.id
        );
        assert!(matches!(entry.capabilities.method.as_str(), "GET" | "POST"));
    }
}

#[cfg(feature = "server")]
#[test]
fn hash_map_server_callers_produce_identical_results() {
    let ordered = lists();
    let hashed: std::collections::HashMap<_, _> = ordered.clone().into_iter().collect();
    for strategy in [
        MergeStrategy::Rrf,
        MergeStrategy::Weighted,
        MergeStrategy::Interleave,
    ] {
        let options = MergeOptions::new().with_strategy(strategy);
        let signature = |results: Vec<SearchResult>| {
            results
                .into_iter()
                .map(|r| (r.title, r.url, r.rank, r.score, r.sources))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            signature(merge_results(&ordered, &options)),
            signature(merge_results(&hashed, &options))
        );
    }
}
