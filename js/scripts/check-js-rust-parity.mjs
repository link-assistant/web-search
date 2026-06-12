#!/usr/bin/env node

import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

import { CATEGORIES, getRegistry } from '../src/providers/index.js';

const ROOT = process.cwd();
const ACCESS_LABELS = new Set([
  'api',
  'html',
  'hybrid',
  'browser',
  'component',
]);

function fail(message) {
  console.error(`ERROR: ${message}`);
  process.exitCode = 1;
}

function assert(condition, message) {
  if (!condition) {
    fail(message);
  }
}

function exists(relativePath) {
  return existsSync(join(ROOT, relativePath));
}

function checkLanguageLayout() {
  const rootJsPaths = [
    'package.json',
    'package-lock.json',
    'eslint.config.js',
    'deno.json',
    'bunfig.toml',
    '.prettierrc',
    '.prettierignore',
    '.jscpd.json',
    '.changeset',
    '.husky',
    'src',
    'tests',
    'bin',
    'examples',
    'scripts',
  ];

  for (const path of rootJsPaths) {
    assert(!exists(path), `JavaScript-specific path must live in js/: ${path}`);
  }

  const requiredJsPaths = [
    'js/package.json',
    'js/package-lock.json',
    'js/eslint.config.js',
    'js/deno.json',
    'js/bunfig.toml',
    'js/.prettierrc',
    'js/.prettierignore',
    'js/.jscpd.json',
    'js/.changeset/config.json',
    'js/src/index.js',
    'js/tests/repository-layout.test.js',
    'js/bin/web-search.js',
    'js/examples/basic-usage.js',
    'js/scripts/detect-code-changes.mjs',
    'js/scripts/check-version.mjs',
    'js/scripts/validate-changeset.mjs',
    'js/scripts/check-js-rust-parity.mjs',
    'rust/scripts/detect-code-changes.rs',
    'rust/scripts/check-version-modification.rs',
    'rust/scripts/check-changelog-fragment.rs',
    'rust/scripts/check-file-size.rs',
    'rust/scripts/check-crate-size.rs',
  ];

  for (const path of requiredJsPaths) {
    assert(exists(path), `Missing expected JavaScript path: ${path}`);
  }

  const misplacedRustTests = readdirSync(join(ROOT, 'rust', 'src'), {
    recursive: true,
  })
    .map(String)
    .filter((file) => file.endsWith('.rs') && file.includes('test'));

  assert(
    misplacedRustTests.length === 0,
    `Rust tests must stay under rust/tests, found: ${misplacedRustTests.join(', ')}`
  );
}

function parseProviderOutput(output) {
  const providers = new Map();
  let category = null;

  for (const line of output.split('\n')) {
    const categoryMatch = line.match(/^([a-z]+)(?: \(\d+\))?:$/);
    if (categoryMatch) {
      category = categoryMatch[1];
      continue;
    }

    const providerMatch = line.match(
      /^\s{2}(\S+)\s+(.+) \(([^()]+(?:, [^()]+)*)\)(?: \[default\])?$/
    );
    if (!providerMatch || !category) {
      continue;
    }

    const [, id, label, flags] = providerMatch;
    providers.set(id, {
      id,
      label,
      category,
      access:
        flags
          .split(',')
          .map((flag) => flag.trim())
          .find((flag) => ACCESS_LABELS.has(flag)) || '',
    });
  }

  return providers;
}

function checkProviderParity() {
  const jsRegistry = getRegistry();
  const jsProviders = new Map(
    jsRegistry.map((entry) => [
      entry.id,
      {
        id: entry.id,
        label: entry.label,
        category: entry.category,
        access: entry.access,
      },
    ])
  );

  const rustOutput = execFileSync(
    'cargo',
    [
      'run',
      '--quiet',
      '--manifest-path',
      'rust/Cargo.toml',
      '--',
      '--list-providers',
    ],
    {
      cwd: ROOT,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'inherit'],
    }
  );
  const rustProviders = parseProviderOutput(rustOutput);

  assert(
    JSON.stringify(CATEGORIES) ===
      JSON.stringify(['search', 'knowledge', 'papers', 'code']),
    `Unexpected JavaScript category order: ${CATEGORIES.join(', ')}`
  );

  assert(
    jsProviders.size === rustProviders.size,
    `Provider count mismatch: JS=${jsProviders.size}, Rust=${rustProviders.size}`
  );

  for (const [id, jsProvider] of jsProviders) {
    const rustProvider = rustProviders.get(id);
    assert(rustProvider, `Rust registry is missing provider: ${id}`);
    if (!rustProvider) {
      continue;
    }
    assert(
      jsProvider.category === rustProvider.category,
      `Category mismatch for ${id}: JS=${jsProvider.category}, Rust=${rustProvider.category}`
    );
    assert(
      jsProvider.access === rustProvider.access,
      `Access mismatch for ${id}: JS=${jsProvider.access}, Rust=${rustProvider.access}`
    );
  }

  for (const id of rustProviders.keys()) {
    assert(
      jsProviders.has(id),
      `JavaScript registry is missing provider: ${id}`
    );
  }
}

checkLanguageLayout();
checkProviderParity();

if (process.exitCode) {
  process.exit(process.exitCode);
}

console.log('JS/Rust layout and provider parity checks passed');
