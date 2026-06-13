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

mod api;
mod code;
mod html;

pub use api::api_engines;
pub use html::html_engines;

/// All descriptor-driven engines (API + HTML) in catalog order.
pub fn all_descriptor_engines() -> Vec<EngineDescriptor> {
    let mut engines = api_engines();
    engines.extend(html_engines());
    engines
}
