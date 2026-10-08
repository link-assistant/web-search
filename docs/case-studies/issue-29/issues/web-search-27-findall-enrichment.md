---
id: WS-27
repo: link-assistant/web-search
title: FindAll per-candidate enrichment with schemas, citations, and partial progress
depends_on: [WS-16, WS-19, WS-24, WS-25]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **FindAll per-candidate enrichment with schemas, citations, and partial progress**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Implement POST run/enrich with saved request specifications, schema validation, processor configuration, and optional MCP sources.
- Keep matching and enrichment basis separate and preserve match conditions/status.
- Expose per-field progress/failure through results and events, retaining successful fields.
- Persist exact enrichment payloads for deliberate reuse on refresh rather than assuming GET /schema includes them.

## Solution alternatives and implementation plan

Reuse Task execution and durable candidate work items rather than a second enrichment engine.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Ten fixture candidates with one failing source retain successful cited fields.
- Enrichment never changes matched/unmatched counts and repeat requests have defined merge semantics.
- Restart resumes pending fields and schema retrieval remains separate from saved request payloads.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/findall-api/features/findall-enrich
- https://docs.parallel.ai/findall-api/features/findall-refresh
- https://docs.parallel.ai/api-reference/findall/add-enrichment-to-findall-run
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
