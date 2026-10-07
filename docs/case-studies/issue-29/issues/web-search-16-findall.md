---
id: WS-16
repo: link-assistant/web-search
title: FindAll-style entity discovery with match conditions, enrichment, and run lifecycle
depends_on: [WS-09, WS-13, WS-14]
labels: enhancement
---

## Summary

parallel.ai FindAll turns an objective into `entity_type` + `match_conditions`
(`/schema`), ingests candidates, runs discovery with generators
preview/base/core/pro, labels candidates generated/matched/unmatched with a
basis, then supports `enrich`, `extend`, `cancel`, `/events`, and webhooks
(`findall-api_findall-quickstart.md`, `findall-api_core-concepts_*.md`). It is
the last piece because it composes WS-09, WS-13, and WS-14.

## Scope

- `POST /v1beta/findall/schema` (LLM via WS-14 turns objective into conditions),
  `/ingest`, `/runs`, `/{id}`, `/result`, `/events`, `/extend`, `/enrich`,
  `/cancel` on the WS-13 store.
- Candidate generation from WS-09 plus multi-query search; match evaluation
  prompts producing `match_status` and basis citations.
- Generator tiers as presets (candidate count × verification depth).

## Acceptance criteria

- End-to-end test with stub LLM and mocked providers: 10 candidates, 4 matched with citations, extend adds more, cancel stops.

## References

- https://docs.parallel.ai/findall-api/findall-quickstart
