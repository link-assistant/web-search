---
id: WS-10
repo: link-assistant/web-search
title: Wire-compatible /v1/search, /v1/extract, error format, 422 validation, and generated OpenAPI 3.1
depends_on: [WS-02, WS-03, web-capture WC-02]
labels: enhancement
---

## Summary

To be a drop-in replacement, the HTTP server must accept parallel.ai's request
bodies and return its response and error shapes: `{type:"error",
error:{ref_id, message, detail?}}` with 401/402/422/429 semantics
(`resources_warnings-and-errors.md`), plus a machine-readable OpenAPI 3.1 spec
(`public-openapi.json`) and `llms.txt`.

## Scope

- Routes: `POST /v1/search`, `POST /v1/images/search` (WS-08), `POST /v1/extract`
  (proxy to web-capture WC-02 when configured), keeping `/search`, `/providers`,
  `/categories`, `/health`.
- Request validation with `zod` (JS) / `serde` + `validator` (Rust) producing 422
  bodies in parallel.ai's format; unknown-field tolerance documented.
- Optional `x-api-key` check (env `WEB_SEARCH_API_KEYS`), 401 on mismatch.
- `GET /openapi.json` generated from the schemas (`@asteasolutions/zod-to-openapi`,
  `utoipa`) and `GET /llms.txt`; diff test against parallel.ai's spec for the shared paths.

## Acceptance criteria

- Contract tests: the parallel.ai quickstart `curl` bodies succeed unchanged against the local server.
- Generated spec validates with an OpenAPI 3.1 validator in CI.

## References

- https://docs.parallel.ai/resources/warnings-and-errors
- https://docs.parallel.ai/openapi.json (local copy in data/parallel-docs/public-openapi.json)
