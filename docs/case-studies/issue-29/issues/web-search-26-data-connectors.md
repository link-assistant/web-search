---
id: WS-26
repo: link-assistant/web-search
title: Research connector registry for free, paid, licensed, and BYOL data
depends_on: [WS-25, WS-17]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **Research connector registry for free, paid, licensed, and BYOL data**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Extend descriptors with access model, schemas, credential/license requirements, processor support, citations, and call accounting.
- Add reference adapters for pubmed, clinical_trials, chembl, biorxiv/medrxiv, npi_registry, and cms_coverage using injectable transports.
- Validate Task advanced_settings.data_sources versus Responses top-level data_sources, free/paid list membership, unsupported processors, and MCP name collisions.
- Support operator-installed paid/licensed sources including a Carbon Arc adapter contract, treating proprietary access as a license prerequisite.

## Solution alternatives and implementation plan

Select a plugin registry instead of hard-coded proprietary datasets. Reuse existing REST transports and WS-25 MCP for BYOL.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- All six free connectors have mocked response/citation/error contracts.
- No selected connector is invoked unnecessarily and failures remain attributable.
- Unavailable paid connectors fail without charges, and a fake paid adapter counts successful calls only.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/resources/data-connectors
- https://docs.parallel.ai/task-api/data-connectors
- https://docs.parallel.ai/responses-api/features/data-connectors
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
