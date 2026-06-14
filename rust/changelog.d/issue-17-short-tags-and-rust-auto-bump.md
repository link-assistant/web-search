---
bump: minor
---

### Added

- Auto-bump the crate version from `changelog.d/` fragments at release time
  (issue #17). The Rust release job previously had no version-bump step, so the
  crate version never advanced past its initial value and no new crates.io or
  GitHub releases were ever produced. New `get-bump-type.rs`, `bump-version.rs`
  and `collect-changelog.rs` scripts (mirroring the JavaScript changeset flow and
  the link-foundation rust template) now read the fragments, pick the highest
  declared bump, rewrite `Cargo.toml`, fold the fragments into `CHANGELOG.md`, and
  commit the result back to `main` before publishing — the same pattern the
  JavaScript side already uses.

### Changed

- Shorten the Rust release tag to `rust-<version>` in multi-language repos and a
  bare `<version>` in single-language repos (issue #17): no `v` prefix and a
  single `-` separator, replacing the previous `rust_v<version>` / `rust-v<version>`
  spellings. `[Rust] <version>` release titles and the crates.io badge are
  unchanged.

### Fixed

- Unify the duplicated `alloc-no-stdlib` major version in the published-crate
  library smoke test (issue #17). A fresh `cargo add web-search` resolve pulled
  both `alloc-no-stdlib` 2.0.4 (required by `brotli`) and 3.0.0 (pulled by
  `brotli-decompressor`); with both majors present, `StandardAlloc` from 2.0.4 no
  longer satisfied `brotli`'s `Allocator` bound from 3.0.0, so the downstream
  consumer failed to compile with `error[E0277]`. The smoke test now collapses the
  3.0.0 instance to 2.0.4 (a no-op once upstream realigns), proving the published
  crate is installable from a clean resolve.
