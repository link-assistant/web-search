# CI findings and resolution

The first revision push was commit
`1ee7875a532d8c45880fdfcbb942200d260225b5`. Runs created on
2026-10-08 at 02:06:36 UTC carry that exact SHA, so these failures were from
the new revision rather than the previous passing documentation runs.

## Missing release records

The [JavaScript run](https://github.com/link-assistant/web-search/actions/runs/37716248370)
failed in its changeset check. Line 1732 of the downloaded full log
`ci-logs/javascript-37716248370.log` reports:

> No changeset found in this PR.

The detector explicitly treats `.github/workflows/parity.yml` as a workflow
change for both languages. Research and experiments alone are excluded, but
adding plan validation to the shared workflow activates release metadata and
language tests. The earlier local validator call fell back to the pre-existing
changeset, so it did not reproduce the PR-specific check. Running it with the
actual base/head SHA reproduced the failure.

The fix adds one JS patch changeset and one Rust patch changelog fragment.
Package versions are left to the existing automatic release tooling.
The [Rust changelog job](https://github.com/link-assistant/web-search/actions/runs/37716248396/job/113113546322)
also failed with the missing-fragment error at line 2991 of the full downloaded
Rust log, `ci-logs/rust-37716248396.log`.

## Pre-existing stale capture dependency

The [Rust dependency freshness job](https://github.com/link-assistant/web-search/actions/runs/37716248396/job/113113253567)
failed. Line 869 of its downloaded log
`ci-logs/rust-freshness-job-113113253567.log` reports:

> web-capture: resolved 0.3.37, latest 0.4.0

The same freshness script reproduced that failure locally. The dependency
was already pinned to 0.3.37 before this revision. The
[0.4.0 release](https://github.com/link-assistant/web-capture/releases/tag/rust-v0.4.0)
adds replay caching and refreshes upstream dependencies. Its existing search
contract remains the interface used by this repository.

The fix updates `rust/Cargo.toml` to 0.4.0 and regenerates the resolved graph
with `cargo update -p web-capture --precise 0.4.0`. Compatibility is checked
through the existing provider, caller-owned transport, cancellation, server,
and parity tests. No freshness exceptions or checker relaxations are added.

The first local upgrade build enabled the dependency's default `runtime`
feature and was killed while compiling `chromiumoxide_cdp` 0.9.1. Its compiler
alone used over 2 GiB within the workspace's approximately 3 GiB memory limit.
This repository uses only `SEARCH_PROVIDERS`, result types, and pure search
URL/parser helpers from web-capture. The dependency now explicitly enables
`search` with default features disabled, using its published minimal contract.
The unused browser/converter runtime is excluded; all existing web-search
features and caller-owned transport tests remain enabled.

## Reproduction and validation

From the repository root:

```bash
# Reproduce the original PR-specific missing changeset check.
GITHUB_BASE_SHA=27760060e6376d08b26a662f0aafad901561eed9 \
GITHUB_HEAD_SHA=1ee7875a532d8c45880fdfcbb942200d260225b5 \
node js/scripts/validate-changeset.mjs --js-root js

# Validate the current committed release records and dependency graph.
GITHUB_BASE_REF=main node js/scripts/validate-changeset.mjs --js-root js
GITHUB_BASE_REF=main rust-script rust/scripts/check-changelog-fragment.rs --rust-root rust
python3 rust/scripts/check-dependency-freshness.py --rust-root rust
cargo test --all-features --manifest-path rust/Cargo.toml
node js/scripts/check-js-rust-parity.mjs
```

Local outputs remain in `experiments/issue-29/logs/`, and downloaded CI logs
remain in `ci-logs/`; both are ignored. The PR description records final
validation and links the latest runs after the fix.
