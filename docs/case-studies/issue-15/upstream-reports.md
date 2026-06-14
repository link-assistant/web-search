# Upstream reports for issue #15

The issue requires that every fix and feature discovered here be reported to the
four AI-driven-development pipeline templates, with reproducible examples,
workarounds, and concrete fix suggestions:

- [`link-foundation/js-ai-driven-development-pipeline-template`](https://github.com/link-foundation/js-ai-driven-development-pipeline-template)
- [`link-foundation/rust-ai-driven-development-pipeline-template`](https://github.com/link-foundation/rust-ai-driven-development-pipeline-template)
- [`link-foundation/python-ai-driven-development-pipeline-template`](https://github.com/link-foundation/python-ai-driven-development-pipeline-template)
- [`link-foundation/csharp-ai-driven-development-pipeline-template`](https://github.com/link-foundation/csharp-ai-driven-development-pipeline-template)

This file holds the ready-to-file issue bodies. Each is scoped to the templates
that genuinely share the gap, so we do not file noise.

---

## Report A — install-from-registry smoke tests must be SIGPIPE-safe (amends issue #6's smoke-test reports)

**Where to file / amend:** the smoke-test issues already opened from issue #6 —
js [#81](https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/81),
rust [#73](https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/73),
python [#20](https://github.com/link-foundation/python-ai-driven-development-pipeline-template/issues/20),
csharp [#27](https://github.com/link-foundation/csharp-ai-driven-development-pipeline-template/issues/27).
Most relevant to **rust** (compiled CLI) but applies to any template whose smoke
test pipes CLI output through `head`/`tail`/`grep -q`/`| true`.

**Title:** Smoke test piping CLI output into `head` causes false-positive release failures (SIGPIPE/broken pipe)

**Body:**

> The install-from-registry smoke test proposed in the issue-6 reports verifies a
> freshly published CLI by listing its output and trimming it, e.g.:
>
> ```bash
> set -euo pipefail
> web-search --list-providers | head -n 5
> ```
>
> This aborts a **healthy** release. A consumer that closes the pipe early
> (`head` exits after N lines) leaves the producer writing to a closed fd. The
> failure mode differs by runtime but the outcome is the same — a non-zero exit
> under `set -o pipefail` that fails the release for no real defect:
>
> - **Rust** ignores `SIGPIPE` by default, so `println!` does not die from the
>   signal; it gets `EPIPE`, panics with `failed printing to stdout: Broken pipe
>   (os error 32)`, and exits **101**.
> - **Node**, **Python**, **.NET** can raise `EPIPE`/`BrokenPipeError`/
>   `IOException` for the same reason if the program writes more than the reader
>   consumes.
>
> **Reproduce (Rust):**
>
> ```bash
> cat > /tmp/m.rs <<'EOF'
> fn main() { for i in 0..10_000 { println!("line {i}"); } }
> EOF
> rustc /tmp/m.rs -o /tmp/m && /tmp/m | head -n 5; echo "exit=${PIPESTATUS[0]}"
> # → failed printing to stdout: Broken pipe (os error 32)  /  exit=101
> ```
>
> **Fix — capture once, then paginate the captured text** (works for every
> language; never feeds the live process into a closing reader):
>
> ```bash
> set -euo pipefail
> OUTPUT="$(web-search --list-providers)"          # full output, pipe stays open
> test -n "$OUTPUT"                                  # still assert it produced output
> printf '%s\n' "$OUTPUT" | head -n 5                # paginate the *string*
> ```
>
> **Hardening (Rust source), recommended for any CLI a template ships:** render
> output into a buffer and write it with one fallible call, mapping `BrokenPipe`
> to a clean exit so a closed reader never panics:
>
> ```rust
> use std::io::{self, Write};
> fn write_stdout(s: &str) -> io::Result<()> {
>     match io::stdout().write_all(s.as_bytes()).and_then(|_| io::stdout().flush()) {
>         Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
>         r => r,
>     }
> }
> ```
>
> (Alternatively restore the default `SIGPIPE` disposition at startup, e.g. via
> the `libc`/`nix` `signal(SIGPIPE, SIG_DFL)` idiom.)
>
> Real-world reproduction and the full fix: link-assistant/web-search#15,
> `docs/case-studies/issue-15`.

---

## Report B — support multi-language monorepos: per-language tag namespace, `[Language]` titles, layout auto-detection

**Where to file:** all four templates (each owns one language of a polyglot
monorepo and must coexist with the others). Primary: **js** and **rust**.

**Title:** Release automation should auto-detect single- vs multi-language layout and namespace tags/titles per language

**Body:**

> When a repository hosts more than one language (e.g. `js/` + `rust/` in one
> monorepo), the per-language release workflows collide in a single flat
> namespace: tags and GitHub-release titles do not say which language they belong
> to, and `latest`/sorting becomes ambiguous.
>
> **Request:** make the release job detect layout and adapt, with **no
> configuration**:
>
> - **Auto-detect.** If the language's manifest is at the repo root, treat the
>   repo as single-language; if it lives in a language subdirectory
>   (`js/package.json`, `rust/Cargo.toml`, …), treat it as multi-language.
>
>   ```bash
>   # rust example
>   if [ -f Cargo.toml ]; then ROOT=.; MULTI=false; else ROOT=rust; MULTI=true; fi
>   ```
>
> - **Namespace the tag** in multi-language mode: `rust_v<version>` /
>   `js_v<version>` / `py_v<version>` / `cs_v<version>`; keep a plain
>   `v<version>` in single-language mode.
> - **Prefix the release title** `[Rust] <version>` / `[JavaScript] <version>` in
>   multi-language mode; `<name> <version>` in single-language mode.
>
> A small, unit-tested helper keeps tag/title construction idempotent (so
> re-running on an already-prefixed version does not double-prefix). Reference
> implementation: `js/scripts/release-naming.mjs` (+ `release-naming.test.js`)
> and the `version` step of `.github/workflows/rust.yml` in
> link-assistant/web-search#16.

---

## Report C — embed a package-manager badge in the GitHub release body

**Where to file:** all four templates.

**Title:** GitHub release body should include a badge linking to the published artifact

**Body:**

> After publishing, the GitHub release should advertise — and link to — the exact
> artifact that was published, so a reader can jump from the release straight to
> the registry page for that version. Add a shields.io badge to the generated
> release notes:
>
> - **npm:** `https://img.shields.io/npm/v/<pkg>/<dist-tag>.svg` (or a static
>   version badge) linking to `https://www.npmjs.com/package/<pkg>/v/<version>`.
> - **crates.io:** `https://img.shields.io/crates/v/<crate>.svg` linking to
>   `https://crates.io/crates/<crate>/<version>`.
> - **PyPI:** `https://img.shields.io/pypi/v/<dist>.svg` → `https://pypi.org/project/<dist>/<version>/`.
> - **NuGet:** `https://img.shields.io/nuget/v/<id>.svg` → `https://www.nuget.org/packages/<id>/<version>`.
>
> Build the link from the **bare** version (strip any `v`/`lang_v` tag prefix
> first) so it resolves to the version-specific page rather than 404ing.
> Reference implementation: `release-naming.mjs#buildNpmBadge` and the crates.io
> badge step of `rust.yml` in link-assistant/web-search#16.

---

## Report D (low priority) — keep `Cargo.lock` committed for binary-producing template repos

**Where to file:** **rust** template only.

**Title:** Document/enforce committing `Cargo.lock` for release-producing crates

**Body:**

> The template already comments `Cargo.lock` out of `.gitignore` with the note
> _"Remove Cargo.lock from gitignore if creating an executable"_, which is
> correct. Two hardening ideas so a downstream repo cannot silently regress:
>
> 1. A CI guard that fails if a `[[bin]]`/`cargo install`-able crate has no
>    committed `Cargo.lock`.
> 2. A note that the cache key `hashFiles('**/Cargo.lock')` silently degrades to
>    a constant (no lockfile → empty hash) when the lockfile is absent, so an
>    unpinned graph is cached and re-resolution drift goes unnoticed.
>
> Real-world impact: in link-assistant/web-search#15 an un-ignored, **un**committed
> lockfile let a freshly published `alloc-no-stdlib 3.0.0` break the `brotli`
> encoder build (`error[E0277]: StandardAlloc: alloc::Allocator<ZopfliNode> is not
> satisfied`) on fresh CI resolution, while a cached `target/` masked it in the
> main build job. The same commit compiled one hour and failed the next with no
> source change. Committing the lockfile restored determinism.

---

## Filing status

| Report | Templates | Action |
| ------ | --------- | ------ |
| A — SIGPIPE-safe smoke test | rust (primary), js, python, csharp | amend issue-6 issues #73/#81/#20/#27 |
| B — multi-language tag/title/detection | js, rust (primary), python, csharp | new issue per template |
| C — release-body package badge | js, rust, python, csharp | new issue per template |
| D — commit `Cargo.lock` for binaries | rust | new issue (low priority) |
