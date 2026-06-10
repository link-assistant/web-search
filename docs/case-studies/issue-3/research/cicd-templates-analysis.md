# Research: CI/CD pipeline templates comparison

> Source: direct clone and inspection of the four `link-foundation` AI-driven development
> pipeline templates, performed while researching issue #3. Nothing in those repos was
> modified.

Templates compared:

- `link-foundation/js-ai-driven-development-pipeline-template`
- `link-foundation/rust-ai-driven-development-pipeline-template`
- `link-foundation/python-ai-driven-development-pipeline-template`
- `link-foundation/csharp-ai-driven-development-pipeline-template`

Since this repo is a **JS + Rust** project, the JS and Rust templates are the primary
references; Python and C# are used to confirm cross-cutting practices.

## JavaScript template — best practices

`.github/workflows/`: `release.yml` (checks + publish), `example-app.yml`
(build/Pages/preview), `links.yml` (broken-link check + Wayback fallback), plus
`.github/actions/publish-dockerhub/`.

1. **Multi-runtime test matrix** — `runtime: [node, bun, deno] × os: [ubuntu, macos,
windows]` (9 cells, `fail-fast: false`).
2. **Fast-fail job ordering** — cheap jobs gate `test`/`release` via `needs` +
   `!cancelled()` + `result == 'success' || 'skipped'`.
3. **Fresh-merge simulation** (`scripts/simulate-fresh-merge.sh`) — validates the _actual_
   merge result, not a stale GitHub merge preview. The single most distinctive practice.
4. **Manual-version-edit guard** (`check-version.mjs`) — versions only change via CI.
5. **Code-duplication check** (`jscpd` via `.jscpd.json`) and **secret scanning**
   (`secretlint`).
6. **npm OIDC trusted publishing** (`id-token: write`, `setup-npm.mjs`) — no long-lived
   tokens. Constraint: only ONE workflow file may be the registered trusted publisher,
   forcing all publish paths into `release.yml`.
7. **`max-lines: 1500`** enforced two ways (ESLint rule + line-limit script) to keep files
   AI-context-friendly.
8. **Self-healing release** (`check-release-needed.mjs`) — republishes an unpublished
   version even without a new changeset.
9. **Broken-link CI with Wayback fallback** (`links.yml` + `check-web-archive.mjs`).
10. **Husky + lint-staged** pre-commit.

## Rust template — best practices

`.github/workflows/release.yml` is the single workflow (checks + publish + docs).

1. **rustfmt + clippy in CI** — `dtolnay/rust-toolchain@stable` with
   `components: rustfmt, clippy`; `cargo fmt --all -- --check` and
   `cargo clippy --all-targets --all-features`.
2. **Warnings-as-errors** — `RUSTFLAGS: -Dwarnings` at workflow env level.
3. **Strict lint config in `Cargo.toml`** (not separate files):
   `[lints.rust] unsafe_code = "forbid"`, `[lints.clippy] all/pedantic/nursery = warn`
   with a small curated allow-list (`module_name_repetitions`, `too_many_lines`,
   `missing_errors_doc`, `missing_panics_doc`).
4. **Cargo caching** — `actions/cache@v5` over `~/.cargo/registry`, `~/.cargo/git`,
   `target`, keyed on `hashFiles('**/Cargo.lock')`; separate keys per job.
5. **Cross-OS test matrix** — ubuntu/macos/windows, plus **doc tests**
   (`cargo test --doc`) and **coverage** via `cargo-llvm-cov` → Codecov.
6. **cargo publish automation** — fragment-driven SemVer bump, `publish-crate.rs`,
   `wait-for-crate.rs` polls the crates.io index before the GitHub release.
7. **Crate-size guard** (`check-crate-size.rs`) + tight `include = [...]` allowlist to stay
   under crates.io's 10 MiB limit.
8. **Optimized release profile** — `lto = true`, `codegen-units = 1`, `strip = true`;
   MSRV pinned via `rust-version`.
9. **`cargo doc` → GitHub Pages**, deployed independently of publish.
10. **CI logic is unit-tested** (`tests/unit/ci-cd/`).

## Cross-cutting practices (all four templates)

- One consolidated `release.yml`: detect-changes → checks → build → publish → GH release,
  plus `workflow_dispatch` manual paths (instant + changeset/changelog-PR).
- `detect-changes` gating job emitting per-file-type booleans.
- Concurrency `group: workflow-ref`, cancel-in-progress for PRs / false for main.
- Changeset/changelog-fragment discipline (changesets for JS/C#, scriv for Python,
  `changelog.d/` for Rust); manual version edits forbidden; self-healing publish.
- Per-job `timeout-minutes`; `fetch-depth: 0` for base-branch diffs.
- File-size/line-limit check (1500-line "AI-context" ceiling) in all four.
- `pre-commit` hooks (framework for Rust/Python/C#, husky+lint-staged for JS).
- GitHub Pages docs deploy via `configure-pages` + `upload-pages-artifact` +
  `deploy-pages`.

## Gap noted in ALL four templates

None of the four ship `dependabot.yml`, `CODEOWNERS`, issue/PR templates, `SECURITY.md`,
or `FUNDING.yml`. Each `.github/` is workflows-only (JS adds `actions/`). A project
adopting these templates should add community-health files itself.

## This repository vs. the templates

This repo (`link-assistant/web-search`) already adopts most of the **JS** template via
`.github/workflows/release.yml`: `detect-changes`, `version-check`, `changeset-check`,
fresh-merge simulation, lint with `jscpd`, multi-runtime matrix (node/bun/deno × 3 OS),
and npm OIDC trusted publishing. Its scripts mirror the template's `scripts/*.mjs`.

The **Rust** side (`.github/workflows/rust.yml`) is leaner than the Rust template. It has
`RUSTFLAGS: -Dwarnings`, fmt + clippy, a 3-OS test matrix, doc tests, cargo caching, and
`cargo build --release` + `cargo package --list`. Compared with the template it is missing
(documented as follow-ups in the case study `README.md`):

- `[lints]` block in `Cargo.toml` → **partially adopted in this PR** (`unsafe_code =
"forbid"`; full clippy pedantic/nursery deferred because it currently produces 73
  warnings that would fail `-Dwarnings`).
- Optimized release profile → **adopted in this PR** (`lto`/`codegen-units`/`strip`).
- crates.io publish automation, coverage (`cargo-llvm-cov` → Codecov), crate-size guard,
  and `cargo doc` → Pages → **deferred** (require new secrets/scripts; tracked as
  follow-ups).
