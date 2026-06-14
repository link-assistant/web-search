---
'@link-assistant/web-search': patch
---

Make GitHub release tags short and self-heal stuck JavaScript releases (issue #17):

- Shorten the JavaScript release tag to `js-<version>` in multi-language repos and a bare `<version>` in single-language repos: no `v` prefix and a single `-` separator, replacing the previous `js_v<version>` spelling. `normalizeVersion` and `buildReleaseTag` accept every legacy spelling so existing releases keep resolving, and the shared `release-naming.mjs` helper centralizes the convention with unit tests.
- Self-heal a release that bumped `package.json` but never published: when `version-and-commit.mjs` finds nothing to commit (the changeset was already consumed by a prior failed run), it now signals `already_released` so the idempotent npm publish step still verifies the registry and publishes the missing version, instead of leaving it stuck on `main` (observed as `0.10.1`).
- Bump pinned GitHub Actions to versions whose runtime is Node 20+, clearing the deprecation warnings on the release workflows.
