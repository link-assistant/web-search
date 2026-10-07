> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Responses API Quickstart

> Answer questions with live web research in seconds

The Parallel Responses API is an OpenAI Responses-compatible endpoint that answers a
question using live web research within a latency budget of \~5–60 seconds. The API automatically handles running multi-step web searches, fetching live pages,
and cross-checking sources to return a **synthesized answer with citations**.

Because it is OpenAI compatible, you can invoke the Responses API with OpenAI's Typescript or Python SDKs.  Just change three parameters: `base_url`, `api_key`, and `model`.

<a id="choosing-the-right-model" />

## Model and reasoning tiers

There is one model id — `parallel` — and `reasoning.effort` selects the reasoning tier:

| `reasoning.effort` | Latency | Designed for | Example |
| - | - | - | - |
| `low` | \~5–10s | Simple fact-retrieval questions | "What's the current U.S. federal funds rate?" |
| `medium` (default) | \~15–20s | More complex, multi-hop fact-retrieval questions | "Which G7 country has the highest corporate tax rate?" |
| `high` | \~30–60s | Deep-research questions requiring extensive search and synthesis | "How have G7 corporate tax rates shifted since 2020, and what's driving the trend?" |

Every tier grounds its answer in live web search, but higher tiers do more web research before producing an answer. If you omit `reasoning`, effort defaults to `medium`. `high` requests can take up to a minute, so give your client timeout headroom (e.g. `timeout=120.0` in the Python SDK).

## Prerequisites

Get an API key from [Parallel Settings](https://platform.parallel.ai) and set it as
`PARALLEL_API_KEY`. If you are using an SDK, install the OpenAI SDK:

<CodeGroup>
  ```bash cURL theme={"system"}
  export PARALLEL_API_KEY="your-api-key"
  ```

  ```bash Python theme={"system"}
  pip install openai
  ```

  ```bash TypeScript theme={"system"}
  npm install openai
  ```
</CodeGroup>

## Make your first request

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "Who is the current CEO of the largest cloud provider by revenue?",
      "reasoning": {"effort": "low"}
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
      input="Who is the current CEO of the largest cloud provider by revenue?",
      reasoning={"effort": "low"},  # low | medium | high
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
    input: "Who is the current CEO of the largest cloud provider by revenue?",
    reasoning: { effort: "low" },
  });

  console.log(response.output_text);
  ```
</CodeGroup>

Read the answer from `output_text` (convenience accessor), or walk `output[]` by item
`type`: one `web_search_call` item per search the model ran or page it read, `mcp_list_tools`
and `mcp_call` items for [MCP tools](/responses-api/features/mcp-tools), and the `message`
item whose `content[]` carries the answer text.

## Migrate from OpenAI

Migrating an existing OpenAI Responses integration is a three-line diff: point `base_url`
at Parallel, swap the key, and set `model="parallel"`. Grounding is automatic, so you can
drop the `web_search` tool. If you keep it, its domain `filters` are honored (see
[Web Search Tool](/responses-api/features/web-search-tool)). `mcp` tools carry over too, as
long as they set `require_approval: "never"` (see
[MCP Tools](/responses-api/features/mcp-tools)):

```diff theme={"system"}
- client = OpenAI(api_key=OPENAI_API_KEY)
+ client = OpenAI(api_key=PARALLEL_API_KEY, base_url="https://api.parallel.ai/v1")

  response = client.responses.create(
-     model="gpt-5.6-luna",
+     model="parallel",
      input=PROMPT,
      reasoning={"effort": "low"},
-     tools=[{"type": "web_search"}],
  )
```

See [OpenAI Responses Compatibility](/responses-api/openai-compatibility) for the full
field-by-field compatibility reference.

## Use it as a research subagent

You can use the Responses API as a web research tool for your own agent by creating a
tool that wraps the Responses API. Each tool call your agent makes is delegated to an
autonomous research subagent that returns a finished, cited answer — not raw search results
your agent has to read through.

```python theme={"system"}
def web_research(query: str, effort: str = "low") -> dict:
    """Tool handler: one complete question in, one researched answer out."""
    r = parallel.responses.create(
        model="parallel", input=query, reasoning={"effort": effort}
    )
    return {"answer": r.output_text}
```

Your agent plans and decomposes; Parallel researches. See
[Using It as a Research Subagent](/responses-api/examples/research-subagent) for the tool
definition, best practices for writing the tool description, and a full orchestrator loop.

## Features

<CardGroup cols={2}>
  <Card title="Statefulness" icon="comments" href="/responses-api/features/statefulness">
    Multi-turn conversations via `previous_response_id`
  </Card>

  <Card title="Structured Outputs" icon="brackets-curly" href="/responses-api/features/structured-outputs">
    JSON answers conforming to your schema
  </Card>

  <Card title="Streaming Events" icon="bolt" href="/responses-api/features/streaming-events">
    Server-sent events with the standard Responses event sequence
  </Card>

  <Card title="Citations" icon="quote-left" href="/responses-api/features/citations">
    Source annotations for grounded answers
  </Card>

  <Card title="Web Search Tool" icon="magnifying-glass" href="/responses-api/features/web-search-tool">
    Domain filters and the trail of searches and page reads behind each answer
  </Card>
</CardGroup>

## Examples

<CardGroup cols={2}>
  <Card title="Direct Requests" icon="paper-plane" href="/responses-api/examples/direct-requests">
    Call it directly: questions, deep research, enrichment, follow-ups
  </Card>

  <Card title="Research Subagent" icon="sitemap" href="/responses-api/examples/research-subagent">
    Expose it as a web-research tool of another LLM
  </Card>
</CardGroup>

## Next steps

* Review [OpenAI Responses Compatibility](/responses-api/openai-compatibility) for honored,
  ignored, and rejected request fields
* For long-running or batch research, use the [Task API](/task-api/task-quickstart) — the
  Responses API is synchronous only
* Check [pricing](/getting-started/pricing) and [rate limits](/getting-started/rate-limits)
