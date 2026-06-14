#!/usr/bin/env node

/**
 * Install-from-package smoke test (issue #6).
 *
 * After a release publishes `@link-assistant/web-search` to npm, this script
 * proves the published artifact is actually installable and that all three
 * advertised entry points work from a clean install — NOT from the repo
 * checkout:
 *
 *   1. Library  — `import { createSearchEngine, WebSearchEngine }`
 *   2. CLI      — `web-search --list-providers` (offline, no network)
 *   3. HTTP     — `web-search serve` / `--serve` boots and answers /health
 *
 * It creates a throwaway project in a temp dir, installs the package straight
 * from the npm registry (with retries to absorb registry propagation lag),
 * then exercises each entry point. Any failure exits non-zero so the release
 * job fails loudly instead of advertising a broken package.
 *
 * Usage:
 *   node js/scripts/smoke-test-package.mjs --package-version <version> [--js-root js]
 *
 * If --package-version is omitted, the version from package.json is used.
 */

import { execSync, spawn } from 'child_process';
import { mkdtempSync, writeFileSync, readFileSync, rmSync } from 'fs';
import { tmpdir } from 'os';
import { join } from 'path';

import {
  getJsRoot,
  getPackageJsonPath,
  parseJsRootConfig,
} from './js-paths.mjs';

const jsRoot = getJsRoot({ jsRoot: parseJsRootConfig(), verbose: true });

function parseArg(name) {
  const idx = process.argv.indexOf(name);
  return idx !== -1 ? process.argv[idx + 1] : undefined;
}

function log(msg) {
  console.log(`[smoke-test] ${msg}`);
}

function getPackageInfo() {
  const pkg = JSON.parse(readFileSync(getPackageJsonPath({ jsRoot }), 'utf8'));
  return { name: pkg.name, version: pkg.version };
}

/**
 * Install the package from npm, retrying to absorb registry propagation delay.
 * @param {string} dir - working directory
 * @param {string} spec - e.g. "@link-assistant/web-search@0.9.0"
 */
function installFromNpm(dir, spec) {
  const maxAttempts = 5;
  for (let attempt = 1; attempt <= maxAttempts; attempt++) {
    try {
      log(`Installing ${spec} (attempt ${attempt}/${maxAttempts})...`);
      execSync(`npm install ${spec} --no-audit --no-fund`, {
        cwd: dir,
        stdio: 'inherit',
      });
      return;
    } catch (error) {
      if (attempt === maxAttempts) {
        throw new Error(
          `Failed to install ${spec} after ${maxAttempts} attempts: ${error.message}`,
          { cause: error }
        );
      }
      const waitSeconds = attempt * 10;
      log(`Install failed; waiting ${waitSeconds}s for npm propagation...`);
      execSync(`sleep ${waitSeconds}`);
    }
  }
}

/** Verify the library entry point. */
function checkLibrary(dir, name) {
  log('Checking library entry point (import)...');
  const script = `
    import { createSearchEngine, WebSearchEngine } from '${name}';
    if (typeof createSearchEngine !== 'function') {
      throw new Error('createSearchEngine is not exported as a function');
    }
    if (typeof WebSearchEngine !== 'function') {
      throw new Error('WebSearchEngine is not exported as a function');
    }
    const engine = createSearchEngine();
    if (!engine) throw new Error('createSearchEngine() returned falsy');
    console.log('library OK: createSearchEngine + WebSearchEngine importable');
  `;
  writeFileSync(join(dir, 'check-library.mjs'), script);
  execSync('node check-library.mjs', { cwd: dir, stdio: 'inherit' });
}

/** Resolve the installed package's CLI bin path. */
function resolveBin(dir, name) {
  const pkgJsonPath = join(dir, 'node_modules', name, 'package.json');
  const pkg = JSON.parse(readFileSync(pkgJsonPath, 'utf8'));
  const binRel = typeof pkg.bin === 'string' ? pkg.bin : pkg.bin['web-search'];
  return join(dir, 'node_modules', name, binRel);
}

/** Verify the CLI entry point (offline). */
function checkCli(dir, binPath) {
  log('Checking CLI entry point (web-search --list-providers)...');
  const out = execSync(`node "${binPath}" --list-providers`, {
    cwd: dir,
    encoding: 'utf8',
  });
  if (!out.trim()) {
    throw new Error('CLI --list-providers produced no output');
  }
  log('CLI OK: --list-providers listed providers');
}

/** Verify the HTTP-server entry point boots and answers /health. */
async function checkServer(dir, binPath) {
  log('Checking HTTP server entry point (web-search serve)...');
  const port = 38217;
  const child = spawn('node', [binPath, 'serve', '--port', String(port)], {
    cwd: dir,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let stderr = '';
  child.stderr.on('data', (d) => (stderr += d));

  try {
    await waitForHealth(port);
    log('server OK: /health responded');
  } catch (error) {
    throw new Error(
      `HTTP server smoke test failed: ${error.message}\nServer stderr:\n${stderr}`,
      { cause: error }
    );
  } finally {
    child.kill('SIGINT');
  }
}

async function waitForHealth(port) {
  const deadline = 15000;
  const start = process.hrtime.bigint();
  let lastError;
  while (Number(process.hrtime.bigint() - start) / 1e6 < deadline) {
    try {
      const res = await fetch(`http://localhost:${port}/health`);
      if (res.ok) {
        return;
      }
      lastError = new Error(`/health returned status ${res.status}`);
    } catch (error) {
      lastError = error;
    }
    await new Promise((r) => setTimeout(r, 500));
  }
  throw lastError || new Error('Timed out waiting for /health');
}

async function main() {
  const { name, version: pkgVersion } = getPackageInfo();
  const version = parseArg('--package-version') || pkgVersion;
  const spec = `${name}@${version}`;

  log(`Smoke-testing installable package ${spec}`);
  const dir = mkdtempSync(join(tmpdir(), 'web-search-smoke-'));
  log(`Workspace: ${dir}`);

  try {
    writeFileSync(
      join(dir, 'package.json'),
      JSON.stringify(
        { name: 'web-search-smoke-test', private: true, type: 'module' },
        null,
        2
      )
    );

    installFromNpm(dir, spec);
    checkLibrary(dir, name);
    const binPath = resolveBin(dir, name);
    checkCli(dir, binPath);
    await checkServer(dir, binPath);

    log(`All entry points verified for ${spec} ✓`);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(`[smoke-test] FAILED: ${error.message}`);
  process.exit(1);
});
