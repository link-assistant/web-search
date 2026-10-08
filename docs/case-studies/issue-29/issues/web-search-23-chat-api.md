---
id: WS-23
repo: link-assistant/web-search
title: Chat Completions compatibility over grounded research
depends_on: [WS-14, WS-19]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Chat Completions compatibility over grounded research**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Mount POST /v1beta/chat/completions with documented messages/models/stream/configuration.
- Map message history to grounded research and results to choices/usage/finish reasons.
- Emit ordered Chat-specific SSE chunks and terminal sentinel independently of Responses event names.
- Publish supported/unsupported field matrix and configurable base URL examples.

## Solution alternatives and implementation plan

Select a thin adapter over the shared model/research interface, reusing SSE framing while retaining Chat-specific payloads.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- A compatible client obtains text and streaming chunks using local retrieval and a stub LLM.
- Empty messages, unsupported options, cancellation, and backend failure have explicit outcomes.
- Multi-turn history stays scoped and receipts contain no credentials.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/api-reference/chat-api-beta/chat-completions
- https://docs.parallel.ai/getting-started/choose-an-api
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
