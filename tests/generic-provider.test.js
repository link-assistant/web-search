/**
 * Unit tests for the descriptor-driven GenericProvider, using an injected fetch
 * so behavior is verified without real network calls.
 */

import { describe, it, expect } from 'test-anywhere';
import { GenericProvider } from '../src/providers/generic.js';

/**
 * Build a fake fetch returning a fixed payload.
 * @param {Object} opts
 * @returns {{fetch: Function, calls: Array}}
 */
function fakeFetch({ ok = true, status = 200, json, text } = {}) {
  const calls = [];
  const fetch = async (url, init) => {
    calls.push({ url, init });
    return {
      ok,
      status,
      json: async () => json,
      text: async () => text,
    };
  };
  return { fetch, calls };
}

const jsonDescriptor = {
  id: 'demo',
  label: 'Demo',
  category: 'search',
  kind: 'json',
  buildUrl: (query) => `https://demo.example/?q=${encodeURIComponent(query)}`,
  parse: (data, limit) =>
    (data.items || []).slice(0, limit).map((it, i) => ({
      title: it.title,
      url: it.url,
      snippet: it.snippet || '',
      source: 'demo',
      rank: i + 1,
    })),
};

describe('GenericProvider', () => {
  it('fetches and parses a JSON engine', async () => {
    const { fetch, calls } = fakeFetch({
      json: { items: [{ title: 'T', url: 'https://t.example' }] },
    });
    const provider = new GenericProvider(jsonDescriptor, { fetchImpl: fetch });

    const results = await provider.search('hello', { limit: 5 });
    expect(results.length).toBe(1);
    expect(results[0].url).toBe('https://t.example');
    expect(calls[0].url).toBe('https://demo.example/?q=hello');
    expect(calls[0].init.headers.Accept).toBe('application/json');
  });

  it('exposes registry-style metadata', () => {
    const provider = new GenericProvider(jsonDescriptor);
    expect(provider.getName()).toBe('demo');
    expect(provider.category).toBe('search');
    expect(provider.label).toBe('Demo');
  });

  it('returns [] for empty queries', async () => {
    const provider = new GenericProvider(jsonDescriptor);
    expect((await provider.search('')).length).toBe(0);
    expect((await provider.search('   ')).length).toBe(0);
  });

  it('returns [] and swallows non-ok responses', async () => {
    const { fetch } = fakeFetch({ ok: false, status: 503 });
    const provider = new GenericProvider(jsonDescriptor, { fetchImpl: fetch });
    expect((await provider.search('x')).length).toBe(0);
  });

  it('returns [] when fetch throws', async () => {
    const provider = new GenericProvider(jsonDescriptor, {
      fetchImpl: async () => {
        throw new Error('network down');
      },
    });
    expect((await provider.search('x')).length).toBe(0);
  });

  it('decodes text bodies for html/text engines', async () => {
    const textDescriptor = {
      id: 'txt',
      label: 'Txt',
      category: 'search',
      kind: 'text',
      buildUrl: () => 'https://txt.example/',
      parse: (body) => [
        {
          title: body,
          url: 'https://txt.example',
          snippet: '',
          source: 'txt',
          rank: 1,
        },
      ],
    };
    const { fetch } = fakeFetch({ text: 'raw body' });
    const provider = new GenericProvider(textDescriptor, { fetchImpl: fetch });
    const results = await provider.search('x');
    expect(results[0].title).toBe('raw body');
  });

  it('sends a POST body when the descriptor declares one', async () => {
    const postDescriptor = {
      id: 'post',
      label: 'Post',
      category: 'search',
      kind: 'html',
      method: 'POST',
      buildUrl: () => 'https://post.example/',
      buildBody: (query) => `q=${query}`,
      parse: () => [],
    };
    const { fetch, calls } = fakeFetch({ text: '' });
    const provider = new GenericProvider(postDescriptor, { fetchImpl: fetch });
    await provider.search('cats');
    expect(calls[0].init.method).toBe('POST');
    expect(calls[0].init.body).toBe('q=cats');
    expect(calls[0].init.headers['Content-Type']).toBe(
      'application/x-www-form-urlencoded'
    );
  });

  it('merges descriptor-provided headers', async () => {
    const headerDescriptor = {
      ...jsonDescriptor,
      id: 'hdr',
      headers: () => ({ Authorization: 'Bearer x' }),
    };
    const { fetch, calls } = fakeFetch({ json: { items: [] } });
    const provider = new GenericProvider(headerDescriptor, {
      fetchImpl: fetch,
    });
    await provider.search('x');
    expect(calls[0].init.headers.Authorization).toBe('Bearer x');
  });
});
