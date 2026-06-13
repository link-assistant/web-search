//! Search-engine descriptor catalog.
//!
//! A faithful Rust port of the JavaScript descriptor catalog
//! (`src/providers/api-engines.js` and `src/providers/html-engines.js`). Each
//! engine declares only its URL, request kind, and parser; the shared
//! [`GenericProvider`](super::generic::GenericProvider) performs all fetch,
//! decode, and error plumbing. Keeping both languages descriptor-driven is the
//! issue #3 parity requirement: a new engine added in one place is added in all
//! places.

use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use super::base::{SearchOptions, SearchResult};
use super::html_utils::{clean_text, parse_anchor_list, AnchorConfig};

/// How a response body is decoded before parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineKind {
    /// `application/json` body parsed with serde_json.
    Json,
    /// Plain-text/XML body (e.g. arXiv Atom).
    Text,
    /// HTML SERP scraped with a regex.
    Html,
}

/// HTTP method used for an engine request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    /// HTTP GET.
    Get,
    /// HTTP POST with a form-encoded body.
    Post,
}

/// Builds a request URL (or POST body) from the query and options.
pub type BuildFn = fn(&str, &SearchOptions) -> String;

/// Produces extra request headers (e.g. auth tokens) from the options.
pub type HeadersFn = fn(&SearchOptions) -> Vec<(String, String)>;

/// Parses a decoded response body into normalized results.
pub type ParseFn = fn(&str, usize, &SearchOptions) -> Vec<SearchResult>;

/// A declarative description of a single search engine.
#[derive(Clone, Copy)]
pub struct EngineDescriptor {
    /// Stable provider id.
    pub id: &'static str,
    /// Human-readable label.
    pub label: &'static str,
    /// Provider category (one of [`super::registry::CATEGORIES`]).
    pub category: &'static str,
    /// How the response body is decoded.
    pub kind: EngineKind,
    /// Whether the endpoint is browser-CORS readable.
    pub cors_readable: bool,
    /// Whether this is its category's default provider.
    pub default_for_category: bool,
    /// HTTP method.
    pub method: HttpMethod,
    /// Build the request URL from the query and options.
    pub build_url: BuildFn,
    /// Build an optional POST body.
    pub build_body: Option<BuildFn>,
    /// Extra request headers (e.g. auth tokens).
    pub headers: Option<HeadersFn>,
    /// Parse the decoded body into results.
    pub parse: ParseFn,
}

/// The descriptor `access` label derived from its [`EngineKind`].
pub fn access_for(kind: EngineKind) -> &'static str {
    match kind {
        EngineKind::Json | EngineKind::Text => "api",
        EngineKind::Html => "html",
    }
}

fn limit_of(options: &SearchOptions, max: usize) -> usize {
    options.limit.unwrap_or(10).min(max)
}

fn language_of(options: &SearchOptions) -> String {
    let lang = options.language.clone().unwrap_or_else(|| "en".to_string());
    lang.chars().take(12).collect()
}

fn make_result(source: &str, title: &str, url: &str, snippet: &str, rank: usize) -> SearchResult {
    let title = clean_text(title);
    SearchResult {
        title: if title.is_empty() {
            "Untitled".to_string()
        } else {
            title
        },
        url: url.to_string(),
        snippet: clean_text(snippet),
        source: source.to_string(),
        rank,
        score: None,
        sources: None,
    }
}

/// Reconstruct an abstract from OpenAlex's inverted-index representation.
pub fn reconstruct_inverted_abstract(inverted: &Value) -> String {
    let obj = match inverted.as_object() {
        Some(obj) => obj,
        None => return String::new(),
    };
    let mut slots: Vec<Option<&str>> = Vec::new();
    for (word, positions) in obj {
        if let Some(arr) = positions.as_array() {
            for pos in arr.iter().filter_map(Value::as_u64) {
                let idx = pos as usize;
                if idx >= slots.len() {
                    slots.resize(idx + 1, None);
                }
                slots[idx] = Some(word);
            }
        }
    }
    slots.into_iter().flatten().collect::<Vec<_>>().join(" ")
}

/// Decode a Yahoo redirect href (`.../RU=<encoded>/RK=...`) to its destination.
pub fn resolve_yahoo_href(href: &str) -> String {
    if href.is_empty() {
        return String::new();
    }
    static RU: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"/RU=([^/]+)/").unwrap());
    if let Some(caps) = RU.captures(href) {
        let encoded = &caps[1];
        return urlencoding::decode(encoded)
            .map(|c| c.into_owned())
            .unwrap_or_else(|_| encoded.to_string());
    }
    href.to_string()
}

/// Parse an arXiv Atom feed into normalized results.
pub fn parse_arxiv_atom(xml: &str, limit: usize) -> Vec<SearchResult> {
    static ENTRY: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?s)<entry>(.*?)</entry>").unwrap());
    static TITLE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?s)<title>(.*?)</title>").unwrap());
    static ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<id>(.*?)</id>").unwrap());
    static SUMMARY: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?s)<summary>(.*?)</summary>").unwrap());

    let mut results = Vec::new();
    for caps in ENTRY.captures_iter(xml) {
        if results.len() >= limit {
            break;
        }
        let entry = &caps[1];
        let id = ID
            .captures(entry)
            .map(|c| c[1].trim().to_string())
            .unwrap_or_default();
        if id.is_empty() {
            continue;
        }
        let title = TITLE
            .captures(entry)
            .map(|c| c[1].to_string())
            .unwrap_or_default();
        let summary = SUMMARY
            .captures(entry)
            .map(|c| c[1].to_string())
            .unwrap_or_default();
        let rank = results.len() + 1;
        results.push(make_result("arxiv", &title, &id, &summary, rank));
    }
    results
}

fn json(body: &str) -> Value {
    serde_json::from_str(body).unwrap_or(Value::Null)
}

fn str_field<'a>(item: &'a Value, key: &str) -> &'a str {
    item.get(key).and_then(Value::as_str).unwrap_or("")
}

/// Project a JSON array into normalized results.
///
/// Centralizes the slice/rank/filter loop (mirrors the JavaScript
/// `listResults` helper): each engine only declares how to turn one raw item
/// into `(title, url, snippet)`, and items whose `url` is empty are dropped.
fn list_results<F>(
    source: &str,
    items: Option<&Value>,
    limit: usize,
    project: F,
) -> Vec<SearchResult>
where
    F: Fn(&Value) -> (String, String, String),
{
    let arr = match items.and_then(Value::as_array) {
        Some(arr) => arr,
        None => return Vec::new(),
    };
    let mut out = Vec::new();
    for item in arr {
        if out.len() >= limit {
            break;
        }
        let (title, url, snippet) = project(item);
        if url.is_empty() {
            continue;
        }
        let rank = out.len() + 1;
        out.push(make_result(source, &title, &url, &snippet, rank));
    }
    out
}

/// Normalize a code-host repository list into results (mirrors the JavaScript
/// `repoResults` helper).
fn repo_results(
    source: &str,
    data: &Value,
    limit: usize,
    container: Option<&str>,
    title_field: &str,
    url_field: &str,
) -> Vec<SearchResult> {
    let items = match container {
        Some(key) => data.get(key),
        None => Some(data),
    };
    list_results(source, items, limit, |it| {
        let title = if str_field(it, title_field).is_empty() {
            str_field(it, "name")
        } else {
            str_field(it, title_field)
        };
        (
            title.to_string(),
            str_field(it, url_field).to_string(),
            str_field(it, "description").to_string(),
        )
    })
}

// ---------------------------------------------------------------------------
// API engines
// ---------------------------------------------------------------------------

/// Build a MediaWiki REST `search/page` URL for a given host domain.
///
/// Wikipedia, Wiktionary, and Wikinews share the same CORS-readable REST
/// endpoint and article-URL shape, differing only by host domain (issue #5 /
/// formal-ai parity). Rust's fn-pointer descriptors cannot capture the domain
/// in a closure, so each engine has a thin wrapper around these helpers.
fn mediawiki_url(domain: &str, query: &str, options: &SearchOptions) -> String {
    let limit = limit_of(options, 100);
    let lang = language_of(options);
    format!(
        "https://{lang}.{domain}/w/rest.php/v1/search/page?q={}&limit={limit}",
        urlencoding::encode(query)
    )
}

fn mediawiki_parse(
    source: &str,
    domain: &str,
    body: &str,
    limit: usize,
    options: &SearchOptions,
) -> Vec<SearchResult> {
    let lang = language_of(options);
    let data = json(body);
    list_results(source, data.get("pages"), limit, |p| {
        let key = str_field(p, "key");
        let url = format!("https://{lang}.{domain}/wiki/{}", urlencoding::encode(key));
        let snippet = if str_field(p, "excerpt").is_empty() {
            str_field(p, "description")
        } else {
            str_field(p, "excerpt")
        };
        (str_field(p, "title").to_string(), url, snippet.to_string())
    })
}

fn wikipedia_url(query: &str, options: &SearchOptions) -> String {
    mediawiki_url("wikipedia.org", query, options)
}

fn wikipedia_parse(body: &str, limit: usize, options: &SearchOptions) -> Vec<SearchResult> {
    mediawiki_parse("wikipedia", "wikipedia.org", body, limit, options)
}

fn wiktionary_url(query: &str, options: &SearchOptions) -> String {
    mediawiki_url("wiktionary.org", query, options)
}

fn wiktionary_parse(body: &str, limit: usize, options: &SearchOptions) -> Vec<SearchResult> {
    mediawiki_parse("wiktionary", "wiktionary.org", body, limit, options)
}

fn wikinews_url(query: &str, options: &SearchOptions) -> String {
    mediawiki_url("wikinews.org", query, options)
}

fn wikinews_parse(body: &str, limit: usize, options: &SearchOptions) -> Vec<SearchResult> {
    mediawiki_parse("wikinews", "wikinews.org", body, limit, options)
}

fn wikidata_url(query: &str, options: &SearchOptions) -> String {
    let limit = limit_of(options, 50);
    let lang = language_of(options);
    format!(
        "https://www.wikidata.org/w/api.php?action=wbsearchentities&format=json&language={lang}&uselang={lang}&limit={limit}&search={}",
        urlencoding::encode(query)
    )
}

fn wikidata_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    json(body)
        .get("search")
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .take(limit)
                .enumerate()
                .map(|(i, e)| {
                    let id = str_field(e, "id");
                    let title = if str_field(e, "label").is_empty() {
                        id
                    } else {
                        str_field(e, "label")
                    };
                    let url = if str_field(e, "concepturi").is_empty() {
                        format!("https://www.wikidata.org/wiki/{id}")
                    } else {
                        str_field(e, "concepturi").to_string()
                    };
                    make_result("wikidata", title, &url, str_field(e, "description"), i + 1)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn searx_url(query: &str, _options: &SearchOptions) -> String {
    format!(
        "https://searx.be/search?format=json&q={}",
        urlencoding::encode(query)
    )
}

fn searx_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    json(body)
        .get("results")
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .take(limit)
                .enumerate()
                .map(|(i, e)| {
                    make_result(
                        "searx",
                        str_field(e, "title"),
                        str_field(e, "url"),
                        str_field(e, "content"),
                        i + 1,
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn crossref_url(query: &str, options: &SearchOptions) -> String {
    let rows = limit_of(options, 50);
    format!(
        "https://api.crossref.org/works?rows={rows}&query={}",
        urlencoding::encode(query)
    )
}

fn first_str(value: Option<&Value>) -> String {
    match value {
        Some(Value::Array(arr)) => arr
            .first()
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}

fn crossref_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    json(body)
        .get("message")
        .and_then(|m| m.get("items"))
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .take(limit)
                .enumerate()
                .filter_map(|(i, it)| {
                    let title = first_str(it.get("title"));
                    let url = if !str_field(it, "URL").is_empty() {
                        str_field(it, "URL").to_string()
                    } else {
                        let doi = str_field(it, "DOI");
                        if doi.is_empty() {
                            String::new()
                        } else {
                            format!("https://doi.org/{doi}")
                        }
                    };
                    if url.is_empty() {
                        return None;
                    }
                    let snippet = if str_field(it, "abstract").is_empty() {
                        first_str(it.get("container-title"))
                    } else {
                        str_field(it, "abstract").to_string()
                    };
                    Some(make_result("crossref", &title, &url, &snippet, i + 1))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn openalex_url(query: &str, options: &SearchOptions) -> String {
    let per_page = limit_of(options, 50);
    format!(
        "https://api.openalex.org/works?per-page={per_page}&search={}",
        urlencoding::encode(query)
    )
}

fn openalex_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    json(body)
        .get("results")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .take(limit)
                .enumerate()
                .filter_map(|(i, it)| {
                    let title = if str_field(it, "title").is_empty() {
                        str_field(it, "display_name")
                    } else {
                        str_field(it, "title")
                    };
                    let url = if str_field(it, "doi").is_empty() {
                        str_field(it, "id")
                    } else {
                        str_field(it, "doi")
                    };
                    if url.is_empty() {
                        return None;
                    }
                    let snippet = it
                        .get("abstract_inverted_index")
                        .map(reconstruct_inverted_abstract)
                        .unwrap_or_default();
                    Some(make_result("openalex", title, url, &snippet, i + 1))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn github_url(query: &str, options: &SearchOptions) -> String {
    let per_page = limit_of(options, 50);
    format!(
        "https://api.github.com/search/repositories?per_page={per_page}&q={}",
        urlencoding::encode(query)
    )
}

fn github_headers(_options: &SearchOptions) -> Vec<(String, String)> {
    let mut headers = vec![
        (
            "Accept".to_string(),
            "application/vnd.github+json".to_string(),
        ),
        ("X-GitHub-Api-Version".to_string(), "2022-11-28".to_string()),
    ];
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        if !token.is_empty() {
            headers.push(("Authorization".to_string(), format!("Bearer {token}")));
        }
    }
    headers
}

fn github_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    json(body)
        .get("items")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .take(limit)
                .enumerate()
                .map(|(i, it)| {
                    let title = if str_field(it, "full_name").is_empty() {
                        str_field(it, "name")
                    } else {
                        str_field(it, "full_name")
                    };
                    make_result(
                        "github",
                        title,
                        str_field(it, "html_url"),
                        str_field(it, "description"),
                        i + 1,
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn hackernews_url(query: &str, options: &SearchOptions) -> String {
    let hits = limit_of(options, 50);
    format!(
        "https://hn.algolia.com/api/v1/search?hitsPerPage={hits}&query={}",
        urlencoding::encode(query)
    )
}

fn hackernews_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    json(body)
        .get("hits")
        .and_then(Value::as_array)
        .map(|hits| {
            hits.iter()
                .take(limit)
                .enumerate()
                .map(|(i, h)| {
                    let title = if str_field(h, "title").is_empty() {
                        str_field(h, "story_title")
                    } else {
                        str_field(h, "title")
                    };
                    let url = if !str_field(h, "url").is_empty() {
                        str_field(h, "url").to_string()
                    } else if !str_field(h, "story_url").is_empty() {
                        str_field(h, "story_url").to_string()
                    } else {
                        format!(
                            "https://news.ycombinator.com/item?id={}",
                            str_field(h, "objectID")
                        )
                    };
                    let snippet = if str_field(h, "story_text").is_empty() {
                        str_field(h, "comment_text")
                    } else {
                        str_field(h, "story_text")
                    };
                    make_result("hackernews", title, &url, snippet, i + 1)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn arxiv_url(query: &str, options: &SearchOptions) -> String {
    let max = limit_of(options, 50);
    format!(
        "http://export.arxiv.org/api/query?max_results={max}&search_query={}",
        urlencoding::encode(&format!("all:{query}"))
    )
}

fn arxiv_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_arxiv_atom(body, limit)
}

fn internet_archive_url(query: &str, options: &SearchOptions) -> String {
    let rows = limit_of(options, 50);
    format!(
        "https://archive.org/advancedsearch.php?q={}&fl[]=identifier&fl[]=title&fl[]=description&rows={rows}&page=1&output=json",
        urlencoding::encode(query)
    )
}

fn internet_archive_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    let data = json(body);
    let docs = data.get("response").and_then(|r| r.get("docs"));
    list_results("internet-archive", docs, limit, |d| {
        let identifier = str_field(d, "identifier");
        let url = if identifier.is_empty() {
            String::new()
        } else {
            format!("https://archive.org/details/{identifier}")
        };
        (
            first_str(d.get("title")),
            url,
            first_str(d.get("description")),
        )
    })
}

fn dbpedia_url(query: &str, options: &SearchOptions) -> String {
    let max = limit_of(options, 50);
    format!(
        "https://lookup.dbpedia.org/api/search?format=json&maxResults={max}&query={}",
        urlencoding::encode(query)
    )
}

fn dbpedia_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    let data = json(body);
    list_results("dbpedia", data.get("docs"), limit, |d| {
        (
            first_str(d.get("label")),
            first_str(d.get("resource")),
            first_str(d.get("comment")),
        )
    })
}

fn openlibrary_url(query: &str, options: &SearchOptions) -> String {
    let limit = limit_of(options, 50);
    format!(
        "https://openlibrary.org/search.json?q={}&limit={limit}",
        urlencoding::encode(query)
    )
}

fn openlibrary_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    let data = json(body);
    list_results("openlibrary", data.get("docs"), limit, |d| {
        let key = str_field(d, "key");
        let url = if key.is_empty() {
            String::new()
        } else {
            format!("https://openlibrary.org{key}")
        };
        let authors = d
            .get("author_name")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();
        (str_field(d, "title").to_string(), url, authors)
    })
}

fn semantic_scholar_url(query: &str, options: &SearchOptions) -> String {
    let limit = limit_of(options, 50);
    format!(
        "https://api.semanticscholar.org/graph/v1/paper/search?query={}&limit={limit}&fields=title,abstract,url,year",
        urlencoding::encode(query)
    )
}

fn semantic_scholar_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    let data = json(body);
    list_results("semantic-scholar", data.get("data"), limit, |p| {
        let url = if !str_field(p, "url").is_empty() {
            str_field(p, "url").to_string()
        } else {
            let paper_id = str_field(p, "paperId");
            if paper_id.is_empty() {
                String::new()
            } else {
                format!("https://www.semanticscholar.org/paper/{paper_id}")
            }
        };
        (
            str_field(p, "title").to_string(),
            url,
            str_field(p, "abstract").to_string(),
        )
    })
}

fn europepmc_url(query: &str, options: &SearchOptions) -> String {
    let page_size = limit_of(options, 50);
    format!(
        "https://www.ebi.ac.uk/europepmc/webservices/rest/search?query={}&format=json&pageSize={page_size}",
        urlencoding::encode(query)
    )
}

fn europepmc_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    let data = json(body);
    let results = data.get("resultList").and_then(|r| r.get("result"));
    list_results("europepmc", results, limit, |r| {
        let doi = str_field(r, "doi");
        let url = if !doi.is_empty() {
            format!("https://doi.org/{doi}")
        } else {
            let id = str_field(r, "id");
            let source = str_field(r, "source");
            if id.is_empty() || source.is_empty() {
                String::new()
            } else {
                format!("https://europepmc.org/article/{source}/{id}")
            }
        };
        (
            str_field(r, "title").to_string(),
            url,
            str_field(r, "abstractText").to_string(),
        )
    })
}

fn doaj_url(query: &str, options: &SearchOptions) -> String {
    let page_size = limit_of(options, 50);
    format!(
        "https://doaj.org/api/search/articles/{}?pageSize={page_size}",
        urlencoding::encode(query)
    )
}

fn doaj_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    let data = json(body);
    list_results("doaj", data.get("results"), limit, |r| {
        let bibjson = r.get("bibjson");
        let title = bibjson
            .and_then(|b| b.get("title"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let abstract_text = bibjson
            .and_then(|b| b.get("abstract"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let links = bibjson
            .and_then(|b| b.get("link"))
            .and_then(Value::as_array);
        let url = links
            .map(|arr| {
                let chosen = arr
                    .iter()
                    .find(|l| str_field(l, "type") == "fulltext")
                    .or_else(|| arr.first());
                chosen
                    .map(|l| str_field(l, "url").to_string())
                    .unwrap_or_default()
            })
            .filter(|u| !u.is_empty())
            .unwrap_or_else(|| {
                let id = str_field(r, "id");
                if id.is_empty() {
                    String::new()
                } else {
                    format!("https://doaj.org/article/{id}")
                }
            });
        (title, url, abstract_text)
    })
}

fn gitlab_url(query: &str, options: &SearchOptions) -> String {
    let per_page = limit_of(options, 50);
    format!(
        "https://gitlab.com/api/v4/projects?search={}&per_page={per_page}&order_by=star_count&sort=desc",
        urlencoding::encode(query)
    )
}

fn gitlab_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    repo_results(
        "gitlab",
        &json(body),
        limit,
        None,
        "path_with_namespace",
        "web_url",
    )
}

fn codeberg_url(query: &str, options: &SearchOptions) -> String {
    let limit = limit_of(options, 50);
    format!(
        "https://codeberg.org/api/v1/repos/search?q={}&limit={limit}",
        urlencoding::encode(query)
    )
}

fn codeberg_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    repo_results(
        "codeberg",
        &json(body),
        limit,
        Some("data"),
        "full_name",
        "html_url",
    )
}

fn gitee_url(query: &str, options: &SearchOptions) -> String {
    let per_page = limit_of(options, 50);
    format!(
        "https://gitee.com/api/v5/search/repositories?q={}&per_page={per_page}",
        urlencoding::encode(query)
    )
}

fn gitee_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    repo_results("gitee", &json(body), limit, None, "full_name", "html_url")
}

fn bitbucket_url(query: &str, options: &SearchOptions) -> String {
    let pagelen = limit_of(options, 50);
    let query_expr = format!("name~\"{query}\"");
    let q = urlencoding::encode(&query_expr);
    format!(
        "https://api.bitbucket.org/2.0/repositories?q={q}&pagelen={pagelen}&fields=values.full_name,values.links.html.href,values.description"
    )
}

fn bitbucket_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    let data = json(body);
    list_results("bitbucket", data.get("values"), limit, |v| {
        let url = v
            .get("links")
            .and_then(|l| l.get("html"))
            .and_then(|h| h.get("href"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        (
            str_field(v, "full_name").to_string(),
            url,
            str_field(v, "description").to_string(),
        )
    })
}

fn gitflic_url(query: &str, options: &SearchOptions) -> String {
    let size = limit_of(options, 50);
    format!(
        "https://api.gitflic.ru/project?query={}&size={size}",
        urlencoding::encode(query)
    )
}

fn gitflic_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    let data = json(body);
    let items = data
        .get("_embedded")
        .and_then(|e| e.get("projectList"))
        .or_else(|| data.get("items"))
        .or_else(|| data.get("results"))
        .or(Some(&data));
    list_results("gitflic", items, limit, |it| {
        let title = [
            str_field(it, "title"),
            str_field(it, "name"),
            str_field(it, "alias"),
        ]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or("")
        .to_string();
        let url = if !str_field(it, "url").is_empty() {
            str_field(it, "url").to_string()
        } else if let Some(href) = it
            .get("_links")
            .and_then(|l| l.get("self"))
            .and_then(|s| s.get("href"))
            .and_then(Value::as_str)
        {
            href.to_string()
        } else {
            let owner = str_field(it, "owner");
            let alias = str_field(it, "alias");
            if owner.is_empty() || alias.is_empty() {
                String::new()
            } else {
                format!("https://gitflic.ru/project/{owner}/{alias}")
            }
        };
        (title, url, str_field(it, "description").to_string())
    })
}

// ---------------------------------------------------------------------------
// HTML engines
// ---------------------------------------------------------------------------

fn skip_mojeek(url: &str) -> bool {
    url.contains("mojeek.com")
}

fn skip_ecosia(url: &str) -> bool {
    url.contains("ecosia.org")
}

fn skip_startpage(url: &str) -> bool {
    url.contains("startpage.com")
}

fn skip_lite(url: &str) -> bool {
    url.contains("duckduckgo.com")
}

fn brave_url(query: &str, options: &SearchOptions) -> String {
    let mut url = format!(
        "https://search.brave.com/search?q={}",
        urlencoding::encode(query)
    );
    match options.safe_search {
        Some(true) => url.push_str("&safesearch=strict"),
        Some(false) => url.push_str("&safesearch=off"),
        None => {}
    }
    url
}

static BRAVE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<a[^>]+href="(https?://[^"]+)"[^>]*class="[^"]*result-header[^"]*"[^>]*>.*?<span[^>]*class="[^"]*title[^"]*"[^>]*>(.*?)</span>.*?<p[^>]*class="[^"]*snippet[^"]*"[^>]*>(.*?)</p>"#).unwrap()
});

fn brave_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_anchor_list(
        html,
        &AnchorConfig {
            source: "brave",
            limit,
            item_regex: &BRAVE_RE,
            url_group: 1,
            title_group: 2,
            snippet_group: Some(3),
            url_transform: None,
            skip: Some(|url| url.contains("search.brave.com")),
        },
    )
}

fn mojeek_url(query: &str, options: &SearchOptions) -> String {
    let mut url = format!(
        "https://www.mojeek.com/search?q={}",
        urlencoding::encode(query)
    );
    if options.safe_search == Some(false) {
        url.push_str("&safe=0");
    }
    url
}

static MOJEEK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<a[^>]+href="(https?://[^"]+)"[^>]*class="[^"]*ob[^"]*"[^>]*>(.*?)</a>.*?<p[^>]*class="[^"]*s[^"]*"[^>]*>(.*?)</p>"#).unwrap()
});

fn mojeek_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_anchor_list(
        html,
        &AnchorConfig {
            source: "mojeek",
            limit,
            item_regex: &MOJEEK_RE,
            url_group: 1,
            title_group: 2,
            snippet_group: Some(3),
            url_transform: None,
            skip: Some(skip_mojeek),
        },
    )
}

fn ecosia_url(query: &str, options: &SearchOptions) -> String {
    let mut url = format!(
        "https://www.ecosia.org/search?q={}",
        urlencoding::encode(query)
    );
    if let Some(ref lang) = options.language {
        url.push_str(&format!("&hl={lang}"));
    }
    url
}

static ECOSIA_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<a[^>]+class="[^"]*result__link[^"]*"[^>]*href="(https?://[^"]+)"[^>]*>(.*?)</a>.*?<(?:p|div)[^>]*class="[^"]*result__description[^"]*"[^>]*>(.*?)</(?:p|div)>"#).unwrap()
});

fn ecosia_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_anchor_list(
        html,
        &AnchorConfig {
            source: "ecosia",
            limit,
            item_regex: &ECOSIA_RE,
            url_group: 1,
            title_group: 2,
            snippet_group: Some(3),
            url_transform: None,
            skip: Some(skip_ecosia),
        },
    )
}

fn startpage_url(query: &str, options: &SearchOptions) -> String {
    let mut url = format!(
        "https://www.startpage.com/sp/search?query={}",
        urlencoding::encode(query)
    );
    if let Some(ref lang) = options.language {
        url.push_str(&format!("&language={lang}"));
    }
    url
}

static STARTPAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<a[^>]+class="[^"]*result-title[^"]*"[^>]*href="(https?://[^"]+)"[^>]*>(.*?)</a>.*?<p[^>]*class="[^"]*description[^"]*"[^>]*>(.*?)</p>"#).unwrap()
});

fn startpage_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_anchor_list(
        html,
        &AnchorConfig {
            source: "startpage",
            limit,
            item_regex: &STARTPAGE_RE,
            url_group: 1,
            title_group: 2,
            snippet_group: Some(3),
            url_transform: None,
            skip: Some(skip_startpage),
        },
    )
}

fn yahoo_url(query: &str, options: &SearchOptions) -> String {
    let mut url = format!(
        "https://search.yahoo.com/search?p={}",
        urlencoding::encode(query)
    );
    if let Some(ref region) = options.region {
        url.push_str(&format!("&vc={region}"));
    }
    url
}

static YAHOO_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?s)<h3[^>]*class="[^"]*title[^"]*"[^>]*>.*?<a[^>]+href="([^"]+)"[^>]*>(.*?)</a>"#,
    )
    .unwrap()
});

fn yahoo_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_anchor_list(
        html,
        &AnchorConfig {
            source: "yahoo",
            limit,
            item_regex: &YAHOO_RE,
            url_group: 1,
            title_group: 2,
            snippet_group: None,
            url_transform: Some(resolve_yahoo_href),
            skip: Some(|url| url.is_empty() || url.contains("yahoo.com")),
        },
    )
}

fn lite_url(_query: &str, _options: &SearchOptions) -> String {
    "https://lite.duckduckgo.com/lite/".to_string()
}

fn lite_body(query: &str, options: &SearchOptions) -> String {
    let mut body = format!("q={}", urlencoding::encode(query));
    if let Some(ref region) = options.region {
        body.push_str(&format!("&kl={region}"));
    }
    body
}

static LITE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?s)<a[^>]+class="[^"]*result-link[^"]*"[^>]*href="(https?://[^"]+)"[^>]*>(.*?)</a>"#,
    )
    .unwrap()
});

fn lite_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_anchor_list(
        html,
        &AnchorConfig {
            source: "lite",
            limit,
            item_regex: &LITE_RE,
            url_group: 1,
            title_group: 2,
            snippet_group: None,
            url_transform: None,
            skip: Some(skip_lite),
        },
    )
}

fn yandex_url(query: &str, options: &SearchOptions) -> String {
    let mut url = format!(
        "https://yandex.com/search/?text={}",
        urlencoding::encode(query)
    );
    if let Some(ref lang) = options.language {
        url.push_str(&format!("&lang={lang}"));
    }
    url
}

static YANDEX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?s)<a[^>]+class="[^"]*OrganicTitle-Link[^"]*"[^>]*href="(https?://[^"]+)"[^>]*>(.*?)</a>"#,
    )
    .unwrap()
});

fn yandex_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_anchor_list(
        html,
        &AnchorConfig {
            source: "yandex",
            limit,
            item_regex: &YANDEX_RE,
            url_group: 1,
            title_group: 2,
            snippet_group: None,
            url_transform: None,
            skip: Some(|url| url.contains("yandex.")),
        },
    )
}

/// Extract a single canonical definition result from a dictionary SERP.
///
/// Dictionary providers resolve a headword to one authoritative entry page, so
/// the parser only needs the canonical URL (from `<link rel="canonical">` or
/// the `og:url` meta tag), a title, and the page description as the gloss
/// snippet (mirrors the JavaScript `parseDefinition` helper).
fn parse_definition(source: &str, html: &str, limit: usize) -> Vec<SearchResult> {
    if limit < 1 || html.is_empty() {
        return Vec::new();
    }
    static CANONICAL: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"(?i)<link[^>]+rel="canonical"[^>]+href="([^"]+)""#).unwrap()
    });
    static OG_URL: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"(?i)<meta[^>]+property="og:url"[^>]+content="([^"]+)""#).unwrap()
    });
    static OG_TITLE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"(?i)<meta[^>]+property="og:title"[^>]+content="([^"]+)""#).unwrap()
    });
    static TITLE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?is)<title>(.*?)</title>").unwrap());
    static DESCRIPTION: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r#"(?i)<meta[^>]+(?:name="description"|property="og:description")[^>]+content="([^"]+)""#,
        )
        .unwrap()
    });

    let url = CANONICAL
        .captures(html)
        .or_else(|| OG_URL.captures(html))
        .map(|c| c[1].to_string())
        .unwrap_or_default();
    if url.is_empty() {
        return Vec::new();
    }
    let title = OG_TITLE
        .captures(html)
        .or_else(|| TITLE.captures(html))
        .map(|c| c[1].to_string())
        .unwrap_or_default();
    let snippet = DESCRIPTION
        .captures(html)
        .map(|c| c[1].to_string())
        .unwrap_or_default();
    vec![make_result(source, &title, &url, &snippet, 1)]
}

fn cambridge_url(query: &str, _options: &SearchOptions) -> String {
    format!(
        "https://dictionary.cambridge.org/dictionary/english/{}",
        urlencoding::encode(query)
    )
}

fn cambridge_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_definition("cambridge-dictionary", html, limit)
}

fn merriam_url(query: &str, _options: &SearchOptions) -> String {
    format!(
        "https://www.merriam-webster.com/dictionary/{}",
        urlencoding::encode(query)
    )
}

fn merriam_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_definition("merriam-webster", html, limit)
}

fn dictionary_com_url(query: &str, _options: &SearchOptions) -> String {
    format!(
        "https://www.dictionary.com/browse/{}",
        urlencoding::encode(query)
    )
}

fn dictionary_com_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_definition("dictionary-com", html, limit)
}

fn collins_url(query: &str, _options: &SearchOptions) -> String {
    format!(
        "https://www.collinsdictionary.com/dictionary/english/{}",
        urlencoding::encode(query)
    )
}

fn collins_parse(html: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    parse_definition("collins-dictionary", html, limit)
}

/// All API-based engine descriptors, in catalog order.
pub fn api_engines() -> Vec<EngineDescriptor> {
    vec![
        EngineDescriptor {
            id: "wikipedia",
            label: "Wikipedia",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: true,
            method: HttpMethod::Get,
            build_url: wikipedia_url,
            build_body: None,
            headers: None,
            parse: wikipedia_parse,
        },
        EngineDescriptor {
            id: "wikidata",
            label: "Wikidata",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: wikidata_url,
            build_body: None,
            headers: None,
            parse: wikidata_parse,
        },
        EngineDescriptor {
            id: "wiktionary",
            label: "Wiktionary",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: wiktionary_url,
            build_body: None,
            headers: None,
            parse: wiktionary_parse,
        },
        EngineDescriptor {
            id: "wikinews",
            label: "Wikinews",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: wikinews_url,
            build_body: None,
            headers: None,
            parse: wikinews_parse,
        },
        EngineDescriptor {
            id: "internet-archive",
            label: "Internet Archive",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: internet_archive_url,
            build_body: None,
            headers: None,
            parse: internet_archive_parse,
        },
        EngineDescriptor {
            id: "dbpedia",
            label: "DBpedia",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: dbpedia_url,
            build_body: None,
            headers: None,
            parse: dbpedia_parse,
        },
        EngineDescriptor {
            id: "openlibrary",
            label: "Open Library",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: openlibrary_url,
            build_body: None,
            headers: None,
            parse: openlibrary_parse,
        },
        EngineDescriptor {
            id: "semantic-scholar",
            label: "Semantic Scholar",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: semantic_scholar_url,
            build_body: None,
            headers: None,
            parse: semantic_scholar_parse,
        },
        EngineDescriptor {
            id: "openalex",
            label: "OpenAlex",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: openalex_url,
            build_body: None,
            headers: None,
            parse: openalex_parse,
        },
        EngineDescriptor {
            id: "crossref",
            label: "Crossref",
            category: "knowledge",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: crossref_url,
            build_body: None,
            headers: None,
            parse: crossref_parse,
        },
        EngineDescriptor {
            id: "searx",
            label: "SearXNG",
            category: "search",
            kind: EngineKind::Json,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: searx_url,
            build_body: None,
            headers: None,
            parse: searx_parse,
        },
        EngineDescriptor {
            id: "arxiv",
            label: "arXiv",
            category: "papers",
            kind: EngineKind::Text,
            cors_readable: true,
            default_for_category: true,
            method: HttpMethod::Get,
            build_url: arxiv_url,
            build_body: None,
            headers: None,
            parse: arxiv_parse,
        },
        EngineDescriptor {
            id: "europepmc",
            label: "Europe PMC",
            category: "papers",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: europepmc_url,
            build_body: None,
            headers: None,
            parse: europepmc_parse,
        },
        EngineDescriptor {
            id: "doaj",
            label: "DOAJ",
            category: "papers",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: doaj_url,
            build_body: None,
            headers: None,
            parse: doaj_parse,
        },
        EngineDescriptor {
            id: "github",
            label: "GitHub",
            category: "code",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: true,
            method: HttpMethod::Get,
            build_url: github_url,
            build_body: None,
            headers: Some(github_headers),
            parse: github_parse,
        },
        EngineDescriptor {
            id: "hackernews",
            label: "Hacker News",
            category: "code",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: hackernews_url,
            build_body: None,
            headers: None,
            parse: hackernews_parse,
        },
        EngineDescriptor {
            id: "gitlab",
            label: "GitLab",
            category: "code",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: gitlab_url,
            build_body: None,
            headers: None,
            parse: gitlab_parse,
        },
        EngineDescriptor {
            id: "codeberg",
            label: "Codeberg",
            category: "code",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: codeberg_url,
            build_body: None,
            headers: None,
            parse: codeberg_parse,
        },
        EngineDescriptor {
            id: "gitee",
            label: "Gitee",
            category: "code",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: gitee_url,
            build_body: None,
            headers: None,
            parse: gitee_parse,
        },
        EngineDescriptor {
            id: "bitbucket",
            label: "Bitbucket",
            category: "code",
            kind: EngineKind::Json,
            cors_readable: true,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: bitbucket_url,
            build_body: None,
            headers: None,
            parse: bitbucket_parse,
        },
        EngineDescriptor {
            id: "gitflic",
            label: "GitFlic",
            category: "code",
            kind: EngineKind::Json,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: gitflic_url,
            build_body: None,
            headers: None,
            parse: gitflic_parse,
        },
    ]
}

/// All HTML-scraping engine descriptors, in catalog order.
pub fn html_engines() -> Vec<EngineDescriptor> {
    vec![
        EngineDescriptor {
            id: "brave",
            label: "Brave Search",
            category: "search",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: brave_url,
            build_body: None,
            headers: None,
            parse: brave_parse,
        },
        EngineDescriptor {
            id: "mojeek",
            label: "Mojeek",
            category: "search",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: mojeek_url,
            build_body: None,
            headers: None,
            parse: mojeek_parse,
        },
        EngineDescriptor {
            id: "ecosia",
            label: "Ecosia",
            category: "search",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: ecosia_url,
            build_body: None,
            headers: None,
            parse: ecosia_parse,
        },
        EngineDescriptor {
            id: "startpage",
            label: "Startpage",
            category: "search",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: startpage_url,
            build_body: None,
            headers: None,
            parse: startpage_parse,
        },
        EngineDescriptor {
            id: "yahoo",
            label: "Yahoo Search",
            category: "search",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: yahoo_url,
            build_body: None,
            headers: None,
            parse: yahoo_parse,
        },
        EngineDescriptor {
            id: "yandex",
            label: "Yandex",
            category: "search",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: yandex_url,
            build_body: None,
            headers: None,
            parse: yandex_parse,
        },
        EngineDescriptor {
            id: "cambridge-dictionary",
            label: "Cambridge Dictionary",
            category: "knowledge",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: cambridge_url,
            build_body: None,
            headers: None,
            parse: cambridge_parse,
        },
        EngineDescriptor {
            id: "merriam-webster",
            label: "Merriam-Webster",
            category: "knowledge",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: merriam_url,
            build_body: None,
            headers: None,
            parse: merriam_parse,
        },
        EngineDescriptor {
            id: "dictionary-com",
            label: "Dictionary.com",
            category: "knowledge",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: dictionary_com_url,
            build_body: None,
            headers: None,
            parse: dictionary_com_parse,
        },
        EngineDescriptor {
            id: "collins-dictionary",
            label: "Collins Dictionary",
            category: "knowledge",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Get,
            build_url: collins_url,
            build_body: None,
            headers: None,
            parse: collins_parse,
        },
        EngineDescriptor {
            id: "lite",
            label: "DuckDuckGo Lite",
            category: "search",
            kind: EngineKind::Html,
            cors_readable: false,
            default_for_category: false,
            method: HttpMethod::Post,
            build_url: lite_url,
            build_body: Some(lite_body),
            headers: None,
            parse: lite_parse,
        },
    ]
}

/// All descriptor-driven engines (API + HTML) in catalog order.
pub fn all_descriptor_engines() -> Vec<EngineDescriptor> {
    let mut engines = api_engines();
    engines.extend(html_engines());
    engines
}
