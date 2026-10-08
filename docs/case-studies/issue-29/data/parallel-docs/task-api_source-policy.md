> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Source Policy

> Control which sources are used in web research with domain and path-prefix filters and a freshness start date.

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

The Source Policy feature allows you to precisely control which domains or sections of a site
Parallel can access during web research and to apply a freshness constraint. It's available for
the Task, Search, Monitor, and Responses APIs and lets you tailor search results by specifying domains or
domain/path prefixes to include or exclude and by setting a start date so results are limited to
recent content.

<Warning>
  Source policies can significantly reduce result quality by excluding relevant pages from retrieval. Use `include_domains` and `exclude_domains` only when absolutely necessary — for compliance-bound corpora, tasks that require a single known publisher, or when specific sources must be blocked. When you only want to *prefer* certain sources, steer in the `objective` instead (e.g., "prefer official documentation").
</Warning>

## Configuration

You can configure source control by setting the following parameters:

| Parameter | Type | Supported | Description |
| - | - | - | - |
| `include_domains` | array\[string] | Task, Search, Monitor, and Responses APIs | Domains or domain/path prefixes to **allow**. Only matching sources are returned. |
| `exclude_domains` | array\[string] | Task, Search, Monitor, and Responses APIs | Domains or domain/path prefixes to **block**. Applied only when `include_domains` is empty. |
| `after_date` | string\<date> | Search API | Earliest publication date to return, formatted as RFC 3339 (`YYYY-MM-DD`). |

On the Responses API, the source policy is set through the OpenAI `web_search` tool:
`filters.allowed_domains` maps to `include_domains` and `filters.blocked_domains` to
`exclude_domains`. See [Web Search Tool](/responses-api/features/web-search-tool).

Specifying an apex domain such as `example.com` will automatically include all its
subdomains (e.g., `www.example.com`, `blog.example.com`, `api.example.com`).

A domain/path prefix such as `example.com/blog` matches that path and its descendants on the
domain and its subdomains. Paths are case-sensitive and match at segment boundaries, so
`example.com/blog` matches `/blog` and `/blog/post`, but not `/blogroll`. Trailing slashes are
ignored and dot segments are normalized.

<Warning>
  Domain/path prefixes are not supported in Search API `turbo` mode. Use `fast`, `basic`, or
  `advanced` when filtering by path. Plain domain and domain-extension filters remain supported in
  `turbo` mode.
</Warning>

### Source filter limit

<Warning>
  **Hard limit: Combined total of 200 entries per request.** You can specify up to 200 domains,
  domain/path prefixes, and domain extensions in total across `include_domains` and
  `exclude_domains`. Exceeding this limit raises a validation error.
</Warning>

## Example

<CodeGroup>
  ```bash Task API theme={"system"}
  curl -X POST "https://api.parallel.ai/v1/tasks/runs" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
        "input": "How many employees does Parallel Web Systems have?",
        "processor": "core",
        "source_policy": {
          "include_domains": ["linkedin.com"]
        }
      }'
  ```

  ```bash Search API theme={"system"}
  curl https://api.parallel.ai/v1/search \
    -H "Content-Type: application/json" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -d '{
      "objective": "Which open source LLMs were released recently and how do they benchmark?",
      "search_queries": ["open source LLMs"],
      "mode": "fast",
      "advanced_settings": {
        "source_policy": {
          "exclude_domains": ["reddit.com/r/LocalLLaMA"],
          "after_date": "2026-01-01"
        }
      }
    }'
  ```

  ```bash Responses API theme={"system"}
  curl https://api.parallel.ai/v1/responses \
    -H "Content-Type: application/json" \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -d '{
      "model": "parallel",
      "input": "How many employees does Parallel Web Systems have?",
      "tools": [
        {
          "type": "web_search",
          "filters": {"allowed_domains": ["linkedin.com"]}
        }
      ]
    }'
  ```
</CodeGroup>

## Best Practices

* Use either `include_domains` or `exclude_domains` in a single query. Specifying `exclude_domains` is redundant when `include_domains` is set, as only `include_domains` will be applied.
* Use an apex domain such as `example.com` to match that domain and all its subdomains, or specify a subdomain such as `docs.example.com` to match that subdomain and any deeper subdomains. A leading `www.` is normalized away.
* Add a path to scope the filter to one section of a site, such as `docs.example.com/api`. Path matching is case-sensitive and occurs at segment boundaries.
* Use a bare domain extension beginning with a period, such as `.org`, `.gov`, or `.co.uk`, to match that extension. Literal wildcard syntax such as `*.org` is not supported.
* Do not include schemes (`http://`, `https://`), ports (`example.com:443`), query strings, or fragments.
* Although there is a maximum of 200 entries, carefully using specific and targeted filters will give better results.
