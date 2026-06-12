import { describe, it, expect } from 'test-anywhere';
import { existsSync, readdirSync } from 'node:fs';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const testDirectory = dirname(fileURLToPath(import.meta.url));
const packageOrProjectRoot = resolve(testDirectory, '..');
const projectRoot =
  basename(packageOrProjectRoot) === 'js'
    ? resolve(packageOrProjectRoot, '..')
    : packageOrProjectRoot;
const jsRoot = join(projectRoot, 'js');

function exists(relativePath) {
  return existsSync(join(projectRoot, relativePath));
}

describe('repository language layout', () => {
  it('keeps JavaScript package files in js/ instead of the repository root', () => {
    for (const rootOnlyPath of [
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
    ]) {
      expect(exists(rootOnlyPath)).toBe(false);
    }

    for (const jsPath of [
      'package.json',
      'package-lock.json',
      'eslint.config.js',
      'deno.json',
      'bunfig.toml',
      '.prettierrc',
      '.prettierignore',
      '.jscpd.json',
      '.changeset/config.json',
      'src/index.js',
      'tests/repository-layout.test.js',
      'bin/web-search.js',
      'examples/basic-usage.js',
    ]) {
      expect(existsSync(join(jsRoot, jsPath))).toBe(true);
    }
  });

  it('keeps Rust tests in rust/tests instead of rust/src', () => {
    const rustSource = join(projectRoot, 'rust', 'src');
    const misplacedTests = readdirSync(rustSource, { recursive: true })
      .map(String)
      .filter((file) => file.endsWith('.rs') && file.includes('test'));

    expect(misplacedTests).toEqual([]);
    expect(exists('rust/tests')).toBe(true);
  });
});
