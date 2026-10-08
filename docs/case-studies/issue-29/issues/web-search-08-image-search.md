---
id: WS-08
repo: link-assistant/web-search
title: Image search providers and /v1/images/search
depends_on: [WS-01, web-capture WC-07]
labels: enhancement
---

## Summary

parallel.ai Image Search returns up to 20 `{title, image_url, source_page_url,
width, height}` for keyword queries with an optional objective
(`image-search_image-search-quickstart.md`). web-search has no image category.

## Scope

- New provider category `images` with descriptors: Bing Images, DuckDuckGo
  Images, Brave Images, Wikimedia Commons API, Openverse API, Unsplash/Pexels
  (API key).
- Result shape as above; WC-07 fills missing dimensions when `probe: true`.
- `POST /v1/images/search` and `GET /images/search?q=`; CLI `web-search images`.

## Acceptance criteria

- Mocked-transport tests per provider; parity fixture.
- `--list-providers` shows the new category.

## References

- https://docs.parallel.ai/image-search/image-search-quickstart
- Openverse API: https://api.openverse.org/v1/

## Implementation follow-through

1. Extend current registry/merger with typed image descriptors, adding adapters incrementally from fixtures.
2. Preserve supplied attribution/license and keep dimension probes optional and policy/byte bounded via WC-10.

## Additional verification

- Missing dimensions/source pages, lazy/relative URLs, duplicates, provider failure, and keyword/objective modes.
- Use mocks/local servers with finite test deadlines, update types and examples, and keep live comparisons opt-in.

## Planning references

- [Case study and verified source snapshot](https://github.com/link-assistant/web-search/blob/issue-29-03fe983c7d11/docs/case-studies/issue-29/README.md).
- Original requirement: https://github.com/link-assistant/web-search/issues/29
