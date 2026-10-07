---
id: WS-06
repo: link-assistant/web-search
title: Search modes turbo/fast/basic/advanced as presets plus an evaluation harness
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
- `experiments/search-eval/` harness: gold query set, per-mode latency and
  hit-rate metrics, comparison against parallel.ai/Tavily/Exa when API keys are present.
- Default when omitted stays the current behaviour (`fast`), documented difference from parallel.ai.

## Acceptance criteria

- Unit tests that each preset resolves to the expected pipeline flags.
- Harness runs offline on recorded fixtures in CI.

## References

- https://docs.parallel.ai/search/modes
- https://docs.parallel.ai/search/evaluating-search
