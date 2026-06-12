# Changelog Fragments

This directory contains Rust changelog fragments for pull requests that change Rust source, tests, scripts, package metadata, or Rust workflows.

## Fragment Format

Each fragment is a Markdown file with frontmatter that declares the semantic version bump:

```markdown
---
bump: patch
---

### Changed
- Description of the Rust change.
```

Use `major` for breaking changes, `minor` for backward-compatible features, and `patch` for fixes or internal workflow changes. If a fragment omits `bump`, release tooling should treat it as `patch`.

## Naming

Use a descriptive unique filename, for example:

```bash
rust/changelog.d/provider-timeouts.md
rust/changelog.d/ci-script-guards.md
```

The CI changelog check ignores this README and requires at least one non-README `*.md` fragment whenever Rust source, tests, scripts, package metadata, or workflow files change in a pull request.
