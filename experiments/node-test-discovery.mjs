// Run with: npm exec --yes --package=node@24 -- node experiments/node-test-discovery.mjs
// Reproduce the CI directory-argument failure and verify automatic discovery.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

assert.equal(
  process.versions.node.split('.')[0],
  '24',
  'Use the CI Node 24 runtime'
);
const fixture = mkdtempSync(join(tmpdir(), 'web-search-node-discovery-'));
try {
  mkdirSync(join(fixture, 'tests'));
  writeFileSync(
    join(fixture, 'tests', 'discovery.test.mjs'),
    "import test from 'node:test';\nimport assert from 'node:assert/strict';\ntest('discovered fixture', () => assert.equal(1 + 1, 2));\n"
  );
  const directory = spawnSync(process.execPath, ['--test', 'tests/'], {
    cwd: fixture,
    encoding: 'utf8',
  });
  assert.ifError(directory.error);
  assert.notEqual(directory.status, 0);
  assert.match(directory.stdout + directory.stderr, /MODULE_NOT_FOUND/);

  const discovery = spawnSync(process.execPath, ['--test'], {
    cwd: fixture,
    encoding: 'utf8',
  });
  assert.ifError(discovery.error);
  assert.equal(discovery.status, 0, discovery.stdout + discovery.stderr);
  assert.match(discovery.stdout, /discovered fixture/);
  console.log(
    'Node 24 directory failure reproduced; automatic discovery passes.'
  );
} finally {
  rmSync(fixture, { recursive: true, force: true });
}
