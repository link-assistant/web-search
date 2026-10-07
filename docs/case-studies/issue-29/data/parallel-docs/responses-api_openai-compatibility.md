> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# OpenAI Responses Compatibility

> Field-by-field compatibility reference for the OpenAI Responses wire format

The Responses API models the canonical OpenAI Responses request
(`POST https://api.parallel.ai/v1/responses`), so a stock `openai` SDK client works by
changing only `base_url`, `api_key`, and `model`. This page documents exactly which fields
are honored, which are accepted for SDK compatibility but ignored, and which are rejected.

## Request fields

### Honored

| Field | Type | Meaning |
| - | - | - |
| `model` | `string` | Must be `"parallel"` — the only model. The tier is chosen via `reasoning.effort`. |
| `input` | `string` \| `message[]` | The question, or a list of `{role, content}` messages. **Text only.** Must be non-empty and contain at least one `user` message. |
| `reasoning` | `{"effort": "low" \| "medium" \| "high"}` | Selects the tier. Defaults to `medium`. |
| `instructions` | `string` | System instructions prepended to the request. |
| `text` | `object` | Text-output config, including [structured output](/responses-api/features/structured-outputs) — structured content is returned JSON-encoded in the output text. |
| `stream` | `boolean` | [SSE streaming](/responses-api/features/streaming-events). The answer currently arrives as a single text delta, not token-by-token. |
| `previous_response_id` | `string` | Prior response id, for [multi-turn context](/responses-api/features/statefulness). |
| `metadata` | `{string: string}` | Arbitrary tags echoed back on the response. Max 16 keys, ≤64 chars/key, ≤512 chars/value. |
| `tools` | `tool[]` | Two tool types have an effect. A `web_search` tool's `filters` (`allowed_domains`, `blocked_domains`) restrict the domains searched, read, and cited (see [Web Search Tool](/responses-api/features/web-search-tool)); at most one per request. An `mcp` tool gives the model a remote MCP server to call (see [MCP Tools](/responses-api/features/mcp-tools)); at most 10 per request. Every other tool is accepted and ignored. |
| `data_sources` | `{"free": string[], "pay_per_use": string[]}` | Parallel extension. Data Connectors to query in addition to the web, for `medium` and `high` effort (see [Data Connectors](/resources/data-connectors#responses-api)). With the OpenAI Python SDK, pass it in `extra_body`. |

### Accepted but ignored (for SDK compatibility)

`tool_choice`, `parallel_tool_calls`, `temperature`, `top_p`, `max_output_tokens`,
`truncation`, `store`, `user`, `include`, any tool other than `web_search` and `mcp`, the
`web_search` tool's `search_context_size` and `user_location`, and the `mcp` tool's
`server_description`.

These are part of the OpenAI contract so a stock SDK client's payload never fails
validation, but the backend does not act on them today. Grounding is automatic, so a
`web_search` tool without `filters` changes nothing. `store` is likewise ignored:
responses are always stored so they can be referenced via `previous_response_id` (see
[Statefulness](/responses-api/features/statefulness)). Unknown top-level fields are
silently dropped, so newer OpenAI SDK versions won't break.

<Note>
  Ignored fields come back on the response object with their **default values**
  (`temperature: null`, `tool_choice: "auto"`), not the values you sent. `tools` echoes
  your `web_search` tool as you sent it, including its ignored `search_context_size` and
  `user_location`, and your `mcp` tools with `headers` and `authorization` removed and any
  `server_url` query string masked. It is `[]` when you sent neither.
</Note>

### Rejected

| Request | Status | Error |
| - | - | - |
| `background: true` | `400` | "background mode is not supported on /v1/responses; submit long-running work via the [Task API](/task-api/task-quickstart) (POST /v1/tasks/runs) instead." |
| Non-text (multimodal) input parts | `400` | "input contains unsupported non-text content parts: input\_image (this endpoint only accepts text input today)" |
| Empty `input` | `400` | "input must not be empty." |
| `input` with no `user` message | `400` | "input must contain at least one user message." |
| Any `model` other than `parallel` | `400` | "The Responses API only supports model 'parallel'." |
| More than one `web_search` tool | `400` | "tools may contain at most one web\_search tool." |
| Malformed `web_search` tool (`filters` not an object, unknown filter key) | `400` | "Request validation error. \[...]" |
| Domain filter entry with a scheme, port, query string, or fragment | `400` | "Invalid source filter(s) in source\_policy: \[...]. Expected format: plain domains (e.g., example.com, subdomain.example.gov) or bare domain extension starting with a period (e.g., .gov, .edu, .co.uk). \[...]" |
| More than 200 domain filter entries | `400` | "Total number of source filters in source\_policy cannot exceed 200." |
| Same entry in `allowed_domains` and `blocked_domains` | `400` | "include\_domains and exclude\_domains cannot have overlapping entries." |
| `mcp` tool without `require_approval: "never"` (omitted, `"always"`, or an object) | `400` | "Request validation error. \[...]" |
| `mcp` tool without `server_url`, with a blank `server_label`, or with `connector_id`, `tunnel_id`, or `defer_loading` | `400` | "Request validation error. \[...]" |
| `mcp` tool with `allowed_tools` as a filter object | `400` | "Request validation error. \[...]" |
| `mcp` tool with `allowed_tools: []` | `400` | "MCP server configuration requires either null or a non-empty list of allowed tools." |
| More than 10 `mcp` tools | `400` | "Number of MCP servers provided (11) exceeds the maximum allowed limit of 10." |
| `data_sources` with `reasoning.effort: "low"` | `400` | "data\_sources requires reasoning.effort='medium' or 'high'." |
| `data_sources` connector name that is unknown, in the wrong list, or an Index Partner | `400` | "Request validation error. \[...] Unknown data partner(s) in pay\_per\_use: \[...]" |
| `data_sources` connector not yet available to your organization | `400` | "Selected data provider(s) are unavailable: \[...]. Available: \[...]." |
| `data_sources` connector with the same name as an `mcp` tool's `server_label` | `400` | "Selected data provider names collide with mcp\_servers names: \[...]." |
| `additional_data_providers` (retired) | `400` | "Request validation error. \[...] additional\_data\_providers was replaced by data\_sources \[...]" |

Errors use the standard OpenAI error envelope:

```json theme={"system"}
{
  "error": {
    "message": "The Responses API only supports model 'parallel'.",
    "type": "invalid_request_error",
    "param": null,
    "code": null
  }
}
```

Requests over your quota return `429` — back off and retry, and see
[rate limits](/getting-started/rate-limits) for current limits.

## Response fields

A standard OpenAI Responses object. The fields you'll typically read:

| Field | Meaning |
| - | - |
| `output_text` | The final answer string (SDK convenience accessor). |
| `output[]` | Output items, read by `type`: one `web_search_call` item per search the model ran or page it read (see [Web Search Tool](/responses-api/features/web-search-tool)), one `mcp_list_tools` item per MCP server reported and one `mcp_call` item per MCP tool call (see [MCP Tools](/responses-api/features/mcp-tools)), and the `message` item whose `content[]` carries the answer text. |
| `usage` | Token counts (`input_tokens`, `output_tokens`, `total_tokens`). |
| `id` | Response id — pass as `previous_response_id` for follow-ups. |
| `metadata` | Your request metadata, echoed back. |
| `tools` | Your `web_search` and `mcp` tools, echoed back with MCP credentials removed; `[]` otherwise. |

<Note>
  `reasoning` and `text` are echoed as `null` on the response even when they were sent and
  honored — read the tier from your request, not the response. Source citations are returned
  as `url_citation` annotations on the output text — see
  [Citations](/responses-api/features/citations).
</Note>
