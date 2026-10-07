# Issues 25–27: requirements, research, and implementation plan

## Scope and checklist

All work belongs to PR [#28](https://github.com/link-assistant/web-search/pull/28).
Issue #27 and both child issues, including all comments, were read on 2026-10-07.
Neither child issue has screenshots. No repository AGENTS.md or contributing
guide was found.

- [x] Read issues #25, #26, #27 and all issue/PR comments and reviews.
- [x] Confirm the prepared branch and inspect prior related PRs and release rules.
- [x] Research current releases and existing no_std/dependency-checking components.
- [x] Reproduce missing `merge` feature and inaccessible core registry before fixing.
- [x] Implement the core API and freshness policy; investigate the upstream duplicate limitation below.
- [x] Update lockfiles and prepare Rust and npm release fragments.
- [x] Run local CI checks before each useful implementation commit.
- [x] Fetch main and confirm it is already an ancestor; preserve history on the prepared branch.
- [x] Review the implementation diff for omissions and regressions.
- [ ] Update the PR title/body with reproduction, validation, API changes, and closing references.
- [ ] Check CI runs against the final SHA/timestamp; save and investigate any failed logs.
- [ ] Confirm a clean working tree and passing CI, then mark PR #28 ready.

## Complete requirements and proposed solutions

| Source      | Requirement                                                                                          | Alternatives and selected implementation plan                                                                                                                                                         | Verification                                                                                   |
| ----------- | ---------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| #27 R1      | Read and fully implement both listed issues and their comments.                                      | Trace all core/server call sites and JS/Rust parity, implementing both issues together.                                                                                                               | This matrix, PR diff review, complete local/CI checks.                                         |
| #27 R2      | One PR, no deferred child issue.                                                                     | Use existing PR #28 and its prepared branch for all commits.                                                                                                                                          | PR scope and closing references.                                                               |
| #27 R3–R4   | Close the parent and each child using one full closing keyword each.                                 | Include separate `Fixes #27`, `Fixes #25`, `Fixes #26` lines in the final PR body.                                                                                                                    | Read back final PR metadata.                                                                   |
| #27 R5      | Document already resolved/nonreproducible scope while retaining references.                          | Earlier PR #24 already isolated runtime dependencies, but it did not provide no_std or the registry; explain the remaining gap.                                                                       | Reproduction evidence and final PR body.                                                       |
| #25         | Split result types into no_std + alloc.                                                              | Use `alloc::String`/`Vec` and Serde's alloc support; retain serialization and the existing result structure.                                                                                          | A standalone no_std consumer and serialization tests.                                          |
| #25         | URL normalization without std.                                                                       | Prefer existing WHATWG `url` parsing with default features disabled over a custom parser; expose the shared normalization function.                                                                   | Existing normalization behavior plus edge cases and target build.                              |
| #25         | RRF (`rrf_k`) and every other merge strategy without std.                                            | Use alloc BTreeMap/BTreeSet internally; accept iterator-compatible provider collections so existing HashMap callers keep working.                                                                     | Three-provider fixture, weighted/interleave tests, identical server/core output.               |
| #25         | Provider registry as data: name, category, endpoint template, capabilities.                          | Extract a public static data catalog, independent of network factories, and have the server reuse it; preserve existing discovery metadata.                                                           | All 40 entries, defaults/categories/capabilities/endpoints, server factory parity.             |
| #25         | Consumers can list/select providers without copying metadata.                                        | Export registry discovery at the crate root and a public core module with the existing selection helpers.                                                                                             | no_std consumer calls discovery and category selection.                                        |
| #25         | `cargo build --no-default-features --features merge --target wasm32-unknown-unknown` succeeds.       | Add an additive `merge` feature and true no_std compilation when server is disabled; extend the existing boundary CI check.                                                                           | Exact command and a bare-metal target check (WASM alone can still support std).                |
| #25         | Same fused ranking as server from three lists.                                                       | One merger implementation shared by server and no_std consumers, with deterministic ordering.                                                                                                         | Regression fixture executed in both feature configurations.                                    |
| #26 R1      | Latest tower-http and all direct dependencies; adapt APIs and list handled breaking changes.         | Query authoritative package registries, update manifests to current stable releases, inspect upstream changelogs, test all enabled APIs.                                                              | Freshness check, full Rust/JS suites, consumer duplicate check.                                |
| #26 R2      | `cargo update`; dry run has no Updating lines.                                                       | Refresh the complete Rust lockfile after manifest changes and re-run update dry run.                                                                                                                  | Saved dry-run output.                                                                          |
| #26 R3      | CI fails for a stale direct dependency, except a manifest-line comment citing an open blocker issue. | Evaluate cargo-outdated/npm outdated; use registry-based checks if needed to validate exact latest stable versions and open-issue exceptions. Check Rust runtime/dev and JS runtime/dev dependencies. | Automated mocked freshness/exception/error tests plus live checks in CI.                       |
| #26 R4      | Release updated crates.io/npm packages.                                                              | Use existing merge-triggered release workflows and required changelog/changeset fragments; do not manually change versions prohibited by those workflows.                                             | Package dry runs and release-trigger review; publication occurs when maintainers merge PR #28. |
| #26 testing | cargo-outdated exits 0 and a fresh consumer has no duplicate tower-http versions.                    | Run the ready-made check where possible and a persisted consumer example depending on both web-search and current tower-http.                                                                         | cargo-outdated output and cargo tree inspection.                                               |

## Research and findings

### Existing implementation and reproduction

Merged [PR #24](https://github.com/link-assistant/web-search/pull/24) already made
runtime dependencies optional and left types/merging available without the
server feature. However, the crate still imported std collections, the registry
remained behind `server`, and the requested `merge` feature did not exist.
The exact WASM command failed with `the package 'web-search' does not contain
this feature: merge`. The new three-provider test, run before implementation,
also failed to compile because core registry exports were inaccessible and the
merger only accepted HashMap. Both failures were saved locally before fixing.
The dependency-freshness unit tests also failed before the policy scripts existed.

The consumer audit in [formal-ai #1182](https://github.com/link-assistant/formal-ai/issues/1182)
motivates exposing the shared catalog and algorithms. Consumer-specific fetching,
concurrency and input/output wrappers remain the consumer's responsibility; the
core now supplies the reusable result model, ranking and discovery data.

### Components considered and selected plans

- [Rust alloc collections](https://doc.rust-lang.org/stable/alloc/collections/index.html)
  provide BTreeMap/BTreeSet without another dependency. Selected for deterministic
  traversal and URL/source ordering. [hashbrown](https://github.com/rust-lang/hashbrown)
  is an existing no_std alternative when hash-table performance matters; it would
  add a dependency and still require explicit ordering for matching fused results.
- Reuse the existing [url parser](https://github.com/servo/rust-url) instead of a
  custom URL parser, preserving IDN, IPv6 and server deduplication behavior.
  The actual 2.5.8 manifest/source and bare-metal compilation establish that
  disabling default features is sufficient; its manifest has no `alloc` feature.
  Enable Serde's existing `derive` and `alloc` features with defaults disabled.
- Extract one static provider catalog and have all 32 server descriptors obtain
  their descriptive metadata from it. Keep all 40 provider factories, categories,
  defaults, custom transports, credentials and web-capture support intact.
  Endpoint tests compare every descriptor's default request against its template.
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated) is the ready-made Rust
  audit tool requested by the issue. The CI policy uses a small standard-library
  Python checker instead: Cargo metadata identifies all resolved direct optional,
  development and runtime dependencies, and the sparse registry identifies the
  newest non-yanked stable release outside or inside the manifest range. This
  also supports validating inline open-issue exceptions without installing an
  additional Rust dependency tree in every freshness job.
- [npm outdated](https://docs.npmjs.com/cli/v11/commands/npm-outdated/) supplies the
  JavaScript audit, wrapped to distinguish its stale-package exit from errors
  and verify open blocker issues. JSON cannot contain comments, so the documented
  `dependencyFreshnessExceptions` object is its per-dependency equivalent.
- [Dependabot](https://docs.github.com/en/code-security/concepts/supply-chain-security/dependabot-version-updates)
  can open future update PRs, but it does not replace the requested failing CI
  policy with verified issue exceptions. No additional bot configuration is needed
  for this change. Freshness jobs run on PRs, before releases, and weekly.

### Dependency versions and breaking changes

The [registry snapshot](research/dependency-versions.json), collected on
2026-10-07 by [the repeatable experiment](../../../experiments/dependency-versions.py),
records every direct Rust/npm dependency and the authoritative registry URL.
All direct manifests and installed lockfile versions now match those releases.
Lockfiles were refreshed fully, including npm transitives; npm audit reports no
vulnerabilities. Dependencies already current remain at their existing releases.

[tower-http 0.7 release notes](https://github.com/tower-rs/tower-http/blob/master/tower-http/CHANGELOG.md)
describe compression negotiation/quality changes, removed no-op Tokio and
async-compression features, non-exhaustive trace/classification types, and redirect
extension handling. This crate only enables `cors`, whose CorsLayer API remains
compatible: the existing server compiles and its tests pass with 0.7.1.
None of the changed compression/trace/redirect APIs are used by web-search.

[Changesets 3](https://github.com/changesets/changesets/releases) moves its tools to
ESM and raises their development Node/npm requirements. Its config schema is
updated; the existing `.mjs` release scripts already use the compatible entry
points. [lint-staged 17](https://github.com/lint-staged/lint-staged/releases)
also requires newer Node. All JavaScript workflow setup steps, including parity,
now use Node 24; the runtime package still supports Node 20. Prettier's refreshed
version reformats the existing declaration union without changing its types.

Core API additions are prepared as a Rust minor release. Server callers retain
HashMap weights and the existing merger/discovery module paths. Merger functions
now accept either HashMap or BTreeMap; ambiguous `collect()` call sites need an
explicit map type (the server's detailed-search path was updated). Without server,
weights use alloc BTreeMap. RegistryEntry gains endpoint/body/capability fields;
external struct literals must supply them. Merge ties, representative duplicates,
source lists and interleave provider order now have a deterministic order.
Scores, normalization rules and duplicate options preserve their existing meaning.

### Fresh-consumer duplicate investigation and publication boundary

The [persisted independent consumer](../../../experiments/tower-consumer) declares
both web-search and tower-http 0.7.1 and includes commands to reproduce the trees.
[The current-version tree](research/tower-http-current.txt) proves the consumer
and web-search share 0.7.1. However, the full server graph still includes 0.6.11:

```text
tower-http 0.6.11
├── reqwest 0.13.5 → web-search
├── web-capture 0.3.37 → web-search
└── reqwest 0.12.28 → web-capture / chromiumoxide / browser-commander
```

[The saved reverse tree](research/tower-http-upstream.txt) records all paths.
Published Reqwest 0.13.5 requires tower-http `0.6.8` unconditionally on native
targets for redirects. Published web-capture 0.3.37 declares `0.6` for its runtime
and brings Reqwest 0.12.28 through its browser stack. Cargo cannot unify these
with 0.7.1, and disabling Reqwest's default features cannot remove that mandatory
dependency. Removing web-capture would drop existing providers. A consumer-local
patch/fork would not propagate through a crates.io release. Reqwest's
[development manifest](https://github.com/seanmonstar/reqwest/blob/master/Cargo.toml)
already declares 0.7.1; a published compatible upstream release is needed.
Forcing Cargo to update the older copy to 0.7.1
[fails on Reqwest's `^0.6.8` requirement](research/tower-http-unification-rejected.txt).
The core-only fresh consumer has exactly one tower-http version and no duplicate
packages. The full-server **no-duplicates acceptance check remains blocked by
published upstream dependencies**, despite every direct dependency being current.

Rust's existing release workflow rejects manual PR version changes. A minor
changelog fragment prepares 0.6.0, and an npm patch changeset prepares 0.11.1.
Both normal releases and JavaScript instant releases are gated on dependency
freshness. Publication occurs through those existing workflows after maintainers
merge PR #28; neither merging main nor publishing a separate branch artifact is
part of this branch's authorized PR workflow. The publication requirement is
therefore prepared here and remains pending the merge.

### Verification

Local checks passed on 2026-10-07:

- Rust all-feature tests: 51 tests plus one doctest; core-only tests: 14 tests
  plus one doctest. The same three-provider ranking fixture runs in both builds.
- Exact merge-only WASM build, standalone WASM and bare-metal consumers,
  server/core dependency boundary check, rustfmt, and all-target/all-feature Clippy
  with warnings denied.
- Eight offline Rust freshness policy tests and the live registry check;
  `cargo outdated --root-deps-only --exit-code 1` exits 0.
- `cargo update --dry-run` locks zero packages. Its only Updating line is the
  registry-index refresh; there are no package Updating lines.
- All 165 JavaScript tests pass under Node, Bun and Deno; ESLint, script syntax,
  formatting and duplication checks pass. npm outdated/freshness is clean;
  npm audit reports zero vulnerabilities.
- JS/Rust catalog and repository layout parity passes. Changesets status/validation
  selects the expected npm patch release, with no manual package version changes.
- Rust file-size and crate-size checks pass; Cargo package contents include the
  new public core/registry and npm pack validates the published file list.
- Independent current-tower core consumer compiles/runs and `cargo tree -d`
  reports no duplicates. Default-server reverse trees and the failed forced update
  establish the upstream limitation above.

Large local logs are saved under ignored `ci-logs/`. In this container's ~3 GB
memory budget, concurrent tool/dependency builds caused a cargo-outdated build
and Clippy dependency compilation to be killed. The bounded cargo-outdated retry
disabled its install-only LTO and used two jobs; Clippy completed with one job
after the installer finished. No library features were removed to pass checks.
An overlapping Cargo package scan also saw a disappearing temporary archive;
repeating the scan after builds completed passed.

Final CI results are checked against the pushed SHA before marking PR #28 ready.
