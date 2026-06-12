import { describe, it, expect } from 'test-anywhere';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const testDirectory = dirname(fileURLToPath(import.meta.url));
const packageOrProjectRoot = resolve(testDirectory, '..');
const projectRoot =
  basename(packageOrProjectRoot) === 'js'
    ? resolve(packageOrProjectRoot, '..')
    : packageOrProjectRoot;
const jsRoot = join(projectRoot, 'js');
const packageJson = JSON.parse(
  readFileSync(join(jsRoot, 'package.json'), 'utf8')
);

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
      'scripts',
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
      'scripts/detect-code-changes.mjs',
      'scripts/check-version.mjs',
      'scripts/validate-changeset.mjs',
      'scripts/check-js-rust-parity.mjs',
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

  it('keeps CI/CD scripts in their language folders', () => {
    for (const rustScriptPath of [
      'rust/scripts/detect-code-changes.rs',
      'rust/scripts/check-version-modification.rs',
      'rust/scripts/check-changelog-fragment.rs',
      'rust/scripts/check-file-size.rs',
      'rust/scripts/check-crate-size.rs',
    ]) {
      expect(exists(rustScriptPath)).toBe(true);
    }

    const jsWorkflow = readFileSync(
      join(projectRoot, '.github/workflows/js.yml'),
      'utf8'
    );
    const rustWorkflow = readFileSync(
      join(projectRoot, '.github/workflows/rust.yml'),
      'utf8'
    );
    const parityWorkflow = readFileSync(
      join(projectRoot, '.github/workflows/parity.yml'),
      'utf8'
    );

    expect(jsWorkflow).toContain('node js/scripts/detect-code-changes.mjs');
    expect(jsWorkflow).toContain('node js/scripts/validate-changeset.mjs');
    expect(jsWorkflow).not.toContain('node scripts/');
    expect(jsWorkflow).not.toContain('../scripts/');

    expect(rustWorkflow).toContain(
      'rust-script rust/scripts/detect-code-changes.rs'
    );
    expect(rustWorkflow).toContain(
      'rust-script rust/scripts/check-version-modification.rs'
    );
    expect(rustWorkflow).toContain(
      'rust-script rust/scripts/check-changelog-fragment.rs'
    );

    expect(parityWorkflow).toContain(
      'node js/scripts/check-js-rust-parity.mjs'
    );
    expect(parityWorkflow).not.toContain('node scripts/');
  });

  it('keeps npm publish metadata explicit and registry-safe', () => {
    expect(packageJson.bin).toEqual({
      'web-search': 'bin/web-search.js',
    });
    expect(existsSync(join(jsRoot, packageJson.bin['web-search']))).toBe(true);

    expect(packageJson.repository).toEqual({
      type: 'git',
      url: 'git+https://github.com/link-assistant/web-search.git',
      directory: 'js',
    });
    expect(packageJson.publishConfig).toEqual({
      access: 'public',
    });
    expect(packageJson.scripts.prepare).toBe(undefined);
  });

  it('publishes only runtime package assets to npm', () => {
    expect(packageJson.files).toEqual([
      'src',
      'bin',
      'examples',
      'README.md',
      'CHANGELOG.md',
    ]);

    for (const devOnlyPath of [
      '.changeset',
      '.husky',
      '.jscpd.json',
      '.prettierrc',
      'eslint.config.js',
      'scripts',
      'tests',
    ]) {
      expect(packageJson.files.includes(devOnlyPath)).toBe(false);
    }
  });
});
