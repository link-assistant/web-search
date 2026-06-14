---
bump: patch
---

### Fixed

- Commit `Cargo.lock` (it was `.gitignore`d) so the binary crate builds from a pinned, reproducible dependency graph. With the lockfile unpinned, a freshly published `alloc-no-stdlib 3.0.0` split the `brotli` encoder's allocator types (`brotli` resolved `alloc-no-stdlib 2.0.4` for its own code while `alloc-stdlib` pulled `3.0.0`), producing `error[E0277]: StandardAlloc: alloc::Allocator<ZopfliNode> is not satisfied` on fresh CI resolution. The same commit built one moment and failed the next with no source change; the committed, unified-on-`2.0.4` lockfile makes the Rust build deterministic again (issue #15).
- Stop the published-crate smoke test from failing on a healthy CLI: it piped `web-search --list-providers` straight into `head`, which closed the pipe and made the binary abort with `failed printing to stdout: Broken pipe` (exit 101) under `set -o pipefail`. The step now captures the output once and paginates the captured text, removing the false-positive release failure (issue #15).
- Make `web-search --list-providers` itself broken-pipe-safe: the listing is rendered into a single buffer and written in one fallible call so a closed reader (`| head`) results in a clean exit instead of a panic.

### Changed

- Auto-detect single-language vs multi-language repository layout for the Rust release. Multi-language repos now tag releases `rust_v<version>` (was `rust-v<version>`) and title them `[Rust] <version>`; single-language repos use a plain `v<version>` tag and `<crate> <version>` title.
- Add a crates.io shields.io badge to the GitHub release body that links to the exact published version's crate page.
