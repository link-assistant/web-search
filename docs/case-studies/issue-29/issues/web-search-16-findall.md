---
id: WS-16
repo: link-assistant/web-search
title: FindAll core discovery, candidate matching, and research basis
depends_on: [WS-09, WS-13, WS-14, WS-19, WS-20, WS-24]
labels: enhancement
---

## Summary

Parallel FindAll compiles an objective into entity/match conditions and returns candidates with evaluated conditions and citations. Core discovery belongs here, enrichment in WS-27, and lifecycle/refresh in WS-28.

## Scope

- POST /v1beta/findall/runs, GET status/result/schema, with spec inference through documented POST /v1beta/findall/ingest from WS-24.
- Candidate generation via WS-09 plus multi-query search, stable IDs, deduplication, and condition-by-condition matched/unmatched evidence.
- Configurable generator budgets and partial results on resource exhaustion; persist source attribution and failures.
- Do not invent a POST /schema endpoint. Compose the existing task/research runtime rather than independent model execution.

## Acceptance criteria

- Finite generator budget, matched/unmatched/unknown condition evidence, ambiguous dedupe, partial result, and schema.
- Existing unit/integration and JS/Rust parity checks pass.

## References

- https://docs.parallel.ai/findall-api/findall-quickstart

## Implementation follow-through

1. Implement core discovery/matching on stable candidates with attributable condition basis and partial results.
2. Use WS-24 inference through POST /v1beta/findall/ingest and GET run/schema retrieval. WS-27/28 own enrichment and lifecycle controls.

## Additional verification

- Finite generator budget, matched/unmatched/unknown condition evidence, ambiguous dedupe, partial result, and schema.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
