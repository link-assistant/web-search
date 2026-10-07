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
