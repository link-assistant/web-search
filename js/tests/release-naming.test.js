import { describe, it, expect } from 'test-anywhere';

import {
  isMultiLanguage,
  getTagPrefix,
  buildReleaseTag,
  buildReleaseTitle,
  normalizeVersion,
  buildNpmBadge,
} from '../scripts/release-naming.mjs';

// All helpers accept an explicit { jsRoot } so these tests never touch the
// filesystem and deterministically exercise both layouts.
const MULTI = { jsRoot: 'js' };
const SINGLE = { jsRoot: '.' };

describe('release-naming: layout detection', () => {
  it('treats a js/ subfolder layout as multi-language', () => {
    expect(isMultiLanguage(MULTI)).toBe(true);
  });

  it('treats a root package.json layout as single-language', () => {
    expect(isMultiLanguage(SINGLE)).toBe(false);
  });
});

describe('release-naming: tag prefix', () => {
  it('namespaces the tag with the short js- prefix in multi-language repos', () => {
    expect(getTagPrefix(MULTI)).toBe('js-');
  });

  it('uses no prefix (bare semver) in single-language repos', () => {
    expect(getTagPrefix(SINGLE)).toBe('');
  });
});

describe('release-naming: buildReleaseTag', () => {
  it('produces js-<semver> for multi-language repos (short, no v)', () => {
    expect(buildReleaseTag('1.2.3', MULTI)).toBe('js-1.2.3');
  });

  it('produces a bare <semver> for single-language repos (no v)', () => {
    expect(buildReleaseTag('1.2.3', SINGLE)).toBe('1.2.3');
  });

  it('never emits a v prefix or underscore separator', () => {
    expect(buildReleaseTag('1.2.3', MULTI)).not.toContain('v');
    expect(buildReleaseTag('1.2.3', MULTI)).not.toContain('_');
    expect(buildReleaseTag('1.2.3', SINGLE)).not.toContain('v');
  });

  it('is idempotent across legacy and new spellings', () => {
    expect(buildReleaseTag('js-1.2.3', MULTI)).toBe('js-1.2.3');
    expect(buildReleaseTag('js_v1.2.3', MULTI)).toBe('js-1.2.3');
    expect(buildReleaseTag('js-v1.2.3', MULTI)).toBe('js-1.2.3');
    expect(buildReleaseTag('v1.2.3', SINGLE)).toBe('1.2.3');
    expect(buildReleaseTag('1.2.3', SINGLE)).toBe('1.2.3');
  });
});

describe('release-naming: buildReleaseTitle', () => {
  it('prefixes the language in multi-language repos', () => {
    expect(buildReleaseTitle('1.2.3', MULTI)).toBe('[JavaScript] 1.2.3');
  });

  it('keeps the historical title in single-language repos', () => {
    expect(buildReleaseTitle('1.2.3', SINGLE)).toBe('JavaScript 1.2.3');
  });

  it('normalizes a prefixed version before titling', () => {
    expect(buildReleaseTitle('js_v1.2.3', MULTI)).toBe('[JavaScript] 1.2.3');
  });
});

describe('release-naming: normalizeVersion', () => {
  const cases = [
    ['1.2.3', '1.2.3'],
    ['v1.2.3', '1.2.3'],
    ['js-v1.2.3', '1.2.3'],
    ['js_v1.2.3', '1.2.3'],
    ['rust_v0.2.0', '0.2.0'],
    ['rust-v0.2.0', '0.2.0'],
    ['rust-0.2.0', '0.2.0'],
    ['js-1.2.3', '1.2.3'],
    ['1.2.3-beta.1', '1.2.3-beta.1'],
    ['js_v1.2.3-beta.1', '1.2.3-beta.1'],
    ['', ''],
  ];
  for (const [input, expected] of cases) {
    it(`normalizes ${JSON.stringify(input)} to ${JSON.stringify(expected)}`, () => {
      expect(normalizeVersion(input)).toBe(expected);
    });
  }
});

describe('release-naming: buildNpmBadge', () => {
  it('links the badge to the exact published version page', () => {
    const badge = buildNpmBadge('@link-assistant/web-search', '1.2.3');
    expect(badge).toContain(
      'https://www.npmjs.com/package/@link-assistant/web-search/v/1.2.3'
    );
    expect(badge).toContain('img.shields.io');
  });

  it('strips a tag prefix before building the badge link', () => {
    const badge = buildNpmBadge('@link-assistant/web-search', 'js_v1.2.3');
    expect(badge).toContain('/v/1.2.3');
    expect(badge).not.toContain('js_v');
  });
});
