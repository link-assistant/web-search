//! Shared HTML text utilities and a generic anchor-list parser.
//!
//! Mirrors the JavaScript `src/providers/html-utils.js` and the
//! `parseAnchorList` helper from `src/providers/html-engines.js`, so the
//! descriptor-driven HTML engines behave identically across both language
//! implementations (issue #3 parity requirement).

use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;

use super::base::SearchResult;

/// Decode the small set of HTML entities that appear in SERP markup, including
/// numeric (`&#160;`) and hex (`&#xA0;`) character references.
pub fn decode_html_entities(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let named = [
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&apos;", "'"),
        ("&nbsp;", " "),
        ("&hellip;", "…"),
        ("&mdash;", "—"),
        ("&ndash;", "–"),
    ];

    let mut output = input.to_string();
    for (entity, replacement) in named {
        output = output.replace(entity, replacement);
    }

    static NUMERIC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"&#(x?[0-9a-fA-F]+);").unwrap());
    NUMERIC
        .replace_all(&output, |caps: &regex::Captures| {
            let token = &caps[1];
            let code =
                if let Some(hex) = token.strip_prefix('x').or_else(|| token.strip_prefix('X')) {
                    u32::from_str_radix(hex, 16).ok()
                } else {
                    token.parse::<u32>().ok()
                };
            code.and_then(char::from_u32)
                .map(|c| c.to_string())
                .unwrap_or_else(|| caps[0].to_string())
        })
        .into_owned()
}

/// Strip HTML tags from a fragment, leaving only its text content.
pub fn strip_html(input: &str) -> String {
    static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]*>").unwrap());
    TAG.replace_all(input, "").into_owned()
}

/// Normalize an HTML fragment to clean display text: strip tags, decode
/// entities, and collapse runs of whitespace.
pub fn clean_text(input: &str) -> String {
    static WS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
    let stripped = strip_html(input);
    let decoded = decode_html_entities(&stripped);
    WS.replace_all(&decoded, " ").trim().to_string()
}

/// Configuration for [`parse_anchor_list`], mirroring the JS `parseAnchorList`
/// per-engine descriptor.
pub struct AnchorConfig {
    /// Provider id recorded as the result `source`.
    pub source: &'static str,
    /// Max results to return.
    pub limit: usize,
    /// Compiled item regex; capture groups feed the fields below.
    pub item_regex: &'static Regex,
    /// Capture-group index for the URL.
    pub url_group: usize,
    /// Capture-group index for the title.
    pub title_group: usize,
    /// Optional capture-group index for the snippet.
    pub snippet_group: Option<usize>,
    /// Optional URL normalizer applied before dedup/skip checks.
    pub url_transform: Option<fn(&str) -> String>,
    /// Optional predicate; return true to drop a URL.
    pub skip: Option<fn(&str) -> bool>,
}

/// Generic HTML result-list parser driven by a per-engine regex.
pub fn parse_anchor_list(html: &str, config: &AnchorConfig) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    for caps in config.item_regex.captures_iter(html) {
        if results.len() >= config.limit {
            break;
        }

        let raw_url = match caps.get(config.url_group) {
            Some(m) => m.as_str(),
            None => continue,
        };
        let url = match config.url_transform {
            Some(transform) => transform(raw_url),
            None => raw_url.to_string(),
        };

        if url.is_empty() || seen.contains(&url) {
            continue;
        }
        if let Some(skip) = config.skip {
            if skip(&url) {
                continue;
            }
        }
        seen.insert(url.clone());

        let title = caps
            .get(config.title_group)
            .map(|m| clean_text(m.as_str()))
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| "Untitled".to_string());
        let snippet = config
            .snippet_group
            .and_then(|g| caps.get(g))
            .map(|m| clean_text(m.as_str()))
            .unwrap_or_default();

        results.push(SearchResult {
            title,
            url,
            snippet,
            source: config.source.to_string(),
            rank: results.len() + 1,
            score: None,
            sources: None,
        });
    }

    results
}
