---
bump: patch
---

### Fixed

- Stop the published-crate smoke test from failing on a healthy CLI: it piped `web-search --list-providers` straight into `head`, which closed the pipe and made the binary abort with `failed printing to stdout: Broken pipe` (exit 101) under `set -o pipefail`. The step now captures the output once and paginates the captured text, removing the false-positive release failure (issue #15).
- Make `web-search --list-providers` itself broken-pipe-safe: the listing is rendered into a single buffer and written in one fallible call so a closed reader (`| head`) results in a clean exit instead of a panic.

### Changed

- Auto-detect single-language vs multi-language repository layout for the Rust release. Multi-language repos now tag releases `rust_v<version>` (was `rust-v<version>`) and title them `[Rust] <version>`; single-language repos use a plain `v<version>` tag and `<crate> <version>` title.
- Add a crates.io shields.io badge to the GitHub release body that links to the exact published version's crate page.
