#!/usr/bin/env node

/**
 * Check for manual version modifications in package.json
 *
 * This script prevents manual version changes in pull requests.
 * Versions should only be changed by the CI/CD pipeline using changesets.
 *
 * Key behavior:
 * - For PRs: compares PR head against base branch to detect version changes
 * - Skips check for automated release PRs (changeset-release/* branches)
 * - Fails the build if manual version changes are detected
 *
 * Usage:
 *   node scripts/check-version.mjs
 *
 * Environment variables (set by GitHub Actions):
 *   - GITHUB_HEAD_REF: Branch name of the PR head
 *   - GITHUB_BASE_REF: Branch name of the PR base
 *
 * Exit codes:
 *   - 0: No manual version changes detected (or skipped for release PRs)
 *   - 1: Manual version changes detected
 */

import { execSync } from 'child_process';

import {
  getJsRoot,
  getPackageJsonPath,
  parseJsRootConfig,
} from './js-paths.mjs';

const jsRoot = getJsRoot({ jsRoot: parseJsRootConfig(), verbose: true });
const packageJsonPath = getPackageJsonPath({ jsRoot }).replace(/^\.\//, '');

/**
 * Execute a shell command and return trimmed output
 * @param {string} command - The command to execute
 * @returns {string} - The trimmed command output
 */
function exec(command) {
  try {
    return execSync(command, { encoding: 'utf-8' }).trim();
  } catch (error) {
    console.error(`Error executing command: ${command}`);
    console.error(error.message);
    return '';
  }
}

/**
 * Check if this is an automated release PR that should skip version check
 * @returns {boolean} True if version check should be skipped
 */
function shouldSkipVersionCheck() {
  const headRef = process.env.GITHUB_HEAD_REF || '';

  // Skip check for automated release PRs created by changeset
  const skipPatterns = ['changeset-release/', 'changeset-manual-release-'];

  for (const pattern of skipPatterns) {
    if (headRef.startsWith(pattern)) {
      return true;
    }
  }

  return false;
}

/**
 * Read a package version from a git ref.
 * @param {string} ref - Git ref to read from
 * @param {string} path - Package file path at that ref
 * @returns {string | null} Package version, or null when unavailable
 */
function readVersionAtRef(ref, path) {
  let content = '';
  try {
    content = execSync(`git show ${ref}:${path}`, {
      encoding: 'utf-8',
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim();
  } catch {
    return null;
  }

  try {
    return JSON.parse(content).version || null;
  } catch {
    return null;
  }
}

/**
 * Get the version change from package.json.
 * @returns {string} The version change if found, empty string otherwise
 */
function getVersionDiff() {
  const baseRef = process.env.GITHUB_BASE_REF || 'main';

  const baseVersion =
    readVersionAtRef(`origin/${baseRef}`, packageJsonPath) ||
    readVersionAtRef(`origin/${baseRef}`, 'package.json');
  const headVersion = readVersionAtRef('HEAD', packageJsonPath);

  if (!baseVersion || !headVersion || baseVersion === headVersion) {
    return '';
  }

  return `${packageJsonPath}: ${baseVersion} -> ${headVersion}`;
}

/**
 * Main function to check for version changes
 */
function checkVersion() {
  console.log(`Checking for manual version changes in ${packageJsonPath}...\n`);

  // Check if we should skip the version check
  if (shouldSkipVersionCheck()) {
    const headRef = process.env.GITHUB_HEAD_REF || '';
    console.log(`Skipping version check for automated release PR: ${headRef}`);
    process.exit(0);
  }

  // Get the version diff
  const versionDiff = getVersionDiff();

  if (versionDiff) {
    console.error('::error::Manual version change detected in package.json');
    console.error('');
    console.error(
      'Version changes in package.json are prohibited in pull requests.'
    );
    console.error(
      'Versions are managed automatically by the CI/CD pipeline using changesets.'
    );
    console.error('');
    console.error('To request a release:');
    console.error(
      '  1. Add a changeset file describing your changes (npx changeset)'
    );
    console.error(
      '  2. The release workflow will automatically bump the version when merged'
    );
    console.error('');
    console.error('Detected change:');
    console.error(versionDiff);
    process.exit(1);
  }

  console.log('No manual version changes detected - check passed');
  process.exit(0);
}

// Run the check
checkVersion();
