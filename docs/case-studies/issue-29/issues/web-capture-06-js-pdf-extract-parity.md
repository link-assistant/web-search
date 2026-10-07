---
id: WC-06
repo: link-assistant/web-capture
title: Route extract through browser capture and PDF conversion when static HTML is insufficient
depends_on: [WC-02]
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
