---
id: WC-05
repo: link-assistant/web-capture
title: Document crawler identity and add robots.txt and per-host politeness controls
depends_on: []
labels: enhancement, documentation
---

## Summary

parallel.ai publishes its crawler user agent (`ShapBot`), an IP list
(`shapbot.json`), and robots handling so site owners can allow it
(`resources_crawler.md`). web-capture fetches with an undocumented user agent
and no robots.txt or rate-limit handling, which hurts both etiquette and
success rates on well-behaved sites.

## Scope

- Default user agent `Mozilla/5.0 (compatible; LinkAssistantBot/<version>; +https://github.com/link-assistant/web-capture)`, overridable per request.
- robots.txt fetch + cache with `allow`/`disallow` evaluation (`robots-parser` JS, `texting_robots` Rust); `respect_robots` option, default on, with a documented opt-out for the user's own sites.
- Per-host concurrency (default 2) and minimum delay (default 500 ms) via the transport wrapper (`bottleneck` JS, `governor` Rust).
- `docs/crawler.md` and a `crawler.json` template for operators who run a public instance.

## Acceptance criteria

- Tests: disallowed path is skipped with a typed error; per-host limiter serializes requests.
- README section "Crawler identity".

## References

- https://docs.parallel.ai/resources/crawler
- Consumers: WC-02 extract, web-search WS-04

## Implementation follow-through

1. Reuse cached robots matching and per-origin scheduling with documented versioned user agent.
2. Publish operator IP-list templates instead of asserting project-wide static IPs, and document unavailable robots/legacy behavior.

## Additional verification

- Allow/disallow precedence, agent groups, robots refresh/redirects, host delay/concurrency, and explicit opt-out.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
