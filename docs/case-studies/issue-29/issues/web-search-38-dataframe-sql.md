---
id: WS-38
repo: link-assistant/web-search
title: DataFrame and SQL enrichment: DuckDB, Polars, Spark, BigQuery, Snowflake, Supabase
depends_on: [WS-21, WS-31]
labels: enhancement
---

## Summary

The issue #29 audit identified this independently testable requirement in Parallel's public documentation: **DataFrame and SQL enrichment: DuckDB, Polars, Spark, BigQuery, Snowflake, Supabase**. The scope below defines the local behavior to implement, with contract tests for full feature coverage.

## Scope

- Define stable row-ID batching/schema/retry/checkpoint contract with bounded work.
- Publish DuckDB/Polars adapters and Spark UDF recipes preserving row alignment/types/null/error columns.
- Provide BigQuery remote-function and Snowflake UDTF deployment templates with secret setup and batch limits.
- Provide Supabase Edge Function integration and explicit cloud-account prerequisites.

## Solution alternatives and implementation plan

Select shared task-group client plus lightweight platform-native wrappers. Reuse current DataFrame APIs and remote-function templates.

1. Inspect the existing search registry, transport, server, CLI, and shared JS/Rust types. Define request/output/error fixtures from the linked sources before integrating the feature.
2. Implement the scope through existing modules and the dependency interfaces. Keep external services injectable and optional; default tracing remains off and redacts secrets when enabled.
3. Add the tests below, update types/OpenAPI and installed-package examples, and run existing unit/integration and JS/Rust parity checks.
4. Document supported versions, configuration, deliberate extensions, limits, and external prerequisites. Release only the capabilities covered by contract tests.

## Acceptance criteria

- Finite fixture tables with duplicate IDs/nulls/mixed outcomes preserve row count/order.
- DuckDB/Polars run locally and cloud adapters have mocked wire tests.
- Restart/cancellation resumes checkpoints without repeating paid jobs.
- Offline tests use mocks or local servers with finite inputs and test deadlines. Live comparisons are opt-in.
- Existing library/CLI/HTTP behavior continues to pass its tests. Optional dependencies are not required for basic search or capture.

## References

- https://docs.parallel.ai/data-integrations/overview
- https://docs.parallel.ai/data-integrations/duckdb
- https://docs.parallel.ai/data-integrations/polars
- https://docs.parallel.ai/data-integrations/spark
- https://docs.parallel.ai/data-integrations/bigquery
- https://docs.parallel.ai/data-integrations/snowflake
- https://docs.parallel.ai/data-integrations/supabase
- [Case study and source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Corresponding slash-to-underscore filenames are in `data/parallel-docs/`, with source URLs and checksums in `manifest.json`.
