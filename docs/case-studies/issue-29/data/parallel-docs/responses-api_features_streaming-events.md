> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Responses API Streaming Events

> Stream Responses API results as server-sent events

Set `stream: true` to receive the response as server-sent events (SSE,
`Content-Type: text/event-stream`) following the standard OpenAI Responses event sequence.
The stock OpenAI SDK's streaming interface works unchanged.

## Event sequence

```
response.created
response.in_progress
response.output_item.added              (web_search_call, one per search or page read)
response.web_search_call.in_progress
response.web_search_call.searching
response.output_item.added              (message)
response.content_part.added
response.output_text.delta
response.web_search_call.completed      (one per search or page read)
response.output_item.done               (web_search_call)
response.output_item.added              (mcp_list_tools, one per mcp tool and per failed connector)
response.mcp_list_tools.completed       (or response.mcp_list_tools.failed)
response.output_item.done               (mcp_list_tools)
response.output_item.added              (mcp_call, one per MCP tool call)
response.mcp_call.completed             (or response.mcp_call.failed)
response.output_item.done               (mcp_call)
response.output_text.annotation.added   (one per citation)
response.output_text.done
response.content_part.done
response.output_item.done               (message)
response.completed
```

Each web search the model runs, and each page it reads, is announced as it starts with its
own `response.output_item.added`, `response.web_search_call.in_progress`, and
`response.web_search_call.searching` events. These steps run concurrently and report only
their start, so the `web_search_call` items close together once research finishes: their
`response.web_search_call.completed` and `response.output_item.done` events follow the
answer's text delta and precede its citation annotations. `output_index` counts items in
opening order, so the `message` item takes the index after the last search opened before
it. See [Web Search Tool](/responses-api/features/web-search-tool) for the item shape.

[MCP tools](/responses-api/features/mcp-tools) and connectors selected with `data_sources`
are reported once research finishes, after the `web_search_call` items close: first an
`mcp_list_tools` item for each `mcp` tool (and for each `data_sources` connector, only when it
failed), marked `response.mcp_list_tools.completed` or `response.mcp_list_tools.failed`, then
each `mcp_call` item, marked `response.mcp_call.completed` or `response.mcp_call.failed`.
Each item is opened, marked, and closed in turn, and takes an `output_index` after the
`message` item. The `response.mcp_list_tools.in_progress`, `response.mcp_call.in_progress`,
and `response.mcp_call_arguments.*` events are not sent.

Today the full answer text arrives as a single `response.output_text.delta` once research
completes — there is no token-by-token streaming yet. The early `response.created` and
`response.in_progress` events still arrive up front, so streaming works well as a connection
acknowledgment during longer requests. Consume deltas in a loop rather than assuming one
chunk; granularity may become finer in the future.

## Usage

<CodeGroup>
  ```bash cURL theme={"system"}
  # -N disables buffering so events print as they arrive
  curl -N https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "Who is the current CEO of Nvidia?",
      "reasoning": {"effort": "low"},
      "stream": true
    }'
  ```

  ```python Python theme={"system"}
  import os
  from openai import OpenAI

  client = OpenAI(
      api_key=os.environ["PARALLEL_API_KEY"],
      base_url="https://api.parallel.ai/v1",
  )

  stream = client.responses.create(
      model="parallel",
      input="Who is the current CEO of Nvidia?",
      reasoning={"effort": "low"},
      stream=True,
  )

  for event in stream:
      if event.type == "response.output_text.delta":
          print(event.delta, end="", flush=True)
      elif event.type == "response.completed":
          print()  # final Response object is on event.response
  ```

  ```typescript TypeScript theme={"system"}
  import OpenAI from "openai";

  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
  });

  const stream = await client.responses.create({
    model: "parallel",
    input: "Who is the current CEO of Nvidia?",
    reasoning: { effort: "low" },
    stream: true,
  });

  for await (const event of stream) {
    if (event.type === "response.output_text.delta") {
      process.stdout.write(event.delta);
    } else if (event.type === "response.completed") {
      process.stdout.write("\n"); // final Response object is on event.response
    }
  }
  ```
</CodeGroup>

The `response.completed` event carries the complete final Response object — the same shape
a non-streaming request returns, including `usage`.

Source citations arrive as `response.output_text.annotation.added` events after the text
delta — see [Citations](/responses-api/features/citations).
