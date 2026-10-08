---
id: WS-03
repo: link-assistant/web-search
title: ISO alpha-2 location targeting mapped to provider parameters with warnings
depends_on: [WS-01]
labels: enhancement
---

## Summary

parallel.ai `advanced_settings.location` takes a lowercase ISO 3166-1 alpha-2
code from a list of 37 countries (`gb`, not `uk`), silently ignores unknown
codes with a warning, and biases results toward that country
(`search_advanced-search-settings.md`). web-search exposes free-form `region`
and `language` whose provider mapping is undocumented.

## Scope

- Shared `location` → provider parameter table: Google `gl`/`hl`, Bing
  `cc`/`mkt`, DuckDuckGo `kl`, Brave `country`/`search_lang`, Startpage, Mojeek.
- Normalize `uk` → `gb`, uppercase → lowercase; validate with
  `i18n-iso-countries` (JS) / `isocountry` (Rust); the parallel.ai list of 37 is
  the tested subset but any valid code is accepted.
- Warning `location_unsupported` per provider that cannot honour it.
- `region` stays as an alias.

## Acceptance criteria

- Tests assert the generated query parameters per provider for `gb`, `jp`, `br`.
- Invalid code produces a warning and the request still runs.

## References

- https://docs.parallel.ai/search/advanced-search-settings

## Implementation follow-through

1. Inspect provider descriptors before adding country mappings and separate country bias from language.
2. Use data-driven code tables, preserve region/language aliases, and report unsupported provider mappings.

## Additional verification

- Country-only/language-only requests, uppercase/gb normalization, unsupported provider, and generated parameters.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
