/**
 * Integration tests for the Express REST API. The server is bound to an
 * ephemeral port and exercised over real HTTP. Only metadata and validation
 * paths are tested so the suite stays offline (no live search requests).
 */

import { describe, it, expect } from 'test-anywhere';
import { app } from '../src/server.js';

/**
 * Start the app on an ephemeral port and return a helper bound to it.
 * @returns {Promise<{base: string, close: Function}>}
 */
function listen() {
  return new Promise((resolve) => {
    const server = app.listen(0, () => {
      const { port } = server.address();
      resolve({
        base: `http://127.0.0.1:${port}`,
        close: () => new Promise((r) => server.close(r)),
      });
    });
  });
}

describe('REST API', () => {
  it('GET /health reports provider status', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/health`);
      const body = await res.json();
      expect(res.status).toBe(200);
      expect(body.status).toBe('healthy');
      expect(typeof body.providers.wikipedia.category).toBe('string');
    } finally {
      await close();
    }
  });

  it('GET /categories lists providers per category', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/categories`);
      const body = await res.json();
      expect(res.status).toBe(200);
      expect(body.categories.knowledge.includes('wikipedia')).toBe(true);
      expect(body.categories.code.includes('github')).toBe(true);
      expect(body.categories.papers.includes('arxiv')).toBe(true);
    } finally {
      await close();
    }
  });

  it('GET /providers returns the full registry', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/providers`);
      const body = await res.json();
      expect(res.status).toBe(200);
      expect(body.categories).toEqual([
        'search',
        'knowledge',
        'papers',
        'code',
      ]);
      expect(body.count > 15).toBe(true);
      expect(Array.isArray(body.registry)).toBe(true);
    } finally {
      await close();
    }
  });

  it('GET /providers?category=code filters by category', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/providers?category=code`);
      const body = await res.json();
      expect(res.status).toBe(200);
      expect(body.registry.every((e) => e.category === 'code')).toBe(true);
      expect(Object.keys(body.providers).includes('github')).toBe(true);
    } finally {
      await close();
    }
  });

  it('GET /providers rejects an unknown category', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/providers?category=nope`);
      const body = await res.json();
      expect(res.status).toBe(400);
      expect(body.error.includes('Unknown category')).toBe(true);
    } finally {
      await close();
    }
  });

  it('GET /search without a query returns 400', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/search`);
      const body = await res.json();
      expect(res.status).toBe(400);
      expect(body.error.includes('Missing required parameter')).toBe(true);
    } finally {
      await close();
    }
  });

  it('GET /search rejects invalid weights JSON', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/search?q=x&weights=not-json`);
      const body = await res.json();
      expect(res.status).toBe(400);
      expect(body.error.includes('Invalid weights')).toBe(true);
    } finally {
      await close();
    }
  });

  it('POST /search without a query returns 400', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/search`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({}),
      });
      const body = await res.json();
      expect(res.status).toBe(400);
      expect(body.error.includes('Missing required parameter')).toBe(true);
    } finally {
      await close();
    }
  });

  it('GET /search/:provider rejects an unknown provider', async () => {
    const { base, close } = await listen();
    try {
      const res = await fetch(`${base}/search/does-not-exist?q=cats`);
      const body = await res.json();
      expect(res.status).toBe(400);
      expect(body.error.includes('Unknown provider')).toBe(true);
      expect(Array.isArray(body.availableProviders)).toBe(true);
    } finally {
      await close();
    }
  });
});
