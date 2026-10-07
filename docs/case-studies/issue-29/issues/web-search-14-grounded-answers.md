---
id: WS-14
repo: link-assistant/web-search
title: Grounded answers: OpenAI Responses and Chat Completions compatible endpoints with citations and structured output
depends_on: [WS-04, WS-13]
labels: enhancement
---

## Summary

parallel.ai's Responses API (`POST /v1/responses`, OpenAI wire format) and
beta Chat Completions return answers with `url_citation` annotations,
`web_search_call` items, JSON-schema structured output, streaming SSE, and a
`reasoning.effort` knob (`responses-api_responses-quickstart.md`,
`responses-api_features_*.md`). web-search can provide the retrieval half and
let the caller plug any LLM.

## Scope

- `POST /v1/responses` and `POST /v1beta/chat/completions` that run
  search (WS-01/02) → excerpts (WS-04) → synthesis through a configured
  OpenAI-compatible endpoint (`LLM_BASE_URL`, `LLM_API_KEY`, `LLM_MODEL`; works
  with Ollama, vLLM, OpenAI, Anthropic via adapter).
- `reasoning.effort` mapped to search mode and number of search rounds;
  `previous_response_id` via the WS-13 run store; `text.format` json_schema
  validated with `ajv`/`jsonschema`.
- Output annotations with character offsets; `web_search_call` items for
  `search` and `open_page` actions; SSE event stream matching the OpenAI event names.
- Task-style `basis[]` (citations, reasoning, confidence) for structured fields.

## Acceptance criteria

- Tests with a stub LLM server: citations point to returned URLs; schema output validated; streaming emits `response.completed`.

## References

- https://docs.parallel.ai/responses-api/responses-quickstart
- https://platform.openai.com/docs/api-reference/responses
