---
id: WS-02
repo: link-assistant/web-search
title: Source policy: include/exclude domains and path prefixes, after_date, and normalization rules
depends_on: [WS-01, web-capture WC-03]
labels: enhancement
---

## Summary

parallel.ai `source_policy{include_domains, exclude_domains, after_date}` has
precise semantics (`resources_source-policy.md`): apex domains cover all
subdomains, `www.` is normalized, path prefixes match at segment boundaries
and are case-sensitive, bare suffixes such as `.gov` or `.co.uk` are allowed,
wildcards/schemes/ports/query strings are rejected, `exclude_domains` is
ignored when `include_domains` is set, and the combined list is capped at 200
entries. `after_date` (YYYY-MM-DD) filters on publish date and is Search-only.

## Scope

- `source_policy` option validated with a shared normalizer (`tldts` JS,
  `psl` Rust) that returns typed 422-style errors for invalid entries.
- Pre-query: translate `include_domains` into `site:` operators for providers
  with operator support (Google, Bing, DuckDuckGo, Brave, Mojeek, Startpage)
  and `after_date` into date operators where available (Google `tbs=cdr`, Bing `freshness`).
- Post-merge filter applying the documented rules to every result; warnings
  when exclusions are ignored or a provider cannot honour the policy.
- `after_date` uses `publish_date` from WC-03; results without a date are kept
  and flagged with a warning (documented behaviour, configurable `strict`).

## Acceptance criteria

- Table-driven tests mirroring every example row of the parallel.ai source
  policy page (apex, www, path prefix, case sensitivity, `.gov`, invalid entries).
- 200-entry limit and include-overrides-exclude rule tested.

## References

- https://docs.parallel.ai/resources/source-policy
- https://docs.parallel.ai/search/source-policy
