//! Provider discovery as data, shared by server and no_std + alloc consumers.
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use serde::{Deserialize, Serialize};

/// Canonical provider categories.
pub const CATEGORIES: [&str; 4] = ["search", "knowledge", "papers", "code"];

/// Method used by the default endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RequestMethod {
    Get,
    Post,
}
impl RequestMethod {
    /// HTTP method text.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
        }
    }
}

/// Capabilities for selecting providers without instantiating network code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapabilities {
    /// Default request method.
    pub method: RequestMethod,
    /// Has an API-backed path.
    pub api: bool,
    /// Has an HTML scraping path.
    pub html: bool,
    /// Accepts optional credentials (HTML fallback may still work without them).
    pub optional_credentials: bool,
    /// Delegates retrieval to web-capture.
    pub component: bool,
}

/// Borrowed static metadata; inspecting the catalog requires no allocation.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderMetadata {
    /// Stable provider name/id.
    pub id: &'static str,
    /// Display name.
    pub label: &'static str,
    /// One of CATEGORIES.
    pub category: &'static str,
    /// Browser CORS support.
    pub cors_readable: bool,
    /// Category's default provider.
    pub default_for_category: bool,
    /// Retrieval mode.
    pub access: &'static str,
    /// Default endpoint; tokens contain percent-encoded query and language/limit.
    /// Hybrid entries describe their unauthenticated HTML fallback. Component
    /// entries describe the upstream endpoint and still require web-capture.
    pub endpoint_template: &'static str,
    /// Form body template for POST endpoints, if any.
    pub body_template: Option<&'static str>,
    /// Selection capabilities.
    pub capabilities: ProviderCapabilities,
}

/// Owned, serializable discovery metadata (compatible with server discovery).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryEntry {
    /// Stable provider id.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Provider category.
    pub category: String,
    /// Browser CORS support.
    pub cors_readable: bool,
    /// Category default flag.
    pub default_for_category: bool,
    /// Retrieval mode.
    pub access: String,
    /// Default endpoint template.
    pub endpoint_template: String,
    /// Optional POST body template.
    pub body_template: Option<String>,
    /// Selection capabilities.
    pub capabilities: ProviderCapabilities,
}

macro_rules! provider {
    (@api "api") => { true };
    (@api "hybrid") => { true };
    (@api $other:tt) => { false };
    (@html "html") => { true };
    (@html "hybrid") => { true };
    (@html $other:tt) => { false };
    (@component "component") => { true };
    (@component $other:tt) => { false };
    ($id:literal, $label:literal, $category:literal, $cors:literal, $default:literal,
     $access:tt, $endpoint:literal, $method:ident, $body:expr, $credentials:literal) => {
        ProviderMetadata {
            id: $id,
            label: $label,
            category: $category,
            cors_readable: $cors,
            default_for_category: $default,
            access: $access,
            endpoint_template: $endpoint,
            body_template: $body,
            capabilities: ProviderCapabilities {
                method: RequestMethod::$method,
                api: provider!(@api $access),
                html: provider!(@html $access),
                component: provider!(@component $access),
                optional_credentials: $credentials,
            },
        }
    };
}

/// All 40 providers in canonical catalog order.
pub static PROVIDER_REGISTRY: &[ProviderMetadata] = &[
    provider!("google", "Google", "search", false, false, "hybrid", "https://www.google.com/search?q={query}", Get, None, true),
    provider!("bing", "Bing", "search", false, false, "hybrid", "https://www.bing.com/search?q={query}", Get, None, true),
    provider!("duckduckgo", "DuckDuckGo", "search", false, true, "html", "https://html.duckduckgo.com/html/", Post, Some("q={query}"), false),
    provider!("wikipedia", "Wikipedia", "knowledge", true, true, "api", "https://{language}.wikipedia.org/w/rest.php/v1/search/page?q={query}&limit={limit}", Get, None, false),
    provider!("wikidata", "Wikidata", "knowledge", true, false, "api", "https://www.wikidata.org/w/api.php?action=wbsearchentities&format=json&language={language}&uselang={language}&limit={limit}&search={query}", Get, None, false),
    provider!("wiktionary", "Wiktionary", "knowledge", true, false, "api", "https://{language}.wiktionary.org/w/rest.php/v1/search/page?q={query}&limit={limit}", Get, None, false),
    provider!("wikinews", "Wikinews", "knowledge", true, false, "api", "https://{language}.wikinews.org/w/rest.php/v1/search/page?q={query}&limit={limit}", Get, None, false),
    provider!("internet-archive", "Internet Archive", "knowledge", true, false, "api", "https://archive.org/advancedsearch.php?q={query}&fl[]=identifier&fl[]=title&fl[]=description&rows={limit}&page=1&output=json", Get, None, false),
    provider!("dbpedia", "DBpedia", "knowledge", false, false, "api", "https://lookup.dbpedia.org/api/search?format=json&maxResults={limit}&query={query}", Get, None, false),
    provider!("openlibrary", "Open Library", "knowledge", true, false, "api", "https://openlibrary.org/search.json?q={query}&limit={limit}", Get, None, false),
    provider!("semantic-scholar", "Semantic Scholar", "knowledge", true, false, "api", "https://api.semanticscholar.org/graph/v1/paper/search?query={query}&limit={limit}&fields=title,abstract,url,year", Get, None, false),
    provider!("openalex", "OpenAlex", "knowledge", true, false, "api", "https://api.openalex.org/works?per-page={limit}&search={query}", Get, None, false),
    provider!("crossref", "Crossref", "knowledge", true, false, "api", "https://api.crossref.org/works?rows={limit}&query={query}", Get, None, false),
    provider!("searx", "SearXNG", "search", false, false, "api", "https://searx.be/search?format=json&q={query}", Get, None, false),
    provider!("arxiv", "arXiv", "papers", true, true, "api", "http://export.arxiv.org/api/query?max_results={limit}&search_query=all%3A{query}", Get, None, false),
    provider!("europepmc", "Europe PMC", "papers", true, false, "api", "https://www.ebi.ac.uk/europepmc/webservices/rest/search?query={query}&format=json&pageSize={limit}", Get, None, false),
    provider!("doaj", "DOAJ", "papers", true, false, "api", "https://doaj.org/api/search/articles/{query}?pageSize={limit}", Get, None, false),
    provider!("github", "GitHub", "code", true, true, "api", "https://api.github.com/search/repositories?per_page={limit}&q={query}", Get, None, true),
    provider!("hackernews", "Hacker News", "code", true, false, "api", "https://hn.algolia.com/api/v1/search?hitsPerPage={limit}&query={query}", Get, None, false),
    provider!("gitlab", "GitLab", "code", true, false, "api", "https://gitlab.com/api/v4/projects?search={query}&per_page={limit}&order_by=star_count&sort=desc", Get, None, false),
    provider!("codeberg", "Codeberg", "code", true, false, "api", "https://codeberg.org/api/v1/repos/search?q={query}&limit={limit}", Get, None, false),
    provider!("gitee", "Gitee", "code", true, false, "api", "https://gitee.com/api/v5/search/repositories?q={query}&per_page={limit}", Get, None, false),
    provider!("bitbucket", "Bitbucket", "code", true, false, "api", "https://api.bitbucket.org/2.0/repositories?q=name~%22{query}%22&pagelen={limit}&fields=values.full_name,values.links.html.href,values.description", Get, None, false),
    provider!("gitflic", "GitFlic", "code", false, false, "api", "https://api.gitflic.ru/project?query={query}&size={limit}", Get, None, false),
    provider!("brave", "Brave Search", "search", false, false, "html", "https://search.brave.com/search?q={query}", Get, None, false),
    provider!("mojeek", "Mojeek", "search", false, false, "html", "https://www.mojeek.com/search?q={query}", Get, None, false),
    provider!("ecosia", "Ecosia", "search", false, false, "html", "https://www.ecosia.org/search?q={query}", Get, None, false),
    provider!("startpage", "Startpage", "search", false, false, "html", "https://www.startpage.com/sp/search?query={query}", Get, None, false),
    provider!("yahoo", "Yahoo Search", "search", false, false, "html", "https://search.yahoo.com/search?p={query}", Get, None, false),
    provider!("yandex", "Yandex", "search", false, false, "html", "https://yandex.com/search/?text={query}", Get, None, false),
    provider!("cambridge-dictionary", "Cambridge Dictionary", "knowledge", false, false, "html", "https://dictionary.cambridge.org/dictionary/english/{query}", Get, None, false),
    provider!("merriam-webster", "Merriam-Webster", "knowledge", false, false, "html", "https://www.merriam-webster.com/dictionary/{query}", Get, None, false),
    provider!("dictionary-com", "Dictionary.com", "knowledge", false, false, "html", "https://www.dictionary.com/browse/{query}", Get, None, false),
    provider!("collins-dictionary", "Collins Dictionary", "knowledge", false, false, "html", "https://www.collinsdictionary.com/dictionary/english/{query}", Get, None, false),
    provider!("lite", "DuckDuckGo Lite", "search", false, false, "html", "https://lite.duckduckgo.com/lite/", Post, Some("q={query}"), false),
    provider!("wc:wikipedia", "web-capture (wikipedia)", "search", true, false, "component", "https://{language}.wikipedia.org/w/rest.php/v1/search/page?q={query}&limit={limit}", Get, None, false),
    provider!("wc:duckduckgo", "web-capture (duckduckgo)", "search", false, false, "component", "https://html.duckduckgo.com/html/", Post, Some("q={query}"), false),
    provider!("wc:google", "web-capture (google)", "search", false, false, "component", "https://www.google.com/search?q={query}", Get, None, false),
    provider!("wc:bing", "web-capture (bing)", "search", false, false, "component", "https://www.bing.com/search?q={query}", Get, None, false),
    provider!("wc:brave", "web-capture (brave)", "search", false, false, "component", "https://search.brave.com/search?q={query}", Get, None, false),
];

/// Find provider metadata by stable id.
pub fn provider_metadata(id: &str) -> Option<&'static ProviderMetadata> {
    PROVIDER_REGISTRY.iter().find(|entry| entry.id == id)
}

/// Build owned registry entries for serialization or existing server consumers.
pub fn get_registry() -> Vec<RegistryEntry> {
    PROVIDER_REGISTRY
        .iter()
        .map(|e| RegistryEntry {
            id: e.id.to_string(),
            label: e.label.to_string(),
            category: e.category.to_string(),
            cors_readable: e.cors_readable,
            default_for_category: e.default_for_category,
            access: e.access.to_string(),
            endpoint_template: e.endpoint_template.to_string(),
            body_template: e.body_template.map(ToString::to_string),
            capabilities: e.capabilities,
        })
        .collect()
}

/// List provider ids, optionally filtered by category.
pub fn get_provider_ids(category: Option<&str>) -> Vec<String> {
    PROVIDER_REGISTRY
        .iter()
        .filter(|e| category.is_none_or(|c| e.category == c))
        .map(|e| e.id.to_string())
        .collect()
}

/// The shared DuckDuckGo-first, CORS-readable knowledge sweep.
pub fn get_default_provider_ids() -> Vec<String> {
    [
        "duckduckgo",
        "internet-archive",
        "wikipedia",
        "wikidata",
        "wiktionary",
        "wikinews",
    ]
    .iter()
    .map(|id| id.to_string())
    .collect()
}

/// Whether a category is recognized.
pub fn is_known_category(category: &str) -> bool {
    CATEGORIES.contains(&category)
}
