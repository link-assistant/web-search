---
id: WC-01
repo: link-assistant/web-capture
title: Add an objective-aware excerpt ranking library (BM25 with optional embeddings)
depends_on: []
labels: enhancement
---

## Summary

parallel.ai Search and Extract return `excerpts[]` per result instead of raw
pages: passages chosen for the caller's `objective` and `search_queries`, capped
by `max_chars_per_result` and `max_chars_total`
(see `search_advanced-search-settings.md` and `extract_advanced-extract-settings.md`
in `docs/case-studies/issue-29/data/parallel-docs/` of web-search).
web-capture already converts pages to markdown but has no passage selection.
Add a pure function in JS and Rust that splits markdown into passages, scores
them with BM25 against objective + queries, optionally reranks with a local
embedding model, and returns excerpts within the character budgets.

## Scope

- `rankExcerpts(markdown, { objective, queries, maxCharsPerResult, maxTotalChars })`
  in JS and the equivalent in Rust, exported from the library entry points.
- Passage splitting on markdown headings/paragraphs with a sliding window for long blocks.
- BM25 scoring (`wink-bm25-text-search` or `minisearch` in JS; `bm25`/`tantivy` tokenizer in Rust).
- Optional embedding rerank behind a feature flag (`@huggingface/transformers` ONNX `bge-small-en-v1.5`; `fastembed` in Rust).
- Deterministic ordering by document position when scores tie; always return at least the lead passage.

## Acceptance criteria

- Unit tests with a fixture page: excerpts match the objective, respect both character caps, and never cut inside a word.
- Same inputs produce the same excerpts in JS and Rust (parity test fixture).
- No network access; model download only when the embedding feature is enabled.

## References

- parallel.ai excerpt settings: https://docs.parallel.ai/search/advanced-search-settings
- Case study: https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md
- Consumers: web-search WS-04 (excerpt orchestration), WC-02 (extract API), WS-18 (memory retrieval)

## Implementation follow-through

1. Add a pure passage scorer over existing conversion output with shared tokenization/scoring fixtures.
2. Compare BM25/lead/optional embeddings using WC-12 and define empty-document/no-objective/tiny-cap behavior.

## Additional verification

- Headings/tables/long passages, multilingual tokens, stable ties, Unicode/no-word-boundary cases, and exact budgets.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
