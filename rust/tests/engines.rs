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
    // 21 API engines + 11 HTML engines = 32 descriptor-driven engines.
    assert_eq!(ids.len(), 32);
    for id in [
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
fn internet_archive_builds_details_urls() {
    let body = r#"{"response":{"docs":[
        {"identifier":"some-item","title":"Some Item","description":"A scanned book"}
    ]}}"#;
    let results = parse("internet-archive", body);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].url, "https://archive.org/details/some-item");
    assert_eq!(results[0].title, "Some Item");
    assert_eq!(results[0].source, "internet-archive");
}

#[test]
fn semantic_scholar_falls_back_to_paper_url() {
    let body = r#"{"data":[
        {"title":"Graph Paper","paperId":"abc123","abstract":"On graphs."},
        {"title":"Direct","url":"https://example.com/p","abstract":"x"}
    ]}"#;
    let results = parse("semantic-scholar", body);
    assert_eq!(results.len(), 2);
    assert_eq!(
        results[0].url,
        "https://www.semanticscholar.org/paper/abc123"
    );
    assert_eq!(results[1].url, "https://example.com/p");
}

#[test]
fn gitlab_parses_projects_from_array_body() {
    let body = r#"[
        {"path_with_namespace":"group/proj","web_url":"https://gitlab.com/group/proj","description":"A project"}
    ]"#;
    let results = parse("gitlab", body);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "group/proj");
    assert_eq!(results[0].url, "https://gitlab.com/group/proj");
}

#[test]
fn codeberg_parses_projects_from_data_container() {
    let body = r#"{"data":[
        {"full_name":"user/repo","html_url":"https://codeberg.org/user/repo","description":"A repo"}
    ]}"#;
    let results = parse("codeberg", body);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "user/repo");
    assert_eq!(results[0].url, "https://codeberg.org/user/repo");
}

#[test]
fn dictionary_parses_canonical_definition() {
    let body = r#"<html><head>
        <link rel="canonical" href="https://dictionary.cambridge.org/dictionary/english/quantum" />
        <meta property="og:title" content="QUANTUM | meaning" />
        <meta name="description" content="the smallest amount of something" />
      </head><body></body></html>"#;
    let results = parse("cambridge-dictionary", body);
    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].url,
        "https://dictionary.cambridge.org/dictionary/english/quantum"
    );
    assert_eq!(results[0].title, "QUANTUM | meaning");
    assert_eq!(results[0].snippet, "the smallest amount of something");
    assert_eq!(results[0].source, "cambridge-dictionary");
}

#[test]
fn yandex_parses_organic_links() {
    let body =
        r#"<a class="Link OrganicTitle-Link" href="https://example.org/page">Example Result</a>"#;
    let results = parse("yandex", body);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].url, "https://example.org/page");
    assert_eq!(results[0].title, "Example Result");
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
