---
id: WS-41
repo: link-assistant/web-search
title: Reproducible search/research evaluation for relevance, latency, citations, and cost
depends_on: [WS-06, WS-14, WC-12]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Reproducible search/research evaluation for relevance, latency, citations, and cost**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Create versioned objective/query sets for locale/policy/freshness/images/entities/research-schema tasks.
- Measure recall/nDCG/citation correctness/excerpt efficiency/p50-p95 latency/errors/provider-model cost.
- Run fixed offline fixtures in CI and opt-in same-budget comparisons with Parallel/Tavily/Exa.
- Publish raw results, versions, uncertainty, and thresholds separating self-hosting advantages from measured quality.

## Solution alternatives and implementation plan

Compare promptfoo with a minimal fixture runner and select the smallest reproducible harness. Use standard ranking metrics and WC-12 extraction corpus.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Seeded finite benchmark works without API keys and never fabricates live scores.
- Only the search adapter changes between live comparisons while synthesis harness stays fixed.
- Fixtures catch broken citations/irrelevant excerpts/missed results with bounded input/concurrency/memory.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/search/evaluating-search
- https://docs.parallel.ai/search/best-practices
- https://docs.parallel.ai/task-api/best-practices
- https://docs.parallel.ai/image-search/best-practices
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
