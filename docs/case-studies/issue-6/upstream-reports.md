# Upstream reports filed for issue #6

Shared CI/CD gap: release pipelines confirm registry _indexing_ but never
install the _published_ artifact and run its entry points. Filed to all four
link-foundation pipeline templates with reproducible examples, workarounds,
and a fix suggestion.

| Template | Issue                                                                                       |
| -------- | ------------------------------------------------------------------------------------------- |
| js       | https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/81     |
| rust     | https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/73   |
| python   | https://github.com/link-foundation/python-ai-driven-development-pipeline-template/issues/20 |
| csharp   | https://github.com/link-foundation/csharp-ai-driven-development-pipeline-template/issues/27 |

---

## js template — filed issue body

## Summary

The `release.yml` pipeline publishes to npm and waits for the version to be
indexed (`verifyPublished`), but it never **installs the published package from
npm into a clean project and runs it**. There is no install-from-package smoke
test, so a successfully-published-but-broken package (bad `files`, wrong `bin`
path, misconfigured `exports`) still produces a green release.

## Reproducible example

1. Misconfigure packaging (e.g. drop the CLI entry from `files`, or point `bin`
   at a path that is not packaged).
2. Push a release. `changeset publish` succeeds and `npm view <pkg>@<ver>`
   resolves, so the release is green.
3. A consumer runs `npm install <pkg>@<ver>` and `npx <bin>` — and it fails.

## Why indexing checks are not enough

The release pipeline already polls the registry until the new version is
_indexed_ (e.g. `Wait for crate/NuGet indexing`, `verifyPublished`). That proves
the version _exists_, not that it is _usable_. A package can index perfectly yet
be broken for consumers when:

- `files` / `package.include` / packaged paths omit a required file
- the `bin` / `[[bin]]` / entry-point path is wrong or not executable
- `main` / `exports` / the published module shape is misconfigured
- a runtime dependency is declared as a dev-dependency (so it is missing on install)

In every one of these cases the release goes green and the release notes
advertise an install command that does not work.

## Suggested fix

Add a **post-publish smoke-test step** that installs the just-published artifact
from the registry into a throwaway project (NOT the repo checkout) and exercises
each advertised entry point (library import, CLI, and HTTP server if any),
with retries to absorb registry-propagation lag. Fail the release loudly if any
entry point is broken.

## Reference implementation

`link-assistant/web-search` (issue
[#6](https://github.com/link-assistant/web-search/issues/6)) implements exactly
this:

- npm: [`js/scripts/smoke-test-package.mjs`](https://github.com/link-assistant/web-search/blob/main/js/scripts/smoke-test-package.mjs)
  — temp project, `npm install <pkg>@<version>` with retries, then verifies
  library import + `--list-providers` CLI + `serve` HTTP `/health`.
- crates.io: an inline `Smoke-test published crate` step in
  [`.github/workflows/rust.yml`](https://github.com/link-assistant/web-search/blob/main/.github/workflows/rust.yml)
  — `cargo install <crate>@<version>`, run the binary, and compile a fresh
  dependent crate against the library.

Happy to open a PR porting the pattern back to this template.

---

## rust template — filed issue body

## Summary

The `release.yml` pipeline publishes to crates.io and waits for the version to
be available (`wait-for-crate.rs`), but it never **installs the published crate
from crates.io and runs it**. There is no install-from-package smoke test, so a
crate that publishes but is broken for downstream consumers (wrong `[[bin]]`
path, a runtime dep declared under `[dev-dependencies]`, an `include` that omits
files) still produces a green release.

## Reproducible example

1. Misconfigure the crate (e.g. an `include` list that omits a module, or a
   `[[bin]]` path that does not match the published sources).
2. Push a release. `cargo publish` succeeds and the crates.io API returns 200,
   so the release is green.
3. A consumer runs `cargo install <crate>` or adds `<crate> = "x.y.z"` — and it
   fails to build/run.

## Why indexing checks are not enough

The release pipeline already polls the registry until the new version is
_indexed_ (e.g. `Wait for crate/NuGet indexing`, `verifyPublished`). That proves
the version _exists_, not that it is _usable_. A package can index perfectly yet
be broken for consumers when:

- `files` / `package.include` / packaged paths omit a required file
- the `bin` / `[[bin]]` / entry-point path is wrong or not executable
- `main` / `exports` / the published module shape is misconfigured
- a runtime dependency is declared as a dev-dependency (so it is missing on install)

In every one of these cases the release goes green and the release notes
advertise an install command that does not work.

## Suggested fix

Add a **post-publish smoke-test step** that installs the just-published artifact
from the registry into a throwaway project (NOT the repo checkout) and exercises
each advertised entry point (library import, CLI, and HTTP server if any),
with retries to absorb registry-propagation lag. Fail the release loudly if any
entry point is broken.

## Reference implementation

`link-assistant/web-search` (issue
[#6](https://github.com/link-assistant/web-search/issues/6)) implements exactly
this:

- npm: [`js/scripts/smoke-test-package.mjs`](https://github.com/link-assistant/web-search/blob/main/js/scripts/smoke-test-package.mjs)
  — temp project, `npm install <pkg>@<version>` with retries, then verifies
  library import + `--list-providers` CLI + `serve` HTTP `/health`.
- crates.io: an inline `Smoke-test published crate` step in
  [`.github/workflows/rust.yml`](https://github.com/link-assistant/web-search/blob/main/.github/workflows/rust.yml)
  — `cargo install <crate>@<version>`, run the binary, and compile a fresh
  dependent crate against the library.

Happy to open a PR porting the pattern back to this template.

---

## python template — filed issue body

## Summary

The `release.yml` pipeline builds with `build`, publishes with `twine`, but it
never **installs the published distribution from PyPI into a clean virtualenv
and imports/runs it**. There is no install-from-package smoke test, so a wheel
that uploads but is broken for consumers (missing package data, wrong
`[project.scripts]` entry point, a runtime dep left in `[dev]` extras) still
produces a green release.

## Reproducible example

1. Misconfigure packaging (e.g. a module missing from the wheel, or a console
   `[project.scripts]` entry point that imports a non-packaged module).
2. Push a release. `twine upload` succeeds, so the release is green.
3. A consumer runs `pip install <pkg>==<ver>` and the console script / import
   fails.

## Why indexing checks are not enough

The release pipeline already polls the registry until the new version is
_indexed_ (e.g. `Wait for crate/NuGet indexing`, `verifyPublished`). That proves
the version _exists_, not that it is _usable_. A package can index perfectly yet
be broken for consumers when:

- `files` / `package.include` / packaged paths omit a required file
- the `bin` / `[[bin]]` / entry-point path is wrong or not executable
- `main` / `exports` / the published module shape is misconfigured
- a runtime dependency is declared as a dev-dependency (so it is missing on install)

In every one of these cases the release goes green and the release notes
advertise an install command that does not work.

## Suggested fix

Add a **post-publish smoke-test step** that installs the just-published artifact
from the registry into a throwaway project (NOT the repo checkout) and exercises
each advertised entry point (library import, CLI, and HTTP server if any),
with retries to absorb registry-propagation lag. Fail the release loudly if any
entry point is broken.

## Reference implementation

`link-assistant/web-search` (issue
[#6](https://github.com/link-assistant/web-search/issues/6)) implements exactly
this:

- npm: [`js/scripts/smoke-test-package.mjs`](https://github.com/link-assistant/web-search/blob/main/js/scripts/smoke-test-package.mjs)
  — temp project, `npm install <pkg>@<version>` with retries, then verifies
  library import + `--list-providers` CLI + `serve` HTTP `/health`.
- crates.io: an inline `Smoke-test published crate` step in
  [`.github/workflows/rust.yml`](https://github.com/link-assistant/web-search/blob/main/.github/workflows/rust.yml)
  — `cargo install <crate>@<version>`, run the binary, and compile a fresh
  dependent crate against the library.

Happy to open a PR porting the pattern back to this template.

---

## csharp template — filed issue body

## Summary

The `release.yml` pipeline pushes to NuGet and waits for indexing
(`wait-for-nuget.mjs`, issue #13), but it never **installs the published package
from NuGet into a clean project and builds/runs against it**. There is no
install-from-package smoke test, so a package that indexes but is broken for
consumers (missing target framework asset, wrong `PackAsTool` command name,
a runtime dependency not flowed into the `.nuspec`) still produces a green
release.

## Reproducible example

1. Misconfigure packaging (e.g. a missing `lib/<tfm>` asset, or a tool command
   name that does not match the packed entry point).
2. Push a release. `dotnet nuget push` succeeds and the flat-container API
   reflects the version, so the release is green.
3. A consumer runs `dotnet add package <id> --version <ver>` (or
   `dotnet tool install`) and the build/tool fails.

## Why indexing checks are not enough

The release pipeline already polls the registry until the new version is
_indexed_ (e.g. `Wait for crate/NuGet indexing`, `verifyPublished`). That proves
the version _exists_, not that it is _usable_. A package can index perfectly yet
be broken for consumers when:

- `files` / `package.include` / packaged paths omit a required file
- the `bin` / `[[bin]]` / entry-point path is wrong or not executable
- `main` / `exports` / the published module shape is misconfigured
- a runtime dependency is declared as a dev-dependency (so it is missing on install)

In every one of these cases the release goes green and the release notes
advertise an install command that does not work.

## Suggested fix

Add a **post-publish smoke-test step** that installs the just-published artifact
from the registry into a throwaway project (NOT the repo checkout) and exercises
each advertised entry point (library import, CLI, and HTTP server if any),
with retries to absorb registry-propagation lag. Fail the release loudly if any
entry point is broken.

## Reference implementation

`link-assistant/web-search` (issue
[#6](https://github.com/link-assistant/web-search/issues/6)) implements exactly
this:

- npm: [`js/scripts/smoke-test-package.mjs`](https://github.com/link-assistant/web-search/blob/main/js/scripts/smoke-test-package.mjs)
  — temp project, `npm install <pkg>@<version>` with retries, then verifies
  library import + `--list-providers` CLI + `serve` HTTP `/health`.
- crates.io: an inline `Smoke-test published crate` step in
  [`.github/workflows/rust.yml`](https://github.com/link-assistant/web-search/blob/main/.github/workflows/rust.yml)
  — `cargo install <crate>@<version>`, run the binary, and compile a fresh
  dependent crate against the library.

Happy to open a PR porting the pattern back to this template.
