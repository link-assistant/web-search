---
id: WS-06
repo: link-assistant/web-search
title: Search modes turbo/fast/basic/advanced as configurable pipeline presets
depends_on: [WS-04, WS-05]
labels: enhancement
---

## Summary

parallel.ai exposes `mode` ∈ turbo (~200 ms), fast (~700 ms), basic (~1 s),
advanced (~3 s, default) trading latency for depth (`search_modes.md`), and
documents how to evaluate search tools with a fixed harness
(`search_evaluating-search.md`). Tavily and Exa use similar tiers, which the
parallel.ai migration guide maps onto these four names.

## Scope

- `mode` option as a named preset: `turbo` = cached results only, no fetch;
  `fast` = default providers + snippets + short excerpts; `basic` = fetch top
  pages + BM25 excerpts; `advanced` = fetch + rerank + larger excerpts.
- Presets are data (overridable) so operators can tune provider sets per mode.
- Evaluation and benchmark comparisons are tracked separately in WS-41.
- Default when omitted stays the current behaviour (`fast`), documented difference from parallel.ai.

## Acceptance criteria

- Unit tests that each preset resolves to the expected pipeline flags.
- Cache-miss policy and effective mode options have offline fixture tests.

## References

- https://docs.parallel.ai/search/modes
- https://docs.parallel.ai/search/evaluating-search

## Implementation follow-through

1. Resolve modes through shared configurable preset data and report effective options.
2. Define turbo cache-miss behavior explicitly and separate model selection from measured quality: WS-41 owns evaluation.

## Additional verification

- Cache hit/miss, override precedence, legacy versus GA defaults, invalid modes, and absent optional model.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
