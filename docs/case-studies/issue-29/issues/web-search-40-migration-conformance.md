---
id: WS-40
repo: link-assistant/web-search
title: Migration guides and conformance fixtures for Parallel, Tavily, Exa, and SERP
depends_on: [WS-10, WS-22, WS-23, WS-28, WS-29, WS-33, WS-34]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Migration guides and conformance fixtures for Parallel, Tavily, Exa, and SERP**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Map shared endpoints and competitor fields to search/source-policy/extract options with concrete examples.
- Test GA nested advanced_settings and explicitly separate beta compatibility adapters.
- Cover current beta FindAll/Ingest/Memory/Chat and GA Monitor plus selected legacy candidates/alpha-monitor/task-group aliases.
- Publish exact compatibility/extensions/unsupported fields and third-party prerequisite matrix.

## Solution alternatives and implementation plan

Select versioned fixtures and explicit adapters instead of replacing every beta prefix with v1. Reuse shared OpenAPI and schema validation.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Fixtures cover every current product/account operation and selected legacy aliases.
- Quickstart bodies run locally against JS/Rust for the claimed subset.
- No guide asserts equal latency/model quality/licensed data/marketplace availability without measurements.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/search/migrate-to-parallel
- https://docs.parallel.ai/search/search-migration-guide
- https://docs.parallel.ai/extract/extract-migration-guide
- https://docs.parallel.ai/findall-api/findall-migration-guide
- https://docs.parallel.ai/monitor-api/monitor-migration-guide
- https://docs.parallel.ai/responses-api/openai-compatibility
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
