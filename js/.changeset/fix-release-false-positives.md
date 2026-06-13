---
'@link-assistant/web-search': patch
---

Fix npm release CI/CD false positives: exit immediately with actionable guidance on permanent publish failures (E404/E401/E403) instead of retrying, add an optional `NPM_TOKEN` bootstrap fallback for the first publish of the package, make GitHub Release creation idempotent, and adopt the multi-strategy npm upgrade from the pipeline template.
