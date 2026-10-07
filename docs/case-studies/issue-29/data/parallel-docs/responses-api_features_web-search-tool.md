> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Web Search Tool

> Restrict the domains the Responses API searches, and read the searches and page reads behind each answer

Web research is always on. You do not need to pass a `web_search` tool to get a grounded
answer. The tool still matters in two ways. Its `filters` restrict which domains are searched
and cited, and every response reports the searches the model ran and the pages it read as
`web_search_call` output items, in the same shape OpenAI returns them.

## Domain filters

Pass the OpenAI `web_search` tool with `filters` to gate research to specific domains. The
filters map onto Parallel's [Source Policy](/resources/source-policy):

| OpenAI `filters` | Parallel `source_policy` | Effect |
| - | - | - |
| `allowed_domains` | `include_domains` | Only sources on these domains (and their subdomains) are searched, read, and cited. |
| `blocked_domains` | `exclude_domains` | Sources on these domains (and their subdomains) are excluded. |

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "What was Nvidia'"'"'s revenue in its most recent quarter?",
      "tools": [
        {
          "type": "web_search",
          "filters": {"allowed_domains": ["sec.gov"]}
        }
      ]
    }'
  ```

  ```python Python theme={"system"}
  import os
  from openai import OpenAI

  client = OpenAI(
      api_key=os.environ["PARALLEL_API_KEY"],
      base_url="https://api.parallel.ai/v1",
  )

  response = client.responses.create(
      model="parallel",
      input="What was Nvidia's revenue in its most recent quarter?",
      tools=[{"type": "web_search", "filters": {"allowed_domains": ["sec.gov"]}}],
  )

  print(response.output_text)
  ```

  ```typescript TypeScript theme={"system"}
  import OpenAI from "openai";

  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
  });

  const response = await client.responses.create({
    model: "parallel",
    input: "What was Nvidia's revenue in its most recent quarter?",
    tools: [{ type: "web_search", filters: { allowed_domains: ["sec.gov"] } }],
  });

  console.log(response.output_text);
  ```
</CodeGroup>

Entries follow the [Source Policy](/resources/source-policy) rules. An apex domain such as
`sec.gov` matches all of its subdomains, `example.com/blog` scopes the filter to a path
prefix, and a bare extension such as `.gov` matches every domain with that extension. Schemes,
ports, query strings, and fragments are not allowed, and a request may carry at most 200
entries across both lists. Invalid entries return a `400` with the same error message the Task
API returns for the same `source_policy`.

<Note>
  Set one list per request. When both `allowed_domains` and `blocked_domains` are present,
  only the allow list applies. If any entry appears in both lists, including a subdomain of
  an allowed domain, the request is rejected with a `400`. A request may contain at most one
  `web_search` tool. `search_context_size` and `user_location` are accepted and ignored.
</Note>

The response echoes your `web_search` tool on `tools`, along with any
[MCP tools](/responses-api/features/mcp-tools). Every other tool type is accepted and
ignored, and is not echoed.

## Search trail

A completed response contains one `web_search_call` item per web action the model took,
ahead of the `message` item. `action.type` says which kind:

| `action.type` | Meaning |
| - | - |
| `search` | A web search. `action.queries` lists the queries issued and `action.query` repeats the first of them, as on OpenAI. |
| `open_page` | A page the model read. `action.url` is the page. Reading the same page twice produces two items. |

```json theme={"system"}
{
  "id": "ws_resp_a0fa0e42-fa1b-4bf0-ae62-baca311fd5b6_0",
  "type": "web_search_call",
  "status": "completed",
  "action": {
    "type": "search",
    "query": "Anthropic announcement September 2026",
    "queries": [
      "Anthropic announcement September 2026",
      "Anthropic news this week September 2026"
    ],
    "sources": null
  }
}
```

```json theme={"system"}
{
  "id": "ws_resp_a0fa0e42-fa1b-4bf0-ae62-baca311fd5b6_1",
  "type": "web_search_call",
  "status": "completed",
  "action": {
    "type": "open_page",
    "url": "https://www.anthropic.com/news"
  }
}
```

Items appear in the order the model started them. Every item closes `completed` when
research finishes; a page that could not be fetched is still reported as read.

Read `output` by item `type`, or use the `output_text` accessor. Do not assume the first
item is the message.

<CodeGroup>
  ```python Python theme={"system"}
  for item in response.output:
      if item.type == "web_search_call":
          if item.action.type == "search":
              print("searched:", item.action.queries)
          elif item.action.type == "open_page":
              print("read:", item.action.url)
      elif item.type == "message":
          print(response.output_text)
  ```

  ```typescript TypeScript theme={"system"}
  for (const item of response.output) {
    if (item.type === "web_search_call") {
      if (item.action.type === "search") {
        console.log("searched:", item.action.queries);
      } else if (item.action.type === "open_page") {
        console.log("read:", item.action.url);
      }
    } else if (item.type === "message") {
      console.log(response.output_text);
    }
  }
  ```
</CodeGroup>

A run that answers without searching or reading a page returns no `web_search_call` items.
Pages your domain filters exclude are never read and do not appear. A search whose results
are all excluded by your filters still appears as a `search` item; the answer then states
what it could not confirm instead of citing an excluded source. `action.sources` is always
`null`, and `include: ["web_search_call.action.sources"]` is accepted and ignored.

These searches and page reads are what surface the sources returned as `url_citation`
annotations on the answer. See [Citations](/responses-api/features/citations) for how to
read them.

With [streaming](/responses-api/features/streaming-events) enabled, each search and page
read is announced as it starts, so `web_search_call` events are the first sign of progress
on longer requests.
The items close together once research finishes, after the answer's text delta.
