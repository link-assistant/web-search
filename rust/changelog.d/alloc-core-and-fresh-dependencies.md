---
bump: minor
---

### Added
- Provide a `no_std` + `alloc` merge feature, shared URL normalization and all three merge strategies for WASM and embedded consumers.
- Export the complete static provider catalog, endpoint/body templates, capabilities, and discovery helpers without server dependencies.
- Check all Rust direct dependencies against their latest stable release in pull requests, before releases, and weekly; allow only an inline comment citing an open blocker issue.

### Changed
- Make merge ordering, score ties, and source lists deterministic across HashMap and BTreeMap callers while preserving the default server API.
- Upgrade tower-http to 0.7.1, web-capture to 0.3.37, and refresh every direct dependency and the complete lockfile.
