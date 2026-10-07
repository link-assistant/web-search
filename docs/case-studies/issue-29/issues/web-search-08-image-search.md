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
