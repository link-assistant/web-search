# Issue 7 Case Study: JavaScript and Rust Workflow Support

## Requirement Breakdown

| Requirement from issue                                              | Implementation                                                                                                                                                                     |
| ------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Use repository organization similar to `link-assistant/web-capture` | JavaScript package files now live under `js/`; Rust remains under `rust/`; root keeps shared docs, scripts, and repository metadata.                                               |
| Separate `js.yml`, `rust.yml`, and folders `js`, `rust`             | Renamed the JS release/check workflow to `.github/workflows/js.yml`, kept `.github/workflows/rust.yml`, and added `.github/workflows/parity.yml`.                                  |
| No language-specific files in the root                              | Moved JS source, tests, examples, package metadata, formatter/linter configs, Deno/Bun configs, changesets, and Husky hook under `js/`.                                            |
| Full JavaScript/Rust parity                                         | Added `scripts/check-js-rust-parity.mjs` and a parity workflow that verifies the layout and compares the live JS and Rust provider catalogs.                                       |
| Rust tests only in `./rust/tests`                                   | Added `js/tests/repository-layout.test.js` and parity checks that fail if Rust test files appear under `rust/src`.                                                                 |
| Reuse CI/CD template best practices                                 | Kept changeset validation, version-change protection, npm trusted publishing, Rust fmt/clippy/test gates, code duplication checks, and added explicit JS/Rust workflow separation. |
| Compile issue data and analysis in `docs/case-studies/issue-7`      | Raw issue/PR metadata is stored in `data/`; existing comparison notes are retained, and this README records the applied solution.                                                  |

## Data Collected

- `data/issue-7.json`: issue body, labels, comments, and metadata.
- `data/pr-8.json`: prepared PR metadata.
- `current-repository-analysis.json`: current repository layout and CI summary.
- `effect-template-analysis.json`: prior template comparison data.
- `BEST-PRACTICES-COMPARISON.md`: JavaScript tooling and workflow comparison.
- `FORMATTER-COMPARISON.md`: formatter tradeoff analysis.

## Solution Plan Executed

1. Added a failing layout regression test proving JS package files were still at the root.
2. Moved JS package files into `js/` while leaving Rust in `rust/`.
3. Updated JS package scripts, release helpers, and changeset helpers for `js/` package-root operation.
4. Split the JS workflow into `.github/workflows/js.yml` and added a parity workflow.
5. Added a changeset for the package-level workflow/layout change.
6. Updated README development commands for the new layout.

## Verification

The intended local verification set is:

```bash
cd js
npm install
npm test
npm run check
cd ..
node scripts/check-js-rust-parity.mjs
cd rust
cargo fmt --all -- --check
RUSTFLAGS="-Dwarnings" cargo clippy --all-targets --all-features
cargo test --all-features
cargo test --doc
```
