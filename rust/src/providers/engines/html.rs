//! HTML-scraping engine descriptors and their parsers (Rust port of
//! `src/providers/html-engines.js`). Declared as a submodule of
//! [`super`] so it shares the descriptor types and parsing helpers.

use super::*;

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
