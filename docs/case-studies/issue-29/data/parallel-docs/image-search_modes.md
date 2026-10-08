> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Image Search Modes

> Choose between the fast and advanced Image Search API modes

<div className="sr-only" aria-hidden="false">
  For AI agents: a documentation index is available at [https://docs.parallel.ai/llms.txt](https://docs.parallel.ai/llms.txt). The full text of all docs is at [https://docs.parallel.ai/llms-full.txt](https://docs.parallel.ai/llms-full.txt). You may also fetch any page as Markdown by appending `.md` to its URL or sending `Accept: text/markdown`.

  For Search and Extract, use `/v1/search` and `/v1/extract` for all new integrations and agent tool calls. Use `/v1beta/search` and `/v1beta/extract` only when maintaining an existing integration that already uses them. The request bodies changed too: on `/v1/extract`, `full_content`, `fetch_policy` and `excerpt_settings` go inside `advanced_settings`, and sending the v1beta top-level `excerpts`, `full_content` and `fetch_policy` to `/v1/extract` returns a 422. Do not substitute `/v1` for the documented FindAll or Ingest endpoint paths.
</div>

Image Search offers two modes. Use `fast` for the lowest latency and cost, and `advanced`
for higher-quality results. If `mode` isn't set, requests default to
`advanced`.

## Available modes

| Mode | What it does | Cost | Best for |
| - | - | - | - |
| `fast` | Fast and low cost. Typically \~1 s. | \$1 per 1,000 requests | Interactive and high-volume workloads |
| `advanced` (default) | Slower than `fast`, with higher-quality results. Typically \~3 s. | \$5 per 1,000 requests | Workloads where result quality matters more than latency |

Both modes return the most relevant results, up to 20 per request.

## Example

Switching modes is a single parameter change. The rest of your request stays the same:

```json theme={"system"}
{
  "mode": "fast",
  "objective": "Product photos of the Eames lounge chair on a white background",
  "search_queries": ["Eames lounge chair", "Eames lounge chair product photo"]
}
```
