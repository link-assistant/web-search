---
id: WC-07
repo: link-assistant/web-capture
title: Extract image candidates with dimensions and source page for image search
depends_on: []
labels: enhancement
---

## Summary

parallel.ai Image Search returns `{title, image_url, source_page_url, width,
height}` (`image-search_image-search-quickstart.md`). web-search will add image
providers (WS-08) but needs a capture helper that lists images on a page with
dimensions.

## Scope

- `extractImages(html, baseUrl)` returning `{image_url, alt, width, height, source_page_url}` from `<img>`, `srcset`, `og:image`, `twitter:image`, JSON-LD `image`.
- When dimensions are absent, probe the first bytes of the image (`image-size` JS, `imagesize` Rust) with a size cap.
- Optional `/images?url=` route.

## Acceptance criteria

- Fixture tests for each source; probe test with a local PNG/JPEG/WebP.

## References

- https://docs.parallel.ai/image-search/image-search-quickstart
- Consumer: web-search WS-08

## Implementation follow-through

1. Add pure HTML image extraction plus optional bounded header probes, separate from screenshots.
2. Resolve base/relative/lazy URLs and preserve source/alt/attribution without fetching whole images.

## Additional verification

- srcset/metadata/duplicates/data URLs, corrupt header, missing size, redirect, and probe budget.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
