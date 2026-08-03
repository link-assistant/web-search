/**
 * Integration tests for WebSearchEngine wired to the registry, using an
 * injected fetch so descriptor providers can be exercised end-to-end offline.
 */

import { describe, it, expect } from 'test-anywhere';
import { WebSearchEngine } from '../src/search.js';

/**
 * A fetch stub that routes by URL substring to canned JSON payloads.
 * @param {Object<string, Object>} routes - substring -> json payload
 * @returns {Function}
 */
function routedFetch(routes) {
  return async (url) => {
    for (const [needle, payload] of Object.entries(routes)) {
      if (url.includes(needle)) {
        return {
          ok: true,
          status: 200,
          json: async () => payload,
          text: async () => '',
        };
      }
    }
    return {
      ok: false,
      status: 404,
      json: async () => ({}),
      text: async () => '',
    };
  };
}

describe('WebSearchEngine + registry', () => {
  it('registers every catalog provider', () => {
    const engine = new WebSearchEngine();
    const providers = engine.getAvailableProviders();
    expect(providers.includes('wikipedia')).toBe(true);
    expect(providers.includes('github')).toBe(true);
    expect(providers.includes('arxiv')).toBe(true);
    expect(providers.includes('wc:google')).toBe(true);
  });

  it('enriches provider status with registry metadata', () => {
    const engine = new WebSearchEngine();
    const status = engine.getProviderStatus();
    expect(status.wikipedia.category).toBe('knowledge');
    expect(status.wikipedia.corsReadable).toBe(true);
    expect(status.github.category).toBe('code');
    expect(typeof status.wikipedia.label).toBe('string');
    expect(typeof status.wikipedia.access).toBe('string');
  });

  it('aggregates results from descriptor providers via injected fetch', async () => {
    const engine = new WebSearchEngine({
      fetchImpl: routedFetch({
        'wikipedia.org': {
          pages: [{ key: 'Cat', title: 'Cat', excerpt: 'A cat' }],
        },
        'api.github.com': {
          items: [
            {
              full_name: 'cat/cat',
              html_url: 'https://github.com/cat/cat',
              description: 'cats',
            },
          ],
        },
      }),
    });

    const results = await engine.search('cat', {
      providers: ['wikipedia', 'github'],
      limit: 5,
    });

    const urls = results.map((r) => r.url);
    expect(urls.includes('https://en.wikipedia.org/wiki/Cat')).toBe(true);
    expect(urls.includes('https://github.com/cat/cat')).toBe(true);
  });

  it('searchSingle targets one descriptor provider', async () => {
    const engine = new WebSearchEngine({
      fetchImpl: routedFetch({
        'api.crossref.org': {
          message: { items: [{ title: ['Paper'], DOI: '10.1/x' }] },
        },
      }),
    });
    const results = await engine.searchSingle('quantum', 'crossref', {
      limit: 3,
    });
    expect(results.length).toBe(1);
    expect(results[0].url).toBe('https://doi.org/10.1/x');
  });

  it('throws for an unknown provider in searchSingle', async () => {
    const engine = new WebSearchEngine();
    let threw = false;
    try {
      await engine.searchSingle('x', 'does-not-exist');
    } catch (error) {
      threw = error.message.includes('Unknown provider');
    }
    expect(threw).toBe(true);
  });

  it('isolates provider failures (one fails, others succeed)', async () => {
    const engine = new WebSearchEngine({
      fetchImpl: async (url) => {
        if (url.includes('wikipedia.org')) {
          throw new Error('wiki down');
        }
        if (url.includes('api.github.com')) {
          return {
            ok: true,
            json: async () => ({
              items: [{ full_name: 'a/b', html_url: 'https://github.com/a/b' }],
            }),
            text: async () => '',
          };
        }
        return {
          ok: false,
          status: 404,
          json: async () => ({}),
          text: async () => '',
        };
      },
    });

    const results = await engine.search('x', {
      providers: ['wikipedia', 'github'],
    });
    expect(results.map((r) => r.url).includes('https://github.com/a/b')).toBe(
      true
    );
  });

  it('returns transport captures and per-provider errors from detailed search', async () => {
    const calls = [];
    const controller = new AbortController();
    const transport = {
      async fetch(url, init) {
        calls.push({ url, init });
        if (url.includes('wikipedia.org')) {
          return new Response(
            JSON.stringify({
              pages: [{ key: 'Cat', title: 'Cat', excerpt: 'A cat' }],
            }),
            { status: 200, headers: { 'content-type': 'application/json' } }
          );
        }
        throw new Error('github transport failed');
      },
    };

    const engine = new WebSearchEngine();
    const detailed = await engine.searchDetailed('cat', {
      providers: ['wikipedia', 'github'],
      transport,
      signal: controller.signal,
    });

    expect(detailed.results.length).toBe(1);
    expect(detailed.outcomes[0].status).toBe('success');
    expect(detailed.outcomes[0].receipts.length).toBe(1);
    expect(
      new TextDecoder().decode(detailed.outcomes[0].receipts[0].body)
    ).toContain('"title":"Cat"');
    expect(detailed.outcomes[1].status).toBe('error');
    expect(detailed.outcomes[1].error.message).toContain(
      'github transport failed'
    );
    expect(calls.every((call) => call.init.signal === controller.signal)).toBe(
      true
    );
  });

  it('threads caller transport through class-based providers', async () => {
    const calls = [];
    const engine = new WebSearchEngine({
      providers: ['duckduckgo'],
      transport: async (url, init) => {
        calls.push({ url, init });
        return new Response(
          '<a class="result__a" href="https://example.com">Example</a>',
          { status: 200 }
        );
      },
    });
    const detailed = await engine.searchDetailed('x');
    expect(detailed.results[0].url).toBe('https://example.com');
    expect(calls.length).toBe(1);
  });

  it('reports caller cancellation as a provider error', async () => {
    const controller = new AbortController();
    controller.abort();
    const engine = new WebSearchEngine({
      transport: async (_url, init) => {
        if (init.signal.aborted) {
          throw new DOMException('cancelled', 'AbortError');
        }
      },
    });
    const detailed = await engine.searchDetailed('x', {
      providers: ['wikipedia'],
      signal: controller.signal,
    });
    expect(detailed.outcomes[0].status).toBe('error');
    expect(detailed.outcomes[0].error.name).toBe('AbortError');
  });

  it('returns [] for an empty query', async () => {
    const engine = new WebSearchEngine();
    expect((await engine.search('')).length).toBe(0);
  });
});
