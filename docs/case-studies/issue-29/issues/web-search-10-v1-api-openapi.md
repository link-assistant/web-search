---
id: WS-10
repo: link-assistant/web-search
title: Wire-compatible /v1/search, /v1/extract, error format, 422 validation, and generated OpenAPI 3.1
depends_on: [WS-02, WS-03, WS-04, WS-08, WC-11]
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
- https://docs.parallel.ai/public-openapi.json (local copy in data/parallel-docs/public-openapi.json)

## Implementation follow-through

1. Define shared schemas first and mount GA adapters without removing existing routes.
2. Advertise only implemented routes in OpenAPI, with image routing after WS-08 and Extract proxy against WC-11.

## Additional verification

- Positive/negative quickstart requests, unavailable capture backend, unknown fields, auth, 422/429, and JS/Rust parity.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
