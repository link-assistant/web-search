/**
 * Unit tests for the typed provider registry.
 */

import { describe, it, expect } from 'test-anywhere';
import {
  CATEGORIES,
  getRegistry,
  getProviderIds,
  getDefaultProviderIds,
  buildProviders,
} from '../src/providers/registry.js';
import { BaseSearchProvider } from '../src/providers/base.js';

describe('registry', () => {
  it('declares the four formal-ai categories', () => {
    expect(CATEGORIES).toEqual(['search', 'knowledge', 'papers', 'code']);
  });

  it('builds a registry covering every category', () => {
    const registry = getRegistry();
    expect(registry.length > 15).toBe(true);
    for (const category of CATEGORIES) {
      const inCategory = registry.filter((e) => e.category === category);
      expect(inCategory.length > 0).toBe(true);
    }
  });

  it('exposes 10+ search-category engines (top 10-20 coverage)', () => {
    const searchEngines = getProviderIds('search');
    expect(searchEngines.length >= 10).toBe(true);
  });

  it('includes knowledge, papers, and code engines', () => {
    expect(getProviderIds('knowledge').includes('wikipedia')).toBe(true);
    expect(getProviderIds('papers').includes('arxiv')).toBe(true);
    expect(getProviderIds('code').includes('github')).toBe(true);
  });

  it('registers web-capture-backed providers', () => {
    const ids = getProviderIds();
    expect(ids.includes('wc:wikipedia')).toBe(true);
    expect(ids.includes('wc:google')).toBe(true);
  });

  it('every entry has well-formed metadata', () => {
    for (const entry of getRegistry()) {
      expect(typeof entry.id).toBe('string');
      expect(typeof entry.label).toBe('string');
      expect(CATEGORIES.includes(entry.category)).toBe(true);
      expect(typeof entry.corsReadable).toBe('boolean');
      expect(typeof entry.defaultForCategory).toBe('boolean');
      expect(typeof entry.access).toBe('string');
    }
  });

  it('ids are unique', () => {
    const ids = getProviderIds();
    expect(new Set(ids).size).toBe(ids.length);
  });

  it('returns sensible defaults spanning search + knowledge', () => {
    const defaults = getDefaultProviderIds();
    expect(defaults.includes('duckduckgo')).toBe(true);
    expect(defaults.includes('wikipedia')).toBe(true);
  });

  it('default ids all exist in the registry', () => {
    const ids = new Set(getProviderIds());
    for (const id of getDefaultProviderIds()) {
      expect(ids.has(id)).toBe(true);
    }
  });

  describe('buildProviders', () => {
    it('instantiates every registered provider', () => {
      const providers = buildProviders();
      expect(providers.size).toBe(getProviderIds().length);
      for (const provider of providers.values()) {
        expect(provider instanceof BaseSearchProvider).toBe(true);
      }
    });

    it('passes google/bing config to class providers', () => {
      const providers = buildProviders({
        google: { apiKey: 'k', searchEngineId: 'cx' },
        bing: { apiKey: 'bk' },
      });
      expect(providers.get('google').hasApiCredentials()).toBe(true);
      expect(providers.get('bing').hasApiCredentials()).toBe(true);
    });

    it('threads fetchImpl into descriptor providers', () => {
      const fetchImpl = async () => ({ ok: true, json: async () => ({}) });
      const providers = buildProviders({ fetchImpl });
      expect(providers.get('wikipedia').fetchImpl).toBe(fetchImpl);
    });
  });
});
