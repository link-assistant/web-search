> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrate to Parallel Search

> Move from Exa, Tavily, SERP APIs, or built-in model search to the Parallel Search API

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

<Tip>
  Comparing providers? See [How to Eval](/search/evaluating-search) for how to run a fair, end-to-end benchmark before you switch.
</Tip>

This guide maps the request you make today to its Parallel Search equivalent. Parallel Search takes keyword `search_queries`, plus an optional natural language `objective`, and returns ranked, LLM-optimized excerpts in a single call. `search_queries` is the only required field, so most migrations are a one-field change. Picking a [mode](/search/modes) is optional; requests default to `advanced`.

| Coming from | Closest Parallel equivalent |
| - | - |
| Exa `instant` | Search with `mode: "turbo"` |
| Exa `fast` | Search with `mode: "fast"` |
| Exa `auto` | Search with `mode: "advanced"` |
| Exa `deep-lite` or `deep` | [Task API](/task-api/task-quickstart) or [Responses API](/responses-api/responses-quickstart) (synthesized output) |
| Exa `deep-reasoning` | [Task API](/task-api/task-quickstart) |
| Tavily `ultra-fast` | Search with `mode: "turbo"` |
| Tavily `fast` | Search with `mode: "fast"` |
| Tavily `basic` | Search with `mode: "basic"` |
| Tavily `advanced` | Search with `mode: "advanced"` |
| SERP API | Search with `mode: "turbo"` or `"basic"` (one call, ranked excerpts) |
| Built-in model search (e.g. OpenAI `web_search`) | Search as a function tool ([OpenAI](/integrations/openai-tool-calling), [Anthropic](/integrations/anthropic-tool-calling)) with `mode: "fast"` for small models and `mode: "advanced"` for large models |

## Start with one field

Exa, Tavily, and SERP APIs all center on a single query string. The fastest migration is to pass that string as one entry in `search_queries`. That alone is a complete, valid request, since `mode` is optional and defaults to `advanced`:

```json theme={"system"}
{ "search_queries": ["latest developments in solid state batteries"] }
```

For the lowest latency and cost, add `"mode": "turbo"`. For better results, also add an optional `objective`, a natural language description of your full intent, and split the query into 2-3 short keyword queries:

```json theme={"system"}
{
  "objective": "Latest developments in solid state batteries. Focus on manufacturing progress and commercialization timelines.",
  "search_queries": ["solid state battery progress", "solid state battery commercialization"],
  "mode": "turbo"
}
```

See [Best Practices](/search/best-practices) for how to write objectives and queries.

## From Exa

| Exa parameter | Parallel equivalent |
| - | - |
| `query` | `search_queries` (the only required field); optionally add an `objective` describing your full intent |
| `type: "instant"` | `mode: "turbo"` |
| `type: "fast"` | `mode: "fast"` |
| `type: "auto"` | `mode: "advanced"` |
| `type: "deep-lite"` or `"deep"` | Use the [Task API](/task-api/task-quickstart) or [Responses API](/responses-api/responses-quickstart) (both return synthesized output) |
| `type: "deep-reasoning"` | Use the [Task API](/task-api/task-quickstart) |
| `numResults` | `advanced_settings.max_results` |
| `additionalQueries` | Add them as extra entries in `search_queries`; Parallel searches multiple queries in one call |
| `includeDomains` / `excludeDomains` | `advanced_settings.source_policy.include_domains` / `exclude_domains` |
| `startPublishedDate` | `advanced_settings.source_policy.after_date` (YYYY-MM-DD) |
| `userLocation` | `advanced_settings.location` (ISO 3166-1 alpha-2 country code) |
| `contents.text` (full page text) | Search returns compressed excerpts, not full page bodies. For full page contents, use the [Extract API](/extract/extract-quickstart) |
| `category: "company"` or `"people"` | Consider [Entity Search](/findall-api/entity-search) |
| `outputSchema` / `summary` | Structured, synthesized outputs are the [Task API](/task-api/task-quickstart) or [Responses API](/responses-api/responses-quickstart) |

Before and after:

<CodeGroup>
  ```bash Exa theme={"system"}
  curl https://api.exa.ai/search \
    -H "x-api-key: $EXA_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "query": "latest developments in solid state batteries",
      "type": "fast",
      "numResults": 10,
      "contents": { "text": { "maxCharacters": 1500 } }
    }'
  ```

  ```bash Parallel theme={"system"}
  curl https://api.parallel.ai/v1/search \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "search_queries": ["latest developments in solid state batteries"],
      "mode": "fast"
    }'
  ```
</CodeGroup>

Excerpts are on by default and sized dynamically, so most requests need no excerpt configuration at all. Add `advanced_settings` knobs only when you have a specific product constraint. Restrictive settings can reduce result quality.

## From Tavily

Tavily's `search_depth` tiers map one-to-one onto Parallel modes.

| Tavily parameter | Parallel equivalent |
| - | - |
| `query` | `search_queries` (the only required field); optionally add an `objective` describing your full intent |
| `search_depth: "ultra-fast"` | `mode: "turbo"` |
| `search_depth: "fast"` | `mode: "fast"` |
| `search_depth: "basic"` | `mode: "basic"` |
| `search_depth: "advanced"` | `mode: "advanced"` |
| `max_results` | `advanced_settings.max_results` |
| `include_domains` / `exclude_domains` | `advanced_settings.source_policy.include_domains` / `exclude_domains` |
| `start_date` | `advanced_settings.source_policy.after_date` (YYYY-MM-DD). Tavily `time_range` (relative, e.g. `week`) has no direct equivalent; convert to an absolute date first |
| `country` | `advanced_settings.location` (ISO 3166-1 alpha-2; convert Tavily's country name to a code) |
| `topic: "news"` or `"finance"` | State the focus in your `objective` (for example "recent news about..." or "latest financial results for...") |
| `include_answer` | Search returns excerpts for your model to reason over. For grounded completions, use the [Responses API](/responses-api/responses-quickstart) |
| `include_raw_content` | Full page contents are the [Extract API](/extract/extract-quickstart) |

Before and after:

<CodeGroup>
  ```bash Tavily theme={"system"}
  curl https://api.tavily.com/search \
    -H "Authorization: Bearer $TAVILY_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "query": "latest developments in solid state batteries",
      "search_depth": "fast",
      "max_results": 10
    }'
  ```

  ```bash Parallel theme={"system"}
  curl https://api.parallel.ai/v1/search \
    -H "x-api-key: $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "search_queries": ["latest developments in solid state batteries"],
      "mode": "fast"
    }'
  ```
</CodeGroup>

Parallel authenticates with an `x-api-key` header rather than `Authorization: Bearer`, so a request that keeps the Tavily-style Bearer header returns 401.

## From a SERP API

A SERP API returns a ranked list of links and snippets, but you must make subsequent fetches to extract usable information.

Parallel Search combines everything into a single call. Instead of links you fetch and process, you get ranked, compressed excerpts sized for a model, so there is no fetch round and no chunking or ranking to build yourself. If a workflow still needs a specific page's full contents, use the [Extract API](/extract/extract-quickstart).

<Tabs>
  <Tab title="Python">
    You search, then fetch and process each result in your own code:

    ```python theme={"system"}
    import os, requests
    from bs4 import BeautifulSoup
    # split_into_chunks() and rank_by_relevance() are helpers you write and maintain

    query = "solid state battery developments"

    # Step 1: search returns links + snippets, no page content
    hits = requests.get("https://serpapi.com/search", params={
        "q": query, "api_key": os.environ["SERPAPI_KEY"],
    }).json()["organic_results"][:10]

    # Step 2: fetch every result URL yourself, one at a time
    pages = []
    for h in hits:
        try:
            html = requests.get(h["link"], timeout=10).text
        except requests.RequestException:
            continue  # dead links, timeouts, bot blocks: your problem to handle
        text = BeautifulSoup(html, "html.parser").get_text(" ", strip=True)
        # (a scraper that returns markdown lets you skip this HTML parsing)
        pages.append(text)

    # Step 3: chunk and rank locally so it fits the model context
    chunks = [c for p in pages for c in split_into_chunks(p)]
    context = rank_by_relevance(chunks, query)[:15]
    ```

    Parallel does it in one call:

    ```python theme={"system"}
    import os
    from parallel import Parallel

    client = Parallel(api_key=os.environ["PARALLEL_API_KEY"])

    # one call returns ranked, model-ready excerpts
    search = client.search(search_queries=["solid state battery developments"], mode="turbo")
    context = [ex for r in search.results for ex in r.excerpts]
    ```
  </Tab>

  <Tab title="cURL">
    You search, then fetch each result:

    ```bash theme={"system"}
    # Step 1: search returns links + snippets, no page content
    curl "https://serpapi.com/search?q=solid+state+battery+developments&api_key=$SERPAPI_KEY"

    # Step 2: fetch each result URL for its content (repeat for every link),
    # then strip HTML, chunk, and rank in your own code
    curl -L "https://<first-result-url>"
    curl -L "https://<second-result-url>"
    ```

    Parallel does it in one call:

    ```bash theme={"system"}
    curl https://api.parallel.ai/v1/search \
      -H "x-api-key: $PARALLEL_API_KEY" \
      -H "Content-Type: application/json" \
      -d '{ "search_queries": ["solid state battery developments"], "mode": "turbo" }'
    ```
  </Tab>
</Tabs>

## From built-in model search

"Built-in model search" splits into two cases that migrate differently.

**Building on the API.** If you call a provider's hosted search tool in your own app (for example OpenAI's `web_search` in the Responses API), the provider runs the search inside its own walls. Migrate by registering Parallel Search as a function tool your app executes, which gives you mode control, the excerpts, and the provider choice. Match the mode to the model driving the tool: use `fast` (\~700ms) for small models, and `advanced` for large models that can spend more latency for higher-quality results.

```python theme={"system"}
def search_web(search_queries: list[str], objective: str | None = None) -> dict:
    return parallel_client.search(
        search_queries=search_queries,
        objective=objective,
        # Small model driving the tool → "fast". Large model → "advanced".
        mode="fast",
    )
```

The model decides when to search. Your handler decides how. Full walkthroughs with the tool schema and the tool-result loop live on the [OpenAI](/integrations/openai-tool-calling), [Anthropic](/integrations/anthropic-tool-calling), and [Ollama](/integrations/ollama-tool-calling) pages.

**Inside the ChatGPT app (no code).** If you rely on web search in ChatGPT itself rather than building on the API, add Parallel as a connector through the [Search MCP](/integrations/mcp/search-mcp).

## What you parse now

Parallel returns a `results` array. Each item has a `url`, `title`, `publish_date`, and `excerpts` (the compressed, relevant snippets your model reads), with a top-level `warnings` field worth checking on early calls:

```json theme={"system"}
{
  "results": [
    { "url": "https://...", "title": "...", "publish_date": null, "excerpts": ["..."] }
  ],
  "warnings": null
}
```

If your current integration expects a field Search does not return, here is where that capability lives:

| If your integration expects | With Parallel Search | Next step |
| - | - | - |
| a relevance `score` to sort on | results come back already ranked | preserve the returned order; there is no per-result score |
| full page text (Exa `text`, Tavily `raw_content`) | you get compressed excerpts, not full page bodies | use the [Extract API](/extract/extract-quickstart) for full contents |
| a synthesized answer or report (Tavily `include_answer`, Exa `summary` / `deep`) | Search returns excerpts for your model to reason over | use the [Responses API](/responses-api/responses-quickstart) for grounded answers, or the [Task API](/task-api/task-quickstart) for multi-step research |
| a list of people or companies (Exa `category`) | Search retrieves pages and excerpts across the web | use [Entity Search](/findall-api/entity-search) |
| `images` | not returned | — |

## Verify your migration

Before moving production traffic, run your top queries in the [Playground](https://platform.parallel.ai/play/search) and start with the mapped mode. On your first calls, check the response `warnings` field for any parameters that were adjusted or ignored, and confirm [Pricing](/getting-started/pricing) and [Rate Limits](/getting-started/rate-limits) for your volume.
