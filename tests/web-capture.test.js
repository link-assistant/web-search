/**
 * Unit tests for the web-capture component-library provider, using an injected
 * `searchImpl` so behavior is verified without the optional dependency or the
 * network.
 */

import { describe, it, expect } from 'test-anywhere';
import {
  WebCaptureProvider,
  createWebCaptureProvider,
} from '../src/providers/web-capture.js';

describe('WebCaptureProvider', () => {
  it('namespaces its provider name as wc:<engine>', () => {
    const provider = new WebCaptureProvider({ engine: 'google' });
    expect(provider.getName()).toBe('wc:google');
    expect(provider.engine).toBe('google');
  });

  it('defaults to the wikipedia engine', () => {
    const provider = new WebCaptureProvider();
    expect(provider.getName()).toBe('wc:wikipedia');
  });

  it('lists the web-capture supported providers', () => {
    expect(WebCaptureProvider.SUPPORTED_PROVIDERS.includes('wikipedia')).toBe(
      true
    );
    expect(WebCaptureProvider.SUPPORTED_PROVIDERS.includes('brave')).toBe(true);
  });

  it('delegates to the injected searchImpl and adapts results', async () => {
    const calls = [];
    const searchImpl = async (args) => {
      calls.push(args);
      return {
        results: [
          { title: 'WC', url: 'https://wc.example', snippet: 's', rank: 1 },
          { url: 'https://wc.example/2' },
        ],
      };
    };
    const provider = new WebCaptureProvider({ engine: 'bing', searchImpl });

    const results = await provider.search('cats', { limit: 7 });
    expect(calls[0].provider).toBe('bing');
    expect(calls[0].query).toBe('cats');
    expect(calls[0].limit).toBe(7);
    expect(results.length).toBe(2);
    expect(results[0].source).toBe('wc:bing');
    expect(results[1].title).toBe('Untitled');
    expect(results[1].rank).toBe(2);
  });

  it('passes an injected fetchImpl through to web-capture', async () => {
    const calls = [];
    const fetchImpl = async () => ({ ok: true });
    const searchImpl = async (args) => {
      calls.push(args);
      return { results: [] };
    };
    const provider = new WebCaptureProvider({
      engine: 'google',
      fetchImpl,
      searchImpl,
    });
    await provider.search('x');
    expect(calls[0].fetchImpl).toBe(fetchImpl);
  });

  it('returns [] for empty queries', async () => {
    const provider = new WebCaptureProvider({
      searchImpl: async () => ({ results: [{ url: 'x' }] }),
    });
    expect((await provider.search('')).length).toBe(0);
  });

  it('returns [] when searchImpl throws', async () => {
    const provider = new WebCaptureProvider({
      searchImpl: async () => {
        throw new Error('boom');
      },
    });
    expect((await provider.search('x')).length).toBe(0);
  });

  it('handles malformed web-capture payloads gracefully', async () => {
    const provider = new WebCaptureProvider({
      searchImpl: async () => ({}),
    });
    expect((await provider.search('x')).length).toBe(0);
  });

  it('createWebCaptureProvider builds an instance', () => {
    expect(
      createWebCaptureProvider({ engine: 'brave' }) instanceof
        WebCaptureProvider
    ).toBe(true);
  });
});
