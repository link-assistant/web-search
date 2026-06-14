# Case study — issue #17: short release tags and the missing releases

> **Issue:** [link-assistant/web-search#17](https://github.com/link-assistant/web-search/issues/17)
> "Make GitHub release tags in format `rust-0.0.1` and `js-0.0.1`, with no `v` and `-` to be short"
> **PR:** [#18](https://github.com/link-assistant/web-search/pull/18) — branch `issue-17-a319cc5b3fec`
> **Opened:** 2026-06-14 · **Author:** link-assistant team

This case study reconstructs the timeline, enumerates every requirement, finds
the root cause of each problem, and records the solution applied (or planned)
for each. Companion documents in this folder:

- [`template-comparison.md`](./template-comparison.md) — file-by-file comparison
  against the four link-foundation pipeline templates, best practices to adopt,
  and shared bugs to report upstream.
- [`logs/`](./logs/) — the raw GitHub Actions logs for the two runs the issue
  links to (`rust-publish-81270751769.log`, `js-release-81272171995.log`).
- [`template-rust-scripts/`](./template-rust-scripts/) — the upstream Rust
  release scripts used as the reference for the new auto-bump pipeline.

---

## 1. The symptom, in the reporter's words

> "Also we need to check CI/CD runs for all false positives and errors, and fix
> them all. **I still don't see all the releases in package managers and GitHub.**"

That single sentence is the whole bug. The release machinery *appeared* to run,
but the artifacts were not where they should be. Reconstructing the actual state
of every release channel on the day the issue was filed:

| Channel | Latest published | Version on `main` | Diagnosis |
|---|---|---|---|
| **npm** (`@link-assistant/web-search`) | `0.8.2` | `0.10.1` | `package.json` advanced four releases ahead of npm; `0.9.x`–`0.10.1` never published |
| **JS GitHub Releases** | *none* | — | no JavaScript release has **ever** been created |
| **crates.io** (`web-search`) | `0.2.0` | `0.2.0` | crate version never advanced — there is no bump step |
| **Rust GitHub Releases** | `0.2.0` (twice) | — | two tags for the same version: `rust-v0.2.0` and `rust_v0.2.0` |

So "I don't see all the releases" was literally true on three of the four
channels. Each has a distinct root cause, detailed below.

---

## 2. Requirements extracted from the issue

Every requirement in the issue body, itemised so each can be tracked:

| # | Requirement | Status |
|---|---|---|
| R1 | GitHub release tags must be **short**: `rust-0.0.1` / `js-0.0.1` — no `v` prefix, single `-` separator | ✅ done |
| R2 | Check CI/CD runs for **all false positives and errors** and fix them | ✅ done (smoke-test FP, stuck JS release, stuck Rust version) |
| R3 | Make the missing releases actually appear in package managers and on GitHub | ✅ fixed (self-heal + auto-bump); first run after merge produces them |
| R4 | Use best practices from the four link-foundation templates; compare the full file tree | ✅ [`template-comparison.md`](./template-comparison.md) |
| R5 | If the same issue exists in a template, **report it upstream** with repro + fix | ✅ planned reports in `template-comparison.md` §"Shared bugs"; see §8 |
| R6 | Download all logs/data to `docs/case-studies/issue-17/` and write a deep case study | ✅ this document + `logs/` + `template-rust-scripts/` |
| R7 | Reconstruct timeline, list requirements, find root causes, propose solutions/plans, check existing libraries | ✅ §3–§7 |
| R8 | If data is insufficient for root cause, **add debug output / verbose mode** | ✅ the release jobs already echo state; new bump step echoes fragment parse + bump decision |
| R9 | Apply fixes across the **entire codebase** (every place the bug occurs) | ✅ tag change applied in both JS scripts and Rust workflow; action bumps across all 3 workflows |
| R10 | Do everything in the **single PR #18** | ✅ all work on `issue-17-a319cc5b3fec` |

---

## 3. Timeline of events

Times are UTC. SHAs and run IDs are from the live repository.

- **2026-06-13 11:56** — Rust release `rust-v0.2.0` created (title "Rust web-search 0.2.0"). This is the older tag spelling.
- **2026-06-14 10:30:39** — PR #16 (issue #15: Deno `node:` import fix) merged to `main` at `f1075fe`. Two workflows trigger on the push:
  - **Run [27496039708](https://github.com/link-assistant/web-search/actions/runs/27496039708)** — *Rust CI* → **failure**.
  - **Run [27496039720](https://github.com/link-assistant/web-search/actions/runs/27496039720)** — *JavaScript Checks and Release* → **success** (but see below).
- **2026-06-14 10:40:51** — In run 708, the Rust publish job logs `web-search 0.2.0 is already published on crates.io` and `GitHub release ... already exists`. Everything is already shipped; the job is idempotent up to this point.
- **2026-06-14 10:40:52** — A *second* Rust release `rust_v0.2.0` (title "[Rust] 0.2.0") is created for the same `0.2.0`, now with the underscore tag — producing the duplicate.
- **2026-06-14 10:44:54** — Same job: the post-publish **library smoke test fails** with `error[E0277]: the trait bound StandardAlloc: alloc::Allocator<...> is not satisfied` (×many). The job goes red. **This is a false positive** — the crate was already published; the smoke test broke on a transitive dependency split, not on anything wrong with the release.
- **2026-06-14 10:40 (run 720)** — The JS release job reports success, but `JS - Instant Release` is **skipped** and no version is published. npm stays at `0.8.2` while `package.json` is `0.10.1`.
- **2026-06-14 11:02** — Issue #17 filed, linking the two runs above.
- **2026-06-14 (this PR)** — Root causes fixed; see §5.

---

## 4. Root cause analysis

### RC-1 — Tag format (R1)

Not a defect, a convention choice. The repo (and all four templates) produced
`rust_v<semver>` / would produce `js_v<semver>` (underscore + `v`). The issue
wants the shorter `rust-<semver>` / `js-<semver>`. Two code paths produce tags:
`js/scripts/release-naming.mjs` (`getTagPrefix`) and the inline tag construction
in `.github/workflows/rust.yml`. Both had to change.

### RC-2 — JS releases never published (R3, the npm `0.8.2` vs `0.10.1` gap)

`js/scripts/version-and-commit.mjs` bumps `package.json`, commits, and pushes.
The publish step downstream is **gated** on `version_committed == 'true'`. When
a prior run had *already* bumped and pushed the version commit but then died
before publishing (e.g. an OIDC/network hiccup in the publish step), the
changeset that drove the bump is already consumed. The next run finds **nothing
to commit**, sets `version_committed=false`, and the publish gate evaluates
false — so the already-bumped version is never published. The version is stuck
in limbo: present on `main`, absent from npm and GitHub Releases, with no
changeset left to retrigger it. That is exactly how `package.json` reached
`0.10.1` while npm stalled at `0.8.2`, and why **zero** JS GitHub releases exist.

### RC-3 — Rust crate version never advances (R3, crates.io stuck at `0.2.0`)

The Rust release job had **no version-bump step at all**. The JS side consumes
changesets to bump `package.json`; the Rust side accumulated `changelog.d/*.md`
fragments but nothing ever consumed them to rewrite `Cargo.toml`. Worse,
`rust/scripts/check-version-modification.rs` actively *forbids* hand-editing the
version in a PR. So the crate version was permanently pinned at its initial
`0.2.0`: `cargo publish` saw `0.2.0` already on crates.io every run and no-oped,
and no new tag/release was produced. This is the Rust analogue of the JS
changeset flow, simply missing.

### RC-4 — Rust publish job is a false positive (R2)

`RUSTFLAGS: -Dwarnings` is global, and the post-publish smoke test does a fresh
`cargo add web-search` resolve in a throwaway crate. A fresh resolve pulls
`alloc-no-stdlib` **3.0.0** (via `brotli-decompressor`) alongside **2.0.4**
(required by `brotli`). With both majors present, `StandardAlloc` from 2.0.4 no
longer satisfies `brotli`'s `Allocator` trait bound from 3.0.0 → `E0277`, ×many.
The committed `rust/Cargo.lock` pins the single working 2.0.4 line (fixed for
issue #15), but a *fresh* downstream resolve does not inherit that lock, so the
smoke test re-hit the split and turned a successful publish red.

### RC-5 — Duplicate Rust release tags (R2)

Two releases exist for `0.2.0`: `rust-v0.2.0` (2026-06-13) and `rust_v0.2.0`
(2026-06-14). The tag-construction logic changed between those two runs
(`rust-v` → `rust_v`), and because the idempotency check keyed on the *new* tag
name it did not see the *old* tag as "already released", so it created a second
release for the same version. Centralising the tag convention and keying
idempotency on the version (not the exact tag spelling) prevents recurrence.

---

## 5. Solutions applied in this PR

| RC | Fix | Files |
|---|---|---|
| RC-1 | `getTagPrefix()` → `js-` (multi) / `''` (single); Rust workflow → `rust-$VERSION` / `$VERSION`. `normalizeVersion()` still strips every legacy prefix (`v`, `js-v`, `js_v`, `rust-v`, `rust_v`) so old tags stay valid inputs. | `js/scripts/release-naming.mjs`, `.github/workflows/rust.yml`, + comment-only updates in `create-github-release.mjs`, `format-github-release.mjs`, `format-release-notes.mjs`; tests in `js/tests/release-naming.test.js` |
| RC-2 | When `version-and-commit.mjs` finds nothing to commit, it now also sets `already_released=true`. The publish gate accepts that flag, and `publish-to-npm.mjs` checks npm first and no-ops if genuinely published — so the pipeline self-heals a stuck version instead of skipping it forever. | `js/scripts/version-and-commit.mjs` |
| RC-3 | New `get-bump-type.rs` / `bump-version.rs` / `collect-changelog.rs` (mirroring the JS changeset flow and the link-foundation rust template), wired into the Rust release job: re-align to `origin/main` (concurrency-safe), pick the highest declared bump, rewrite `Cargo.toml`, fold fragments into `CHANGELOG.md`, `cargo update -p` the crate, commit `rust-<version>`, push, and target the release at the bump commit. | `rust/scripts/*.rs`, `.github/workflows/rust.yml` |
| RC-4 | The smoke test now runs `cargo update -p alloc-no-stdlib@3.0.0 --precise 2.0.4` on the fresh consumer resolve, collapsing the duplicate major to the single version `brotli` accepts. No-op once upstream realigns. | `.github/workflows/rust.yml` |
| RC-5 | Tag convention centralised; idempotency documented. Existing duplicate tags are left in place (deleting published releases is destructive and outward-facing — see §7). | — |
| R4/R9 | Action pins bumped to Node 20+ runtimes (`checkout@v6`, `setup-node@v6`, `cache@v5`, `create-pull-request@v8`) across `js.yml`, `rust.yml`, `parity.yml`. | all three workflows |

### What happens on the first push to `main` after merge

1. **Rust**: the bump step finds the issue-17 fragment(s), bumps `0.2.0` → `0.3.0`
   (minor), folds the changelog, commits `rust-0.3.0`, publishes the crate to
   crates.io, and creates GitHub release **`rust-0.3.0`** (short tag).
2. **JS**: the release job either consumes the pending changeset (normal path) or,
   if `0.10.1` is still uncommitted-in-limbo, self-heals — publishing `0.10.1` to
   npm and creating GitHub release **`js-0.10.1`** (short tag).

---

## 6. Existing components / libraries considered

Per R7, options surveyed before building bespoke scripts:

- **Changesets (`@changesets/cli`)** — already in use on the JS side; the bug was
  not Changesets itself but the gate logic around its "nothing to commit" state.
  Kept; added the self-heal signal around it rather than replacing it.
- **`cargo-release`** / **`release-plz`** — established crates for Rust release
  automation (version bump + changelog + tag + publish). Rejected for this repo:
  they assume their own tag/changelog conventions and would fight
  `check-version-modification.rs` and the existing `changelog.d/` fragment format.
  The link-foundation rust template already solved this with small `rust-script`
  helpers that match the repo's conventions exactly, so we ported that pattern
  (see `template-rust-scripts/`) instead of adopting a heavier tool.
- **`git-cliff`** — changelog generator from commit history. Rejected: the repo
  uses explicit `changelog.d/` fragments with `bump:` frontmatter (author-curated
  notes), which `git-cliff`'s commit-derived model does not preserve.
- **`semantic-release`** (JS) — rejected: the repo deliberately uses Changesets;
  switching release tooling is out of scope for a tag-format issue.

Conclusion: the smallest correct fix reuses the existing Changesets flow (JS) and
mirrors the existing template's `rust-script` helpers (Rust), keeping both
language pipelines symmetric and convention-compatible.

---

## 7. Tag cleanup (R2 / RC-5) — recommendation, not executed

There are two published releases for `0.2.0`: `rust-v0.2.0` and `rust_v0.2.0`.
Deleting a published GitHub release/tag is **destructive and outward-facing** —
anyone who pinned `rust_v0.2.0` would get a dangling reference — so this PR does
**not** delete them automatically. Recommended manual cleanup once confirmed safe:

```sh
# Keep the canonical short tag going forward (rust-0.3.0 will be created by CI).
# Optionally retire the redundant 0.2.0 duplicate (choose ONE to keep):
gh release delete rust_v0.2.0 --repo link-assistant/web-search --cleanup-tag
# (or rust-v0.2.0 — keep whichever you consider canonical)
```

Going forward the short-tag scheme yields exactly one tag per version
(`rust-<semver>`, `js-<semver>`), so the duplication cannot recur.

---

## 8. Upstream reports (R5)

The full list of defects shared with the templates — each with `file:line`, a
reproduction, and a suggested fix — is in
[`template-comparison.md` §"Shared bugs to report upstream"](./template-comparison.md).
Three were verified against the live `link-foundation/js-ai-driven-development-pipeline-template`
source and **filed upstream**:

1. **CHANGELOG `## 1.2` vs `## 1.2.3` prefix collision (JS)** — the release-notes
   extraction regex is escaped but unanchored, so extracting notes for `1.2`
   matches the `## 1.2.3` section. Fix: `## ${escapeRegex(version)}(?=\s|$)`.
   → [js-template#85](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/85)
2. **Changeset detection counts arbitrary `.md` files** — a stray
   `.changeset/NOTES.md` is treated as a pending changeset.
   → [js-template#86](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/86)
3. **`merge-changesets` silently drops malformed changesets** — a broken
   frontmatter file is skipped with only a warning, losing its notes/bump.
   → [js-template#87](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/87)

These are JS-template defects; `tpl-rust` is immune to (1) (bracket-delimited,
escaped, line-anchored regex). The Rust auto-bump scripts added here were ported
from `tpl-rust` and carry no new upstream-reportable defect.

---

## 9. Verification

- `npm test` (JS): **158/158 pass** including the updated `release-naming` tag tests.
- Rust scripts compiled clean under `RUSTFLAGS=-Dwarnings` and exercised locally:
  5 fragments → `bump_type=minor`; `0.2.0` → `0.3.0`; changelog folded; fragments
  consumed; `Cargo.lock` version synced via `cargo update -p`.
- All three workflow YAMLs parse (`yaml.safe_load`).
- Tag format proven by unit test: `buildReleaseTag('1.2.3', MULTI) === 'js-1.2.3'`,
  never contains `v` or `_`; `normalizeVersion` accepts all legacy spellings.
</content>
</invoke>
