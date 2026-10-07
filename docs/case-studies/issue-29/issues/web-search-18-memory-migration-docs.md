---
id: WS-18
repo: link-assistant/web-search
title: Memory scope API over the run store and migration guides from parallel.ai, Tavily, Exa, and SERP APIs
depends_on: [WS-13, web-capture WC-01]
labels: enhancement, documentation
---

## Summary

parallel.ai Memory (`memory_scope_key` on runs; `/v1beta/memory/retrieve`,
`/evict`, `/clear`) lets later runs reuse earlier findings
(`resources_memory.md`), and its migration guide maps Tavily/Exa/SERP
parameters onto its request shape (`search_migrate-to-parallel.md`).

## Scope

- `memory_scope_key` accepted on WS-13 runs; retrieval with the WC-01 BM25
  ranker over stored results; evict/clear routes.
- `docs/migrate/` guides: from parallel.ai (base-URL swap + unsupported
  fields), from Tavily (`include_domains`, `start_date` → `after_date`,
  `country` → `location`, `include_raw_content` → extract), from Exa, from
  SERP APIs; plus an "advantages over parallel.ai" page (self-hosting,
  provider diversity, no result cap, caller-owned LLM, authenticated capture).

## Acceptance criteria

- Memory round-trip test; docs linked from README and `llms.txt` (WS-10).
