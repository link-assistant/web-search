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

## Implementation follow-through

1. Separate natural-language objective planning from source adapters and validate supported source filters.
2. Preserve ambiguous entities with stable IDs/canonical websites and provenance rather than deduping by name alone.

## Additional verification

- Same-name distinct entities, pagination, match_limit boundaries, unavailable licensed source, and stable ranking.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
