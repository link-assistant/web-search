/**
 * Unit tests for web-search library
 * Works with Node.js, Bun, and Deno
 */

import { describe, it, expect } from 'test-anywhere';
import {
  mergeWithRRF,
  mergeWithWeights,
  mergeWithInterleave,
  mergeResults,
} from '../src/merger.js';

describe('merger', () => {
  const createResult = (url, title, source, rank) => ({
    title,
    url,
    snippet: `Snippet for ${title}`,
    source,
    rank,
  });

  describe('mergeWithRRF', () => {
    it('should merge results from multiple providers', () => {
      const resultsByProvider = {
        google: [
          createResult('https://example.com/1', 'Result 1', 'google', 1),
          createResult('https://example.com/2', 'Result 2', 'google', 2),
        ],
        bing: [
          createResult('https://example.com/2', 'Result 2', 'bing', 1),
          createResult('https://example.com/3', 'Result 3', 'bing', 2),
        ],
      };

      const merged = mergeWithRRF(resultsByProvider);

      expect(merged.length).toBe(3);
      expect(merged[0].url.includes('example.com/2')).toBe(true);
      expect(merged[0].sources !== undefined).toBe(true);
      expect(merged[0].sources.length).toBe(2);
    });

    it('should handle empty results', () => {
      const merged = mergeWithRRF({});
      expect(merged.length).toBe(0);
    });

    it('should apply provider weights', () => {
      const resultsByProvider = {
        google: [createResult('https://a.com', 'A', 'google', 1)],
        bing: [createResult('https://b.com', 'B', 'bing', 1)],
      };

      const merged = mergeWithRRF(resultsByProvider, {
        weights: { google: 2.0, bing: 0.5 },
      });

      expect(merged[0].url).toBe('https://a.com');
    });

    it('should deduplicate URLs by normalized form', () => {
      const resultsByProvider = {
        google: [createResult('https://example.com/path/', 'R1', 'google', 1)],
        bing: [createResult('https://EXAMPLE.COM/path', 'R1', 'bing', 1)],
      };

      const merged = mergeWithRRF(resultsByProvider);
      expect(merged.length).toBe(1);
    });
  });

  describe('mergeWithWeights', () => {
    it('should merge using weighted scoring', () => {
      const resultsByProvider = {
        google: [
          createResult('https://a.com', 'A', 'google', 1),
          createResult('https://b.com', 'B', 'google', 2),
        ],
        bing: [createResult('https://b.com', 'B', 'bing', 1)],
      };

      const merged = mergeWithWeights(resultsByProvider);

      expect(merged.length).toBe(2);
      expect(merged[0].score !== undefined).toBe(true);
    });

    it('should respect weight overrides', () => {
      const resultsByProvider = {
        google: [createResult('https://a.com', 'A', 'google', 1)],
        bing: [createResult('https://b.com', 'B', 'bing', 1)],
      };

      const merged = mergeWithWeights(resultsByProvider, {
        weights: { bing: 2.0, google: 0.1 },
      });

      expect(merged[0].url).toBe('https://b.com');
    });
  });

  describe('mergeWithInterleave', () => {
    it('should interleave results round-robin style', () => {
      const resultsByProvider = {
        google: [
          createResult('https://g1.com', 'G1', 'google', 1),
          createResult('https://g2.com', 'G2', 'google', 2),
        ],
        bing: [
          createResult('https://b1.com', 'B1', 'bing', 1),
          createResult('https://b2.com', 'B2', 'bing', 2),
        ],
      };

      const merged = mergeWithInterleave(resultsByProvider);

      expect(merged.length).toBe(4);
    });

    it('should remove duplicates when enabled', () => {
      const resultsByProvider = {
        google: [createResult('https://same.com', 'Same', 'google', 1)],
        bing: [createResult('https://same.com', 'Same', 'bing', 1)],
      };

      const merged = mergeWithInterleave(resultsByProvider, {
        removeDuplicates: true,
      });

      expect(merged.length).toBe(1);
    });
  });

  describe('mergeResults', () => {
    it('should use RRF strategy by default', () => {
      const resultsByProvider = {
        google: [createResult('https://a.com', 'A', 'google', 1)],
      };

      const merged = mergeResults(resultsByProvider);
      expect(merged.length).toBe(1);
    });

    it('should use weighted strategy when specified', () => {
      const resultsByProvider = {
        google: [createResult('https://a.com', 'A', 'google', 1)],
      };

      const merged = mergeResults(resultsByProvider, { strategy: 'weighted' });
      expect(merged[0].score !== undefined).toBe(true);
    });

    it('should use interleave strategy when specified', () => {
      const resultsByProvider = {
        google: [createResult('https://a.com', 'A', 'google', 1)],
        bing: [createResult('https://b.com', 'B', 'bing', 1)],
      };

      const merged = mergeResults(resultsByProvider, {
        strategy: 'interleave',
      });
      expect(merged.length).toBe(2);
    });
  });
});

describe('providers', () => {
  describe('BaseSearchProvider', () => {
    it('should export BaseSearchProvider', async () => {
      const { BaseSearchProvider } = await import('../src/providers/base.js');
      expect(BaseSearchProvider !== undefined).toBe(true);
    });
  });

  describe('GoogleProvider', () => {
    it('should create GoogleProvider instance', async () => {
      const { GoogleProvider } = await import('../src/providers/google.js');
      const provider = new GoogleProvider();

      expect(provider.getName()).toBe('google');
      expect(provider.isAvailable()).toBe(true);
    });

    it('should report API credentials status', async () => {
      const { GoogleProvider } = await import('../src/providers/google.js');

      const withoutApi = new GoogleProvider();
      expect(withoutApi.hasApiCredentials()).toBe(false);

      const withApi = new GoogleProvider({
        apiKey: 'test-key',
        searchEngineId: 'test-cx',
      });
      expect(withApi.hasApiCredentials()).toBe(true);
    });
  });

  describe('DuckDuckGoProvider', () => {
    it('should create DuckDuckGoProvider instance', async () => {
      const { DuckDuckGoProvider } =
        await import('../src/providers/duckduckgo.js');
      const provider = new DuckDuckGoProvider();

      expect(provider.getName()).toBe('duckduckgo');
      expect(provider.isAvailable()).toBe(true);
    });
  });

  describe('BingProvider', () => {
    it('should create BingProvider instance', async () => {
      const { BingProvider } = await import('../src/providers/bing.js');
      const provider = new BingProvider();

      expect(provider.getName()).toBe('bing');
      expect(provider.isAvailable()).toBe(true);
    });

    it('should report API credentials status', async () => {
      const { BingProvider } = await import('../src/providers/bing.js');

      const withoutApi = new BingProvider();
      expect(withoutApi.hasApiCredentials()).toBe(false);

      const withApi = new BingProvider({ apiKey: 'test-key' });
      expect(withApi.hasApiCredentials()).toBe(true);
    });
  });
});

describe('search engine', () => {
  it('should create WebSearchEngine instance', async () => {
    const { WebSearchEngine } = await import('../src/search.js');
    const engine = new WebSearchEngine();

    expect(engine.getAvailableProviders().includes('google')).toBe(true);
    expect(engine.getAvailableProviders().includes('duckduckgo')).toBe(true);
    expect(engine.getAvailableProviders().includes('bing')).toBe(true);
  });

  it('should get provider status', async () => {
    const { WebSearchEngine } = await import('../src/search.js');
    const engine = new WebSearchEngine();

    const status = engine.getProviderStatus();

    expect(status.google !== undefined).toBe(true);
    expect(status.google.enabled).toBe(true);
    expect(typeof status.google.weight).toBe('number');
  });

  it('should allow setting provider weight', async () => {
    const { WebSearchEngine } = await import('../src/search.js');
    const engine = new WebSearchEngine();

    engine.setProviderWeight('google', 0.5);
    const status = engine.getProviderStatus();

    expect(status.google.weight).toBe(0.5);
  });

  it('should return empty array for empty query', async () => {
    const { WebSearchEngine } = await import('../src/search.js');
    const engine = new WebSearchEngine();

    const results = await engine.search('');
    expect(results.length).toBe(0);
  });
});

describe('exports', () => {
  it('should export all main components', async () => {
    const exports = await import('../src/index.js');

    expect(exports.WebSearchEngine !== undefined).toBe(true);
    expect(exports.createSearchEngine !== undefined).toBe(true);
    expect(exports.BaseSearchProvider !== undefined).toBe(true);
    expect(exports.GoogleProvider !== undefined).toBe(true);
    expect(exports.DuckDuckGoProvider !== undefined).toBe(true);
    expect(exports.BingProvider !== undefined).toBe(true);
    expect(exports.mergeResults !== undefined).toBe(true);
    expect(exports.mergeWithRRF !== undefined).toBe(true);
    expect(exports.mergeWithWeights !== undefined).toBe(true);
    expect(exports.mergeWithInterleave !== undefined).toBe(true);
  });
});
