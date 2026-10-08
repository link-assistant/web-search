---
id: WC-06
repo: link-assistant/web-capture
title: Route extract through browser capture and PDF conversion when static HTML is insufficient
depends_on: [WC-02, WC-10]
labels: enhancement
---

## Summary

parallel.ai Extract advertises support for JavaScript-rendered pages and PDFs
with markdown output (`extract_extract-quickstart.md`). web-capture has browser
capture (browser-commander) and PDF conversion, but the WC-02 batch path needs
an automatic fallback policy.

## Scope

- Heuristic: when the static fetch yields < N visible characters or a known
  SPA shell, retry with the browser capture (bounded per batch).
- Content-type routing: `application/pdf` → PDF module → markdown; DOCX likewise.
- Per-URL `error_type: unsupported_content` for binaries without a converter.
- Size and time limits documented; receipts record which path was used.

## Acceptance criteria

- Fixture SPA page and fixture PDF both return markdown through `/extract`.
- Fallback is skipped when `fetch_policy.timeout_seconds` is exhausted.

## References

- https://docs.parallel.ai/extract/extract-quickstart

## Implementation follow-through

1. Route by type/visible-content heuristics into existing browser/PDF modules and record the path.
2. One end-to-end deadline and WC-10 limits cover static/browser/converter fallback and cleanup.

## Additional verification

- SPA/static/PDF, wrong type, encrypted/malformed PDF, finite oversized input, browser failure, and cancellation.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
