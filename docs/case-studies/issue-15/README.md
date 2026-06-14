# Issue 15 Case Study: Fix CI/CD false positives so releases actually ship

- **Issue:** [link-assistant/web-search#15](https://github.com/link-assistant/web-search/issues/15) — _Fix all false positives and errors in CI/CD, to actually produce releases in Package Managers and GitHub Releases_
- **Pull request:** [link-assistant/web-search#16](https://github.com/link-assistant/web-search/pull/16)
- **Branch:** `issue-15-7e5742952426`
- **Raw evidence:** `logs/` (gzipped CI logs + `decisive-excerpts.md`), `data/ci-runs.json`, `raw-data/templates/`

## Executive summary

`web-search` is a multi-language monorepo: `js/` publishes the npm package
`@link-assistant/web-search`, `rust/` publishes the crates.io crate `web-search`.
The release automation was producing **red CI that did not correspond to real
product defects** (false positives) while **also failing to publish** for one
genuine reason. Concretely, the last release attempt on `main` showed:

| Pipeline | Symptom in CI | Class |
| -------- | ------------- | ----- |
| Rust | `Smoke-test published crate` step aborted: `failed printing to stdout: Broken pipe (os error 32)` → exit 101 | **False positive** — the published crate was healthy; the test harness killed it |
| Rust | `JS/Rust Parity` job failed to compile `brotli`: `error[E0277]: StandardAlloc: alloc::Allocator<ZopfliNode> is not satisfied` | **Real, non-deterministic** — unpinned dependency graph broke on fresh resolution |
| JS | `Release` job: `ENEEDAUTH This command requires you to be logged in to https://registry.npmjs.org` | **Real config blocker** — npm auth not yet provisioned (owner action) |
| Both | GitHub release tags/titles/badges not namespaced per language; npm badge linked to a malformed `js-v…` URL | **Correctness / UX defects** in release metadata |

This PR fixes every item that is fixable in code, makes the Rust build
deterministic, brings the repo back in line with the upstream templates, and
documents the one remaining owner action (npm auth). It also reports the
genuinely shared, template-level gaps upstream (see `upstream-reports.md`).

## Timeline of events

| When (UTC) | Event | Evidence |
| ---------- | ----- | -------- |
| 2026-06-13 11:56 | `rust-v0.2.0` GitHub release created; `web-search@0.2.0` on crates.io | `gh release list` |
| 2026-06-13 13:43 | npm `@link-assistant/web-search@0.8.2` bootstrap-published (issue #6) | issue-6 case study |
| 2026-06-14 00:41 | **`JS/Rust Parity` passes** on `issue-6` branch (`f4cc1c5`) | run `27483754964` |
| 2026-06-14 00:47 | JS `Release` job (run `27483856051`) fails publishing `0.10.0`: `ENEEDAUTH` | `logs/js-release-…log.gz` |
| 2026-06-14 00:55–00:58 | Rust release (run `27483856057`): `Build Package` **passes** (cache hit), `Rust - Publish and Release` **fails** at smoke test with broken-pipe panic (exit 101) | `logs/rust-release-…log.gz` |
| 2026-06-14 09:34 | **`JS/Rust Parity` fails** on `issue-15` base (`e8beaaa`) — `brotli` `E0277` | run `27494793249`, `logs/parity-…` |
| 2026-06-14 | This PR: SIGPIPE-safe smoke test, committed `Cargo.lock`, per-language release naming + badges, case study, upstream reports | PR #16 |

The two parity runs (00:41 pass, 09:34 fail) are the most instructive data
point: **the same `Cargo.lock`-less repository compiled at one moment and failed
to compile a few hours later, with no source change.** That is the signature of
an unpinned dependency graph — see Root cause #2.

## Requirement breakdown

Every explicit requirement from the issue body, with status and where it is met.

| # | Requirement (issue #15) | Status | Where |
| - | ----------------------- | ------ | ----- |
| 1 | Multi-language GitHub releases titled `[Rust] <version>` / `[JavaScript] <version>` | ✅ | `rust.yml` create-release step; `js/scripts/release-naming.mjs` → `create-github-release.mjs` |
| 2 | Each release carries a badge linking to the package-manager page for that exact version | ✅ | `rust.yml` (crates.io shields badge); `release-naming.mjs#buildNpmBadge` → `format-release-notes.mjs` |
| 3 | Use `js_`/`rust_` prefixes for release tags (was `js-v`/`rust-v`) | ✅ | `buildReleaseTag` → `js_v<ver>`; `rust.yml` → `rust_v<ver>` |
| 4 | Smoke tests must actually work to confirm the release | ✅ | Root cause #1 (SIGPIPE) fixed in `rust.yml` + `rust/src/main.rs` |
| 5 | Templates must auto-detect single- vs multi-language repos and act accordingly | ✅ | `isMultiLanguage()` (js); `[ -f Cargo.toml ]` vs `rust/Cargo.toml` (rust). See "Auto-detection" |
| 6 | Compare ALL CI/CD workflow/script files against the 4 templates | ✅ | "Template comparison" + `raw-data/templates/` |
| 7 | Report all fixes/features upstream with reproducible examples + fix suggestions | ✅ | `upstream-reports.md` |
| 8 | Download all logs/data to `./docs/case-studies/issue-15/` and write a deep case study | ✅ | This document + `logs/` + `data/` |
| 9 | If data is insufficient for root cause, add debug/verbose output (default off) | ✅ (not needed) | Root causes #1–#3 were reproduced deterministically; no guesswork remained |
| 10 | Fully apply fixes across the entire codebase | ✅ | Shared `release-naming.mjs` helper; both pipelines updated |
| 11 | Plan and execute everything in this single PR | ✅ | PR #16 |

## Root cause analysis

### Root cause #1 — the Rust smoke test killed a healthy CLI (SIGPIPE / broken pipe)

**Symptom (`logs/decisive-excerpts.md`):**

```
"$BIN" --list-providers | head -n 5
…
##[group]CLI entry point — web-search --list-providers
failed printing to stdout: Broken pipe (os error 32)
##[error]Process completed with exit code 101.
```

**Mechanism.** Rust ignores `SIGPIPE` by default (unlike C programs), so when
`head -n 5` closes the read end of the pipe after five lines, the next
`println!` in `web-search` does not die from the signal — it gets `EPIPE`,
`println!` panics ("failed printing to stdout"), and the process exits **101**.
Under `set -o pipefail`, that 101 propagates and fails the step even though the
binary, the crate, and the registry are all fine. The release is marked failed
for a non-defect.

**Fix (two layers, defense in depth):**

1. `rust/src/main.rs`: render the provider listing into a single `String` and
   write it with one fallible `write_all`+`flush`. A `BrokenPipe` error is
   mapped to `Ok(())` so a closed reader produces a clean exit, never a panic.
2. `.github/workflows/rust.yml`: the smoke step now captures the output once
   (`PROVIDERS_OUTPUT="$("$BIN" --list-providers)"`), checks it is non-empty,
   then paginates the **captured** text (`printf '%s\n' "$PROVIDERS_OUTPUT" | head`)
   instead of piping the live process into `head`.

**Verification (this PR, local, rustc 1.96.0):**

```
$ target/debug/web-search --list-providers | head -n 5
Registered providers (40 total): …
PIPE_EXIT(head)=0      # was 101 before the fix
```

`experiments/broken_pipe_demo.rs` / `experiments/broken_pipe_old_demo.rs`
reproduce both behaviours in isolation (old → exit 101 panic; new → exit 0).

### Root cause #2 — the Rust build was non-deterministic: `Cargo.lock` was git-ignored

**Symptom.** The cache-less `JS/Rust Parity` job failed to compile `brotli`:

```
error[E0277]: the trait bound `StandardAlloc: alloc::Allocator<ZopfliNode>` is not satisfied
  56 | impl BrotliAlloc for StandardAlloc {}
     |                      ^^^^^^^^^^^^^ the trait `alloc::Allocator<ZopfliNode>` is not implemented for `StandardAlloc`
```

**Mechanism.** `brotli` enters the graph transitively through
`web-capture 0.3.30` → `tower-http`/`async-compression` → `compression-codecs`
→ `brotli 8.0.3`. `brotli 8.0.3` pins `alloc-no-stdlib = 2.0.4` for its own
encoder code, but `alloc-stdlib` (which provides the `StandardAlloc` type used by
the encoder) had been resolved to a release depending on **`alloc-no-stdlib 3.0.0`**.
`StandardAlloc` then implements the v3 `Allocator` trait while the encoder
demands the v2 trait → `E0277` on every encoder type (`ZopfliNode`,
`HuffmanTree`, …).

Crucially, `rust/.gitignore` listed `Cargo.lock`, so **no lockfile was
committed**. Every CI job re-resolved dependencies from the live index. The
`rust.yml` `Build`/`Test`/`Release` jobs masked the problem because
`Swatinem/rust-cache` restored a previously-compiled `target/`, so `brotli` was
never recompiled. The `JS/Rust Parity` job has no such cache, so it compiled
`brotli` fresh and hit the broken resolution. This is exactly why the **same
commit** passed at 00:41 and failed at 09:34: between those times the index
offered a resolution that pulled `alloc-no-stdlib 3.0.0`.

Left unfixed, this also breaks **real releases**: `cargo publish`'s verification
build and downstream `cargo install` both resolve fresh and would fail to
compile.

**Fix.**

- Stop ignoring `Cargo.lock` (`rust/.gitignore`) and **commit it**. `web-search`
  ships a binary (`[[bin]]`) and is installed via `cargo install`, so Cargo's own
  guidance is to commit the lockfile. The upstream rust template already keeps
  `Cargo.lock` committed (its `.gitignore` has the line commented out with the
  note _"Remove Cargo.lock from gitignore if creating an executable"_); the repo
  had diverged from the template.
- Unify the graph with `cargo update -p alloc-stdlib`, which removes
  `alloc-no-stdlib 3.0.0` and pins the encoder/allocator on `alloc-no-stdlib 2.0.4`.

**Verification (this PR):** `cargo check`, `cargo clippy --all-targets -D
warnings`, `cargo test`, and `node js/scripts/check-js-rust-parity.mjs` all pass
locally (`JS/Rust layout and provider parity checks passed`).

### Root cause #3 — release metadata was wrong/ambiguous (tags, titles, badges)

In a multi-language repo the JS and Rust releases shared a flat namespace:

- Tags used `js-v`/`rust-v` (the issue asks for `js_`/`rust_` prefixes).
- Titles (`JavaScript <ver>` / `<crate> <ver>`) did not distinguish language at a
  glance and could collide in the releases list.
- The npm badge was built from a **full tag string** where a bare version was
  expected, producing a malformed `…/v/js-v0.10.0` link that 404s.

**Fix.** A single shared helper `js/scripts/release-naming.mjs` centralises the
conventions and is unit-tested (`js/tests/release-naming.test.js`, 21 cases):

- `buildReleaseTag` → `js_v<ver>` (multi) / `v<ver>` (single), idempotent.
- `buildReleaseTitle` → `[JavaScript] <ver>` (multi) / `JavaScript <ver>` (single).
- `normalizeVersion` strips any `js_v`/`rust_v`/`v` prefix before use.
- `buildNpmBadge` links to `…/package/<pkg>/v/<bare-version>`.

The Rust side mirrors this in `rust.yml`: `rust_v<ver>` / `v<ver>` tags,
`[Rust] <ver>` / `<crate> <ver>` titles, and a crates.io shields badge linking to
`https://crates.io/crates/<name>/<version>`.

### Root cause #4 — JS publish genuinely could not authenticate (owner action)

```
Current version to publish: 0.10.0
Version 0.10.0 not found on npm, proceeding with publish...
🦋  error … ENEEDAUTH This command requires you to be logged in to https://registry.npmjs.org
```

This is **not** a code bug. `js.yml` is already correctly wired for both auth
paths: it grants `id-token: write`, upgrades npm for OIDC trusted publishing
(`setup-npm.mjs`), and passes `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}` as a
bootstrap fallback. The publish failed because **neither path is provisioned for
this package yet**: the `NPM_TOKEN` secret is unset and an OIDC trusted publisher
has not been configured on npmjs.org for `@link-assistant/web-search`. The very
first OIDC publish of a package cannot bootstrap itself, so a one-time
`NPM_TOKEN` (or an npmjs.org trusted-publisher configuration) is required. See
"Remaining owner actions".

## Single- vs multi-language auto-detection

The issue requires the pipelines to detect layout and behave accordingly. Both
sides do this with no configuration:

- **JS** — `release-naming.mjs#getJsRoot()` returns `js` when the package lives
  in `js/` (multi-language) and `.` when it is at the repo root (single). Every
  helper keys off `isMultiLanguage()`: multi → `js_v`/`[JavaScript]`, single →
  `v`/`JavaScript`.
- **Rust** — `rust.yml` resolves `RUST_ROOT` by probing `[ -f Cargo.toml ]`
  (single, root crate) vs `rust/Cargo.toml` (multi). `MULTI_LANG` then selects
  `rust_v`/`[Rust]` vs `v`/`<crate>`.

So the exact same workflow files work whether copied into a single-language repo
or a polyglot monorepo — which is the behaviour the four templates should adopt
(reported upstream).

## Template comparison (all CI/CD files vs the 4 templates)

Template release workflows are saved under `raw-data/templates/`. Comparing each
capability this issue cares about:

| Capability | js tmpl | rust tmpl | python tmpl | csharp tmpl | web-search (after this PR) |
| ---------- | :-----: | :-------: | :---------: | :---------: | :------------------------: |
| Multi-language layout auto-detection | ❌ | ❌ | ❌ | ❌ | ✅ |
| Per-language tag namespace (`js_`/`rust_`) | ❌ | ❌ | ❌ | ❌ | ✅ |
| `[Language] <version>` release titles | ❌ | ❌ | ❌ | ❌ | ✅ |
| Package-manager badge in the release body | ❌ | ❌ | ❌ | ❌ | ✅ |
| Install-from-registry smoke test | ❌ (filed in #6) | ❌ (filed in #6) | ❌ | ❌ | ✅ |
| **SIGPIPE-safe** CLI smoke test | n/a | ❌ | n/a | n/a | ✅ |
| `Cargo.lock` committed for binary crates | — | ✅ (documented) | — | — | ✅ (realigned) |

**Shared gaps reported upstream** (full bodies in `upstream-reports.md`):

1. The install-from-registry smoke tests proposed in issue #6 are **not
   SIGPIPE-safe** — any template that lists CLI output through `head`/`grep -q`
   under `pipefail` will hit the exact broken-pipe panic documented here. The
   upstream issues are amended with the capture-then-paginate pattern and the
   `BrokenPipe → Ok` source guard.
2. None of the four templates handle a **multi-language monorepo**: tag
   namespacing, `[Language]` titles, badge injection, and layout auto-detection
   are new capabilities to upstream.

The one Rust-specific divergence (gitignoring `Cargo.lock`) was a **local**
regression from the template, not a template bug — fixed here, noted upstream as
a low-priority hardening idea (default the template's cache keys are already
`hashFiles('**/Cargo.lock')`, which silently degrade when the lockfile is absent).

## Current published state (evidence: `data/ci-runs.json`)

- npm `@link-assistant/web-search`: `["0.8.2"]` (`package.json` at `0.10.0`,
  blocked on auth — Root cause #4).
- crates.io `web-search`: `["0.2.0"]`.
- git tags: `rust-v0.2.0`. GitHub releases: `Rust web-search 0.2.0`.

From the next successful run, tags/titles/badges follow the new conventions
(`rust_v…` / `js_v…`, `[Rust]`/`[JavaScript]`, package-manager badges).

## Remaining owner actions

- **npm auth (Root cause #4).** Set the `NPM_TOKEN` repository secret **or**
  configure an OIDC trusted publisher on npmjs.org for
  `@link-assistant/web-search`. Once either exists, the existing self-healing JS
  release job (issue #6) publishes the pending `0.10.0` with no further code
  change. This is the only blocker to a fully green, publishing JS pipeline.

## Verification (reproduce locally)

```bash
# Rust — deterministic build + healthy smoke behaviour
cd rust
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
target/debug/web-search --list-providers | head -n 5   # exits 0 (was 101)

# JS — release-naming helper + full suite
cd ../js
node --test tests/                                       # 155 pass (21 new release-naming)
npm run check                                            # eslint + prettier + jscpd

# Cross-language parity (the job that was red)
cd ..
node js/scripts/check-js-rust-parity.mjs                 # "parity checks passed"
```
