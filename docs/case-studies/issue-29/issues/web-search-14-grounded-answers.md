---
id: WS-14
repo: link-assistant/web-search
title: Grounded Task research with pluggable models and field-level basis
depends_on: [WS-04, WS-13]
labels: enhancement
---

## Summary

Parallel Task executes web-grounded text or structured research with field-level citations, reasoning, and confidence. Build the shared research engine here. WS-22/23 expose Responses/Chat, and WS-24 adds task specifications and interaction chains.

## Scope

- Caller-configured model backend for planning and synthesis with bounded search rounds, tool calls, tokens, and wall-clock budget.
- Search/extract evidence through WS-04 and existing transport, then synthesize text/JSON with Task-style basis per field.
- Model adapters for OpenAI-compatible local/remote endpoints and optional Anthropic backend, with secret-safe receipts.
- Use ajv/jsonschema for structure, record actual source use, and report backend/citation/schema failures without fabricated evidence.

## Acceptance criteria

- Stub research loop, unsupported backend, budget exhaustion, invalid JSON, uncited fields, and partial sources.
- Existing unit/integration and JS/Rust parity checks pass.

## References

- https://docs.parallel.ai/responses-api/responses-quickstart
- https://platform.openai.com/docs/api-reference/responses

## Implementation follow-through

1. Execute Task research through caller-configured model adapters: plan bounded retrieval, extract evidence, and synthesize text/JSON with field-level basis.
2. Keep evidence/provenance explicit and reject invented citation URLs. WS-24 handles specs/interactions and WS-22/23 format Responses/Chat.

## Additional verification

- Stub research loop, unsupported backend, budget exhaustion, invalid JSON, uncited fields, and partial sources.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
