#!/usr/bin/env bun

/**
 * Create GitHub Release from CHANGELOG.md
 * Usage: node js/scripts/create-github-release.mjs --release-version <version> --repository <repository>
 *   release-version: Version number (e.g., 1.0.0)
 *   repository: GitHub repository (e.g., owner/repo)
 *
 * Uses link-foundation libraries:
 * - use-m: Dynamic package loading without package.json dependencies
 * - command-stream: Modern shell command execution with streaming support
 * - lino-arguments: Unified configuration from CLI args, env vars, and .lenv files
 */

import { readFileSync } from 'fs';
import { join } from 'path';

import { getJsRoot, parseJsRootConfig } from './js-paths.mjs';

// Load use-m dynamically
const { use } = eval(
  await (await fetch('https://unpkg.com/use-m/use.js')).text()
);

// Import link-foundation libraries
const { $ } = await use('command-stream');
const { makeConfig } = await use('lino-arguments');

// Parse CLI arguments using lino-arguments
// Note: Using --release-version instead of --version to avoid conflict with yargs' built-in --version flag
const config = makeConfig({
  yargs: ({ yargs, getenv }) =>
    yargs
      .option('release-version', {
        type: 'string',
        default: getenv('VERSION', ''),
        describe: 'Version number (e.g., 1.0.0)',
      })
      .option('repository', {
        type: 'string',
        default: getenv('REPOSITORY', ''),
        describe: 'GitHub repository (e.g., owner/repo)',
      })
      .option('js-root', {
        type: 'string',
        default: getenv('JS_ROOT', ''),
        describe:
          'JavaScript package root directory (auto-detected if not specified)',
      }),
});

const { releaseVersion: version, repository, jsRoot: jsRootArg } = config;
const jsRootConfig = jsRootArg || parseJsRootConfig();
const jsRoot = getJsRoot({ jsRoot: jsRootConfig, verbose: true });

if (!version || !repository) {
  console.error('Error: Missing required arguments');
  console.error(
    'Usage: node js/scripts/create-github-release.mjs --release-version <version> --repository <repository>'
  );
  process.exit(1);
}

const tag = `js-v${version}`;

// Keep comfortably below GitHub's observed 125000-character release body limit.
// Reference: link-foundation/js-ai-driven-development-pipeline-template
const GITHUB_RELEASE_BODY_MAX_BYTES = 120_000;
const textEncoder = new globalThis.TextEncoder();

function getUtf8ByteLength(value) {
  return textEncoder.encode(value).byteLength;
}

function truncateToUtf8Bytes(value, maxBytes) {
  const chunks = [];
  let usedBytes = 0;
  for (const character of value) {
    const characterBytes = getUtf8ByteLength(character);
    if (usedBytes + characterBytes > maxBytes) {
      break;
    }
    chunks.push(character);
    usedBytes += characterBytes;
  }
  return chunks.join('');
}

/**
 * Limit release notes to GitHub's body byte budget, appending a pointer to the
 * full tagged CHANGELOG.md when truncation is required.
 * @param {string} releaseNotes
 * @returns {string}
 */
function limitReleaseNotesBytes(releaseNotes) {
  if (getUtf8ByteLength(releaseNotes) <= GITHUB_RELEASE_BODY_MAX_BYTES) {
    return releaseNotes;
  }
  const changelogUrl = `https://github.com/${repository}/blob/${tag}/CHANGELOG.md`;
  const suffix = `\n\n...\n\nRelease notes were shortened to fit GitHub's release body limit. See the full tagged CHANGELOG.md: ${changelogUrl}`;
  const availableBytes = Math.max(
    0,
    GITHUB_RELEASE_BODY_MAX_BYTES - getUtf8ByteLength(suffix)
  );
  const shortenedNotes = truncateToUtf8Bytes(
    releaseNotes,
    availableBytes
  ).trimEnd();
  const limitedNotes = `${shortenedNotes}${suffix}`;
  if (getUtf8ByteLength(limitedNotes) <= GITHUB_RELEASE_BODY_MAX_BYTES) {
    return limitedNotes;
  }
  return truncateToUtf8Bytes(limitedNotes, GITHUB_RELEASE_BODY_MAX_BYTES);
}

console.log(`Creating GitHub release for ${tag}...`);

try {
  // Read CHANGELOG.md
  const changelogPath =
    jsRoot === '.' ? './CHANGELOG.md' : join(jsRoot, 'CHANGELOG.md');
  const changelog = readFileSync(changelogPath, 'utf8');

  // Extract changelog entry for this version
  // Read from CHANGELOG.md between this version header and the next version header
  const versionHeaderRegex = new RegExp(`## ${version}[\\s\\S]*?(?=## \\d|$)`);
  const match = changelog.match(versionHeaderRegex);

  let releaseNotes = '';
  if (match) {
    // Remove the version header itself and trim
    releaseNotes = match[0].replace(`## ${version}`, '').trim();
  }

  if (!releaseNotes) {
    releaseNotes = `Release ${version}`;
  }

  // Create release using GitHub API with JSON input
  // This avoids shell escaping issues that occur when passing text via command-line arguments
  // (Previously caused apostrophes like "didn't" to appear as "didn'''" in releases)
  const payload = JSON.stringify({
    tag_name: tag,
    name: `JavaScript ${version}`,
    body: limitReleaseNotesBytes(releaseNotes),
  });

  // Idempotent create: a re-run for an already-released version (e.g. after a
  // transient failure in a later step) must not turn the release job red.
  // GitHub returns HTTP 422 with an "already_exists" error code in that case.
  const result =
    await $`gh api repos/${repository}/releases -X POST --input -`.run({
      stdin: payload,
      capture: true,
      mirror: false,
    });

  const combinedOutput = `${result.stdout || ''}\n${result.stderr || ''}`;

  if (result.code === 0) {
    console.log(`\u2705 Created GitHub release: ${tag}`);
  } else if (/already_exists/i.test(combinedOutput)) {
    console.log(`GitHub release already exists: ${tag}. Skipping creation.`);
  } else {
    console.error(combinedOutput.trim());
    throw new Error(`gh api failed with exit code ${result.code}`);
  }
} catch (error) {
  console.error('Error creating release:', error.message);
  process.exit(1);
}
