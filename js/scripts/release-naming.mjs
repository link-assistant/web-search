#!/usr/bin/env node

/**
 * Release-naming helper for the JavaScript package.
 *
 * Centralises the tag / title / badge conventions so that every script which
 * touches a GitHub release (create-github-release.mjs, format-github-release.mjs,
 * format-release-notes.mjs) stays in sync.
 *
 * Single-language vs multi-language auto-detection
 * ------------------------------------------------
 * The layout is derived from js-paths.mjs `getJsRoot()`:
 *   - jsRoot === '.'  → single-language repo (package.json at the repo root)
 *   - jsRoot === 'js' → multi-language repo (package.json under js/)
 *
 * In a multi-language repo the JavaScript and Rust (and any future language)
 * releases share one GitHub Releases list, so they must be disambiguated:
 *   - tags are namespaced with a short `js-` prefix (e.g. `js-1.2.3`)
 *   - titles are prefixed with the language (e.g. `[JavaScript] 1.2.3`)
 *
 * In a single-language repo there is nothing to disambiguate, so the bare
 * `1.2.3` tag and `JavaScript 1.2.3` title are used.
 *
 * Tag format (issue #17): tags are deliberately SHORT — no `v` prefix and a
 * single hyphen separator (`js-1.2.3`, `rust-1.2.3`), never the longer
 * `js_v1.2.3` / `rust_v1.2.3` forms used historically. normalizeVersion()
 * still accepts every legacy spelling (`v`, `js-v`, `js_v`, `rust_v`, `rust-v`)
 * so old tags remain valid inputs.
 */

import { getJsRoot } from './js-paths.mjs';

/**
 * @param {Object} [options]
 * @param {string} [options.jsRoot] - Explicit JS root override (passed through to getJsRoot)
 * @param {boolean} [options.verbose=false]
 * @returns {boolean} true when the repo is laid out as multi-language (js/ subfolder)
 */
export function isMultiLanguage(options = {}) {
  return getJsRoot(options) !== '.';
}

/**
 * The tag prefix to use for this repository layout.
 * Multi-language → `js-` (short, no `v`); single-language → `''` (bare semver).
 * @param {Object} [options]
 * @returns {string}
 */
export function getTagPrefix(options = {}) {
  return isMultiLanguage(options) ? 'js-' : '';
}

/**
 * Build the git tag for a release version.
 * The version is normalised first so callers may pass `1.2.3`, `v1.2.3`,
 * `js-v1.2.3` or `js_v1.2.3` and always get a consistent result.
 * @param {string} version
 * @param {Object} [options]
 * @returns {string} e.g. `js-1.2.3` (multi) or `1.2.3` (single)
 */
export function buildReleaseTag(version, options = {}) {
  return `${getTagPrefix(options)}${normalizeVersion(version)}`;
}

/**
 * Build the GitHub release title.
 * Multi-language → `[JavaScript] 1.2.3`, single-language → `JavaScript 1.2.3`.
 * @param {string} version
 * @param {Object} [options]
 * @returns {string}
 */
export function buildReleaseTitle(version, options = {}) {
  const v = normalizeVersion(version);
  return isMultiLanguage(options) ? `[JavaScript] ${v}` : `JavaScript ${v}`;
}

/**
 * Strip any leading tag prefix (`v`, `js-v`, `js_v`, `rust-v`, …) from a
 * version string, returning the bare semver. Mirrors the normalisation used by
 * the upstream templates so a tag and a version are interchangeable inputs.
 * @param {string} version
 * @returns {string}
 */
export function normalizeVersion(version) {
  if (!version) {
    return '';
  }
  // Remove an optional `<word>` prefix followed by `-` or `_`, then an
  // optional leading `v`. Examples:
  //   js-v1.2.3 → 1.2.3, rust_v0.2.0 → 0.2.0, v1.2.3 → 1.2.3, 1.2.3 → 1.2.3
  return String(version)
    .trim()
    .replace(/^[A-Za-z]+[-_]v?/, '')
    .replace(/^v/, '');
}

/**
 * Build the npm shields.io badge that links to the published version's page.
 * @param {string} packageName - e.g. `@link-assistant/web-search`
 * @param {string} version
 * @returns {string} Markdown badge
 */
export function buildNpmBadge(packageName, version) {
  const v = normalizeVersion(version);
  // Static badge showing the released version, linking to that exact version's
  // page on npmjs.com (the "target version's release on the package manager").
  return `[![npm version](https://img.shields.io/badge/npm-${encodeURIComponent(
    v
  )}-blue.svg?logo=npm)](https://www.npmjs.com/package/${packageName}/v/${v})`;
}
