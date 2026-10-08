---
id: WS-04
repo: link-assistant/web-search
title: Objective-focused excerpts with max_chars_per_result and max_chars_total
depends_on: [WS-01, web-capture WC-01, web-capture WC-02]
labels: enhancement
---

## Summary

parallel.ai's main differentiator is returning token-efficient `excerpts[]`
per result selected for the objective, bounded by
`excerpt_settings.max_chars_per_result` and `max_chars_total`
(`search_best-practices.md`, `search_advanced-search-settings.md`). web-search
returns provider snippets only.

## Scope

- After merging, fetch the top `max_results` pages through the injected
  transport (or web-capture WC-02 `/extract` when a `webCapture` base URL is
  configured) with a bounded pool (`p-limit`, `tokio::sync::Semaphore`).
- Rank passages with WC-01 using `objective` + `search_queries`; enforce both
  caps; fall back to the provider snippet when fetch fails and add a warning.
- Options: `excerpt_settings{max_chars_per_result}`, `max_chars_total`,
  `excerpts: false` to keep today's fast path.
- Receipts record fetch outcome per result.

## Acceptance criteria

- Mocked-transport test: excerpts obey caps; a failing URL falls back to the snippet.
- Benchmark script in `experiments/` comparing excerpt vs snippet token counts.

## References

- https://docs.parallel.ai/search/best-practices

## Implementation follow-through

1. Reuse capture batch/ranker interfaces after merge with optional bounded fetching.
2. Allocate budgets deterministically across deduplicated results and preserve original sources/fetch receipts.

## Additional verification

- Zero/tiny caps, Unicode, duplicate URLs, mixed failures, cancellation, and fallback snippets within the total cap.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
