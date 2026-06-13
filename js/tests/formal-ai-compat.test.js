/**
 * FormalAI provider-parity contract.
 *
 * Asserts that `@link-assistant/web-search` is a superset of FormalAI's
 * `web_search_core` provider registry (issue #5). The expected FormalAI
 * surface lives in the shared compatibility map
 * `docs/case-studies/issue-5/formal-ai-compatibility.json`, which both this
 * JavaScript suite and the Rust `formal_ai_compat.rs` suite read so the two
 * languages enforce identical guarantees.
 */

import { describe, it, expect } from 'test-anywhere';
import { readFileSync } from 'node:fs';
import { dirname, join, resolve, basename } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  getRegistry,
  getDefaultProviderIds,
} from '../src/providers/registry.js';

const testDirectory = dirname(fileURLToPath(import.meta.url));
const packageOrProjectRoot = resolve(testDirectory, '..');
const projectRoot =
  basename(packageOrProjectRoot) === 'js'
    ? resolve(packageOrProjectRoot, '..')
    : packageOrProjectRoot;

const compat = JSON.parse(
  readFileSync(
    join(
      projectRoot,
      'docs',
      'case-studies',
      'issue-5',
      'formal-ai-compatibility.json'
    ),
    'utf8'
  )
);

const registry = getRegistry();
const byId = new Map(registry.map((entry) => [entry.id, entry]));
const corsException = new Map(
  compat.corsReadableExceptions.map((e) => [e.id, e])
);

describe('formal-ai provider parity', () => {
  it('registers every FormalAI provider id (superset requirement)', () => {
    for (const provider of compat.formalAiProviders) {
      expect(byId.has(provider.id)).toBe(true);
    }
  });

  it('matches FormalAI category for every shared provider', () => {
    for (const provider of compat.formalAiProviders) {
      const entry = byId.get(provider.id);
      expect(entry.category).toBe(provider.category);
    }
  });

  it('matches FormalAI default-for-category flags', () => {
    for (const provider of compat.formalAiProviders) {
      const entry = byId.get(provider.id);
      expect(entry.defaultForCategory).toBe(provider.defaultForCategory);
    }
  });

  it('matches FormalAI CORS-readability except for documented exceptions', () => {
    for (const provider of compat.formalAiProviders) {
      const entry = byId.get(provider.id);
      const exception = corsException.get(provider.id);
      const expected = exception ? exception.webSearch : provider.corsReadable;
      expect(entry.corsReadable).toBe(expected);
    }
  });

  it('uses the FormalAI live default plan as its own defaults', () => {
    expect(getDefaultProviderIds()).toEqual(compat.formalAiDefaultPlan);
  });

  it('every default-plan provider is registered and CORS-readable', () => {
    for (const id of compat.formalAiDefaultPlan) {
      const entry = byId.get(id);
      expect(Boolean(entry)).toBe(true);
      // DuckDuckGo is the documented HTML-scraper exception; the rest of the
      // knowledge sweep must be browser-CORS-readable.
      if (id !== 'duckduckgo') {
        expect(entry.corsReadable).toBe(true);
      }
    }
  });

  it('exactly one default provider per category', () => {
    for (const category of ['search', 'knowledge', 'papers', 'code']) {
      const defaults = registry.filter(
        (e) => e.category === category && e.defaultForCategory
      );
      expect(defaults.length).toBe(1);
    }
  });

  it('keeps the web-search-only providers outside the FormalAI surface', () => {
    const formalIds = new Set(compat.formalAiProviders.map((p) => p.id));
    for (const extra of compat.webSearchOnlyProviders) {
      if (extra.id === 'wc:*') {
        const wc = registry.filter((e) => e.id.startsWith('wc:'));
        expect(wc.length > 0).toBe(true);
        for (const entry of wc) {
          expect(entry.access).toBe('component');
        }
        continue;
      }
      expect(byId.has(extra.id)).toBe(true);
      expect(formalIds.has(extra.id)).toBe(false);
    }
  });
});
