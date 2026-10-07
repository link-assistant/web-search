---
id: WS-09
repo: link-assistant/web-search
title: Entity search for people and companies over open data sources
depends_on: [WS-01]
labels: enhancement
---

## Summary

parallel.ai Entity Search (`POST /v1beta/findall/entity-search`) returns a
ranked `entities[]{name, url, description}` for `entity_type` people or
companies and a natural-language `objective`, `match_limit` 5–1000
(`findall-api_entity-search.md`).

## Scope

- `entitySearch({ entity_type, objective, match_limit })` and
  `POST /v1beta/findall/entity-search`.
- Sources: Wikidata (`wbsearchentities` + SPARQL class filters), OpenCorporates,
  GitHub organizations/users, Crunchbase ODM when a key exists, plus general
  web search with entity post-processing (knowledge-panel style parsing).
- Dedupe by canonical URL/Wikidata QID; return `entity_set_id` for later WS-16 use.

## Acceptance criteria

- Mocked tests for Wikidata and OpenCorporates adapters; `match_limit` respected.

## References

- https://docs.parallel.ai/findall-api/entity-search
- https://www.wikidata.org/w/api.php
