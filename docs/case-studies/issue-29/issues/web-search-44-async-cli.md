---
id: WS-44
repo: link-assistant/web-search
title: CLI for Tasks, groups, Responses, FindAll, Monitor, Memory, and accounts
depends_on: [WS-12, WS-21, WS-22, WS-23, WS-28, WS-29, WS-18, WS-33, WS-34]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **CLI for Tasks, groups, Responses, FindAll, Monitor, Memory, and accounts**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Add create/status/result/stream research and group enrichment commands plus Responses/Chat.
- Add FindAll preview/extend/enrich/cancel/refresh, Monitor events/update/trigger/cancel, and Memory retrieve/evict/clear.
- Add app/key/balance commands with login/profiles/custom base URLs and explicit secret output.
- Support stdin/file JSON, stable exit codes, finite wait/cancellation, and shared JS/Rust help/command parity.

## Solution alternatives and implementation plan

Extend existing parsers and typed service interfaces instead of writing a second shell HTTP client. Share command metadata where useful.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Installed binaries agree on mocked command JSON/exit codes for each family.
- SIGINT stops polling/streams and invalid inputs do not create work.
- Legacy positional search works and secrets never appear in ordinary diagnostics.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/integrations/cli
- https://docs.parallel.ai/integrations/developer-quickstart
- https://docs.parallel.ai/integrations/account-api
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
