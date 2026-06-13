//! API-based engine descriptors and their parsers (Rust port of
//! `src/providers/api-engines.js`). Declared as a submodule of
//! [`super`] so it shares the descriptor types and parsing helpers; the
//! code-host parsers it references live in [`super::code`].

use super::code::*;
use super::*;

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
