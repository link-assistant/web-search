> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# MCP Tools

> Give the Responses API a remote MCP server to call while it researches

Pass the OpenAI `mcp` tool to let the model call tools on a remote MCP server alongside its
web research. Parallel connects to the server, lists its tools, and calls them when they are
useful for the question. The tools found on each server are reported as an `mcp_list_tools`
output item and each call as an `mcp_call` item, in the same shapes OpenAI returns them.

Use an `mcp` tool for a server you run or a provider your organization already licenses. For
Data Connectors Parallel manages, use [`data_sources`](/resources/data-connectors#responses-api)
instead.

## Add a server

<CodeGroup>
  ```bash cURL theme={"system"}
  curl https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "What transport protocols does the MCP Python SDK support?",
      "tools": [
        {
          "type": "mcp",
          "server_label": "deepwiki",
          "server_url": "https://mcp.deepwiki.com/mcp",
          "require_approval": "never"
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
      input="What transport protocols does the MCP Python SDK support?",
      tools=[
          {
              "type": "mcp",
              "server_label": "deepwiki",
              "server_url": "https://mcp.deepwiki.com/mcp",
              "require_approval": "never",
          }
      ],
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
    input: "What transport protocols does the MCP Python SDK support?",
    tools: [
      {
        type: "mcp",
        server_label: "deepwiki",
        server_url: "https://mcp.deepwiki.com/mcp",
        require_approval: "never",
      },
    ],
  });

  console.log(response.output_text);
  ```
</CodeGroup>

| Field | Type | Description |
| - | - | - |
| `type` | `"mcp"` | Required. |
| `server_label` | `string` | Required. A name for the server, reported as `server_label` on its `mcp_call` items. Must not be blank or match a connector named in `data_sources`. |
| `server_url` | `string` | Required. URL of the MCP server. |
| `require_approval` | `"never"` | Required. Parallel runs tool calls without an approval step, so this must be set to `"never"`. Omitting it, which OpenAI treats as `"always"`, is rejected. |
| `headers` | `{string: string}` | Optional. HTTP headers sent on every request to the server, such as an API key. |
| `authorization` | `string` | Optional. An OAuth access token, sent as `Authorization: Bearer <token>`. |
| `allowed_tools` | `string[]` | Optional. Names of the tools the model may call. Omit to allow every tool on the server. |

`server_description` is accepted and ignored. Parallel does not run the OAuth flow for you;
obtain a token separately and pass it as `authorization` or in `headers`:

```json theme={"system"}
{
  "type": "mcp",
  "server_label": "crm",
  "server_url": "https://mcp.example.com/mcp",
  "require_approval": "never",
  "headers": { "X-Api-Key": "YOUR_PROVIDER_KEY" },
  "allowed_tools": ["search_accounts", "get_account"]
}
```

A request may carry up to 10 `mcp` tools, but fewer usually gives better answers. MCP tools
work on every `reasoning.effort` tier, and can be combined with a
[`web_search` tool](/responses-api/features/web-search-tool) and with `data_sources`.

### Restrictions

* Only servers using the Streamable HTTP transport are supported, and only their tools: MCP
  resources and prompts are not used.
* `require_approval` must be `"never"`. There is no approval round trip, so the response
  never contains `mcp_approval_request` items.
* `connector_id` (OpenAI connectors), `tunnel_id`, and `defer_loading` are not supported and
  are rejected when set, including `defer_loading: false`.
* `allowed_tools` must be a list of tool names. An empty list and the filter-object form
  (`{"read_only": true}`) are rejected.
* `allowed_tools` names that match no tool on the server are ignored. If none match, the
  server is never called and its [`mcp_list_tools` item](#check-server-connections) reports
  the error.

Rejected tools return a `400` with the reason; see
[OpenAI Responses Compatibility](/responses-api/openai-compatibility#rejected).

## Check server connections

A completed response contains one `mcp_list_tools` item per `mcp` tool, listing the tools the
model could call on that server after `allowed_tools` filtering:

```json theme={"system"}
{
  "id": "mcpl_resp_9c6b43a6-c6ae-4193-95d8-bc199e09fa4f_0",
  "type": "mcp_list_tools",
  "server_label": "deepwiki",
  "tools": [
    {
      "name": "ask_wiki_question",
      "description": "Ask any question about a GitHub repository's codebase and get an AI-powered answer\ngrounded in its DeepWiki.",
      "input_schema": {
        "type": "object",
        "properties": {
          "repoName": {
            "anyOf": [{ "type": "string" }, { "type": "array", "items": { "type": "string" } }],
            "description": "GitHub repository or list of repositories (max 10) in owner/repo format."
          },
          "question": { "type": "string", "description": "The question to ask about the repository." }
        },
        "required": ["repoName", "question"]
      },
      "annotations": null
    }
  ],
  "error": null
}
```

If Parallel can't connect to the server or list its tools, `tools` is empty and `error` says
why. The request still completes, on web research and any other servers:

```json theme={"system"}
{
  "id": "mcpl_resp_9c6b43a6-c6ae-4193-95d8-bc199e09fa4f_1",
  "type": "mcp_list_tools",
  "server_label": "broken",
  "tools": [],
  "error": "mcp_protocol_error (32600): Session terminated. Reference ID: bdcd28f6-6758-4742-a241-21ff5bcbbe81"
}
```

A server that lists no tools, or none that match `allowed_tools`, is reported the same way.
Connectors named in `data_sources` get an `mcp_list_tools` item only when they fail, with an
empty `tools` list.

## Read tool calls

A completed response contains one `mcp_call` item per tool call the model made, for your
`mcp` tools and for the connectors you named in `data_sources`:

```json theme={"system"}
{
  "id": "mcp_resp_a0fa0e42-fa1b-4bf0-ae62-baca311fd5b6_0",
  "type": "mcp_call",
  "server_label": "deepwiki",
  "name": "ask_wiki_question",
  "arguments": "{\"repoName\": \"modelcontextprotocol/python-sdk\", \"question\": \"Which transports does the SDK support?\"}",
  "output": "The SDK supports three transports: stdio, SSE, and Streamable HTTP...",
  "error": null,
  "status": "completed"
}
```

| Field | Meaning |
| - | - |
| `server_label` | The server called: your tool's `server_label`, or the connector name from `data_sources`. |
| `name` | The tool called. |
| `arguments` | The tool input, as a JSON-encoded string. |
| `output` | The text the tool returned, or `null` if the call failed. |
| `error` | Why the call failed, or `null` if it succeeded. |
| `status` | `completed` or `failed`. A call is `failed` when the server returned a tool error or the call could not be made. |

The model decides when a tool is useful, so a server may be called several times or not at
all. A failed call does not fail the response. Calls to Index Partners are not reported.

A failed call's `error` is an object, in the same shape OpenAI returns:

| `error.type` | Other fields | Meaning |
| - | - | - |
| `mcp_tool_execution_error` | `content` | The tool reported an error, or the call failed for another reason. `content` is the text. |
| `mcp_protocol_error` | `code`, `message` | The server returned a JSON-RPC error. |
| `http_error` | `code`, `message` | The request to the server failed. `code` is the HTTP status, or `0` if no response came back. |

Read `output` by item `type` rather than by position. `mcp_list_tools` items, then
`mcp_call` items, follow the `web_search_call` items and precede the `message` item in a
non-streaming response, but follow the `message` item in a streamed one.

<CodeGroup>
  ```python Python theme={"system"}
  for item in response.output:
      if item.type == "mcp_call":
          result = item.output if item.status == "completed" else item.error
          print(f"{item.server_label}.{item.name}({item.arguments}) -> {result}")
  ```

  ```typescript TypeScript theme={"system"}
  for (const item of response.output) {
    if (item.type === "mcp_call") {
      const result = item.status === "completed" ? item.output : JSON.stringify(item.error);
      console.log(`${item.server_label}.${item.name}(${item.arguments}) -> ${result}`);
    }
  }
  ```
</CodeGroup>

The response echoes your `mcp` tools on `tools` with credentials removed: `headers` and
`authorization` are `null`, and any query string on `server_url` is replaced with `***`.

With [streaming](/responses-api/features/streaming-events) enabled, server connections and
tool calls are reported once research finishes, after the answer's text delta. Each
`mcp_list_tools` item is sent as `response.output_item.added`,
`response.mcp_list_tools.completed` or `response.mcp_list_tools.failed`, and
`response.output_item.done`, then each `mcp_call` item the same way with
`response.mcp_call.completed` or `response.mcp_call.failed`. The
`response.mcp_list_tools.in_progress`, `response.mcp_call.in_progress`, and
`response.mcp_call_arguments.*` events are not sent.
