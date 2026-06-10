//! Integration tests for the descriptor-driven engine parsers (Rust parity with
//! `tests/api-engines.test.js` and `tests/html-engines.test.js`). Parsers are
//! exercised through the public descriptor catalog, the same path the
//! `GenericProvider` uses at runtime.

use web_search::providers::{all_descriptor_engines, EngineDescriptor};
use web_search::SearchOptions;

fn descriptor(id: &str) -> EngineDescriptor {
    all_descriptor_engines()
        .into_iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| panic!("no descriptor for {id}"))
}

fn parse(id: &str, body: &str) -> Vec<web_search::SearchResult> {
    let d = descriptor(id);
    (d.parse)(body, 10, &SearchOptions::default())
}

#[test]
fn catalog_contains_all_engines() {
    let ids: Vec<&str> = all_descriptor_engines().iter().map(|d| d.id).collect();
    // 8 API engines + 6 HTML engines = 14 descriptor-driven engines.
    assert_eq!(ids.len(), 14);
    for id in [
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
    ] {
        assert!(ids.contains(&id), "missing descriptor {id}");
    }
}

#[test]
fn wikipedia_parses_pages() {
    let body = r#"{"pages":[
        {"key":"Cat","title":"Cat","excerpt":"A small <b>cat</b>"},
        {"key":"Dog","title":"Dog","description":"A dog"}
    ]}"#;
    let results = parse("wikipedia", body);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "Cat");
    assert_eq!(results[0].url, "https://en.wikipedia.org/wiki/Cat");
    assert_eq!(results[0].snippet, "A small cat");
    assert_eq!(results[0].rank, 1);
    // Falls back to `description` when `excerpt` is missing.
    assert_eq!(results[1].snippet, "A dog");
}

#[test]
fn github_parses_repositories() {
    let body = r#"{"items":[
        {"full_name":"rust-lang/rust","html_url":"https://github.com/rust-lang/rust","description":"The Rust language"}
    ]}"#;
    let results = parse("github", body);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "rust-lang/rust");
    assert_eq!(results[0].url, "https://github.com/rust-lang/rust");
    assert_eq!(results[0].source, "github");
}

#[test]
fn crossref_builds_doi_urls() {
    let body = r#"{"message":{"items":[
        {"title":["A Paper"],"DOI":"10.1/abc","container-title":["Journal"]}
    ]}}"#;
    let results = parse("crossref", body);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].url, "https://doi.org/10.1/abc");
    assert_eq!(results[0].title, "A Paper");
    // Falls back to the container title for the snippet.
    assert_eq!(results[0].snippet, "Journal");
}

#[test]
fn openalex_reconstructs_inverted_abstract() {
    let body = r#"{"results":[
        {"display_name":"Quantum","id":"https://openalex.org/W1",
         "abstract_inverted_index":{"Hello":[0],"quantum":[2],"world":[1]}}
    ]}"#;
    let results = parse("openalex", body);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Quantum");
    assert_eq!(results[0].url, "https://openalex.org/W1");
    assert_eq!(results[0].snippet, "Hello world quantum");
}

#[test]
fn arxiv_parses_atom_feed() {
    let body = r#"<feed>
      <entry>
        <title>Deep Learning</title>
        <id>http://arxiv.org/abs/1234.5678</id>
        <summary>A study of deep nets.</summary>
      </entry>
      <entry>
        <title>No Id Entry</title>
        <summary>skipped</summary>
      </entry>
    </feed>"#;
    let results = parse("arxiv", body);
    // The second entry has no <id> and is dropped.
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Deep Learning");
    assert_eq!(results[0].url, "http://arxiv.org/abs/1234.5678");
    assert_eq!(results[0].snippet, "A study of deep nets.");
}

#[test]
fn yahoo_resolves_redirect_hrefs() {
    let body = r#"<h3 class="title">
        <a href="https://r.search.yahoo.com/_ylt=abc/RU=https%3A%2F%2Fexample.com%2Fpage/RK=2/">Example Page</a>
      </h3>"#;
    let results = parse("yahoo", body);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].url, "https://example.com/page");
    assert_eq!(results[0].title, "Example Page");
}

#[test]
fn lite_parses_result_links_and_dedupes() {
    let body = r#"
      <a class="result-link" href="https://example.com/a">First</a>
      <a class="result-link" href="https://example.com/a">Duplicate</a>
      <a class="result-link" href="https://example.com/b">Second</a>
      <a class="result-link" href="https://duckduckgo.com/ad">Ad</a>
    "#;
    let results = parse("lite", body);
    // Duplicate URL deduped; duckduckgo.com self-link skipped.
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].url, "https://example.com/a");
    assert_eq!(results[0].title, "First");
    assert_eq!(results[1].url, "https://example.com/b");
}

#[test]
fn parsers_return_empty_on_malformed_bodies() {
    assert!(parse("wikipedia", "not json").is_empty());
    assert!(parse("github", "{}").is_empty());
    assert!(parse("arxiv", "<feed></feed>").is_empty());
    assert!(parse("lite", "<html></html>").is_empty());
}

#[test]
fn limit_is_respected() {
    let d = descriptor("wikipedia");
    let body = r#"{"pages":[
        {"key":"A","title":"A"},{"key":"B","title":"B"},{"key":"C","title":"C"}
    ]}"#;
    let results = (d.parse)(body, 2, &SearchOptions::default());
    assert_eq!(results.len(), 2);
}
