---
id: WS-36
repo: link-assistant/web-search
title: LLM framework and gateway adapters for grounded search
depends_on: [WS-10, WS-11, WS-22, WS-26, WS-31]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **LLM framework and gateway adapters for grounded search**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Publish shared Search/Extract tool adapters for OpenAI-style/Anthropic-style/Gemini-style calling.
- Add LangChain wrappers, LiteLLM gateway configuration, Ollama examples, and OpenRouter search adapter contract.
- Provide Gemini Enterprise grounding mapping with source attribution and explicit external registration prerequisites.
- Preserve policy/budgets/errors/cancellation/usage through all adapters.

## Solution alternatives and implementation plan

Select framework-native thin wrappers over shared clients/schemas. Evaluate maintained SDKs before adding new tool loops.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Stub tool-call fixtures round-trip arguments/results for each format without live model keys.
- Installed examples run locally and cancellation/errors remain typed.
- Integration matrix records tested versions and registration state without claiming unverified listings.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/integrations/langchain
- https://docs.parallel.ai/integrations/litellm
- https://docs.parallel.ai/integrations/openrouter
- https://docs.parallel.ai/integrations/ollama-tool-calling
- https://docs.parallel.ai/integrations/google-gemini-enterprise
- https://docs.parallel.ai/integrations/anthropic-tool-calling
- https://docs.parallel.ai/integrations/openai-tool-calling
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
