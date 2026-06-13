//! Code-host engine parsers (GitHub, Hacker News, GitLab, Codeberg, Gitee,
//! Bitbucket, GitFlic) shared by the API catalog. Declared as a submodule of
//! [`super`] so it shares the descriptor types and parsing helpers; the
//! parser/url/header fns are `pub(super)` so [`super::api::api_engines`] can
//! reference them when building descriptors.

use super::*;

pub(super) fn github_url(query: &str, options: &SearchOptions) -> String {
    let per_page = limit_of(options, 50);
    format!(
        "https://api.github.com/search/repositories?per_page={per_page}&q={}",
        urlencoding::encode(query)
    )
}

pub(super) fn github_headers(_options: &SearchOptions) -> Vec<(String, String)> {
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

pub(super) fn github_parse(
    body: &str,
    limit: usize,
    _options: &SearchOptions,
) -> Vec<SearchResult> {
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

pub(super) fn hackernews_url(query: &str, options: &SearchOptions) -> String {
    let hits = limit_of(options, 50);
    format!(
        "https://hn.algolia.com/api/v1/search?hitsPerPage={hits}&query={}",
        urlencoding::encode(query)
    )
}

pub(super) fn hackernews_parse(
    body: &str,
    limit: usize,
    _options: &SearchOptions,
) -> Vec<SearchResult> {
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

pub(super) fn gitlab_url(query: &str, options: &SearchOptions) -> String {
    let per_page = limit_of(options, 50);
    format!(
        "https://gitlab.com/api/v4/projects?search={}&per_page={per_page}&order_by=star_count&sort=desc",
        urlencoding::encode(query)
    )
}

pub(super) fn gitlab_parse(
    body: &str,
    limit: usize,
    _options: &SearchOptions,
) -> Vec<SearchResult> {
    repo_results(
        "gitlab",
        &json(body),
        limit,
        None,
        "path_with_namespace",
        "web_url",
    )
}

pub(super) fn codeberg_url(query: &str, options: &SearchOptions) -> String {
    let limit = limit_of(options, 50);
    format!(
        "https://codeberg.org/api/v1/repos/search?q={}&limit={limit}",
        urlencoding::encode(query)
    )
}

pub(super) fn codeberg_parse(
    body: &str,
    limit: usize,
    _options: &SearchOptions,
) -> Vec<SearchResult> {
    repo_results(
        "codeberg",
        &json(body),
        limit,
        Some("data"),
        "full_name",
        "html_url",
    )
}

pub(super) fn gitee_url(query: &str, options: &SearchOptions) -> String {
    let per_page = limit_of(options, 50);
    format!(
        "https://gitee.com/api/v5/search/repositories?q={}&per_page={per_page}",
        urlencoding::encode(query)
    )
}

pub(super) fn gitee_parse(body: &str, limit: usize, _options: &SearchOptions) -> Vec<SearchResult> {
    repo_results("gitee", &json(body), limit, None, "full_name", "html_url")
}

pub(super) fn bitbucket_url(query: &str, options: &SearchOptions) -> String {
    let pagelen = limit_of(options, 50);
    let query_expr = format!("name~\"{query}\"");
    let q = urlencoding::encode(&query_expr);
    format!(
        "https://api.bitbucket.org/2.0/repositories?q={q}&pagelen={pagelen}&fields=values.full_name,values.links.html.href,values.description"
    )
}

pub(super) fn bitbucket_parse(
    body: &str,
    limit: usize,
    _options: &SearchOptions,
) -> Vec<SearchResult> {
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

pub(super) fn gitflic_url(query: &str, options: &SearchOptions) -> String {
    let size = limit_of(options, 50);
    format!(
        "https://api.gitflic.ru/project?query={}&size={size}",
        urlencoding::encode(query)
    )
}

pub(super) fn gitflic_parse(
    body: &str,
    limit: usize,
    _options: &SearchOptions,
) -> Vec<SearchResult> {
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
