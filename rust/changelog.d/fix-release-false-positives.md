---
bump: patch
---

### Fixed

- Send a descriptive `User-Agent` on the crates.io pre-flight existence check so already-published versions are detected (crates.io returns HTTP 403 without one), eliminating false-positive republish attempts on no-op pushes.
- Read the crates.io token from `CARGO_REGISTRY_TOKEN` with a `CARGO_TOKEN` fallback, and delegate publishing to `publish-crate.rs` with failure classification and deferred rate-limit handling.
- Make GitHub Release creation idempotent so re-runs over an existing tag stay green.
