---
id: WS-05
repo: link-assistant/web-search
title: Optional local cross-encoder reranking with relevance scores
depends_on: [WS-04]
labels: enhancement
---

## Summary

parallel.ai ranks results against the objective inside its index; web-search
merges provider rankings ordinally (RRF) and has no relevance score. A local
reranker closes the quality gap without a hosted service.

## Scope

- `rerank: { model, topK }` option; default off so the base install stays small.
- JS: `@xenova/transformers` ONNX cross-encoders (`bge-reranker-base`,
  `mxbai-rerank-xsmall`); Rust: `fastembed` `TextRerank` via `ort`.
- Input text = title + snippet/excerpts; score written to `score`, order updated,
  `sources` preserved.
- Model cache directory option; graceful failure to the unreranked order with a warning.

## Acceptance criteria

- Test with a tiny deterministic model stub verifying reordering and score presence.
- Documented latency numbers from `experiments/`.

## References

- Candidate models: BAAI/bge-reranker-v2-m3, mixedbread-ai/mxbai-rerank-v2, Qwen3-Reranker
