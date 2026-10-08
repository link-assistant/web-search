---
id: WC-12
repo: link-assistant/web-capture
title: Extraction quality/parity fixtures for HTML, multilingual pages, JS, and PDF
depends_on: [WC-01, WC-03, WC-06]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Extraction quality/parity fixtures for HTML, multilingual pages, JS, and PDF**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Add local article/table/code/multilingual/SPA/PDF fixtures with expected markdown/metadata/relevant passages.
- Measure boilerplate removal/content retention/link-table preservation/date attribution/budget correctness.
- Compare existing converters with Readability/trafilatura/Kreuzberg candidates using the same corpus and licenses.
- Publish offline evaluation and opt-in Parallel comparisons with path receipts and format limits.

## Solution alternatives and implementation plan

Add a common fixture suite before choosing converter replacements and make only evidence-driven targeted improvements.

1. Inspect the existing capture transport, conversion modules, server, CLI, and capture receipts. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- JS/Rust equivalent normalized outputs have documented converter differences.
- Tables/code/links/Unicode survive within budgets without invalid truncation.
- Finite corpus and bounded converter/browser workers record corpus/runtime versions and quality evidence.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/extract/extract-quickstart
- https://docs.parallel.ai/extract/best-practices
- https://docs.parallel.ai/search/evaluating-search
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
