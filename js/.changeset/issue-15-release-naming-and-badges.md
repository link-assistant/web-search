---
'@link-assistant/web-search': patch
---

Fix multi-language release metadata so JavaScript releases are unambiguous and verifiable (issue #15):

- Namespace the git tag as `js_v<version>` in multi-language repos (was `js-v<version>`) and keep a plain `v<version>` tag in single-language repos, auto-detected from the package layout.
- Title GitHub releases `[JavaScript] <version>` in multi-language repos so they no longer collide with `[Rust] <version>` releases in the shared list.
- Link the npm shields.io badge to the exact published version page (`/package/<pkg>/v/<version>`), fixing a normalization bug that previously produced a `js-v…` link.
- Centralize the tag/title/badge conventions in a shared `release-naming.mjs` helper with unit tests.
