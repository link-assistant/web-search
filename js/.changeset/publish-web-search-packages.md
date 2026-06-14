---
'@link-assistant/web-search': minor
---

Make `web-search` reliably consumable as a published package (issue #6,
FormalAI adoption blocker).

- Add a `serve` subcommand to the CLI (`web-search serve --port <port>`) as a
  parity alias for the existing `--serve` flag, matching the Rust CLI so both
  ecosystems share the same HTTP-server entry-point syntax.
- Self-healing JS release detection (`check-release-needed.mjs`): when
  `package.json` is ahead of npm but no changeset exists (e.g. a previous
  publish failed after consuming its changeset), the next push republishes the
  unpublished version instead of silently skipping.
- Install-from-package smoke test (`smoke-test-package.mjs`) run after publish:
  installs the released package from npm into a clean project and verifies the
  library, CLI, and HTTP-server entry points actually work.
- Document the library, CLI, and HTTP-server entry points in the README with
  versioned npm/cargo install commands.
