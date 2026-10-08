> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Search indexed web content for coding agents

> Give coding agents documentation excerpts from the Parallel web index without a live source-page fetch

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Coding agents need API documentation and error explanations while working in environments that cannot browse source websites. You can use the Search API to retrieve excerpts from Parallel's existing web index. Search disables live source-page fetching by default when you omit `advanced_settings.fetch_policy`.

## Search indexed documentation

Send a request to `POST /v1/search` without `advanced_settings.fetch_policy`:

```json theme={"system"}
{
  "objective": "Find the official Python documentation for how asyncio TaskGroup handles cancellation.",
  "search_queries": [
    "Python asyncio TaskGroup cancellation docs",
    "TaskGroup cancellation behavior Python documentation"
  ],
  "advanced_settings": {
    "source_policy": {
      "include_domains": ["docs.python.org"]
    }
  }
}
```

Search returns titles, URLs, and excerpts from indexed pages that match the query. The `include_domains` filter limits the results to Python's documentation site. Remove that filter when your agent needs results from more sources. See [Source Policy](/resources/source-policy) for domain and path filters.

The index may lack a page or contain an older copy. Search does not fetch a missing or stale source page in this default mode. If you supply `advanced_settings.fetch_policy` to request fresher content, Search may fetch source pages. See [Advanced Search Settings](/search/advanced-search-settings).

## Keep URL extraction separate

If you give the agent the Extract API, it can submit a URL that did not appear in Search results. Extract uses a separate, dynamic fetch policy by default and may request the source website. Its `max_age_seconds`, `timeout_seconds`, and `disable_cache_fallback` fields control freshness, fetch duration, and fallback behavior. They do not provide a documented security guarantee against all upstream requests. See [Advanced Extract Settings](/extract/advanced-extract-settings#fetch-policy).
