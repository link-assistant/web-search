/**
 * web-capture component-library provider.
 *
 * Issue #3 (R3/R4) asks `web-search` to use `@link-assistant/web-capture` as a
 * component library so this project can focus on the search-aggregation API
 * rather than re-implementing per-provider scraping. This provider delegates a
 * single-provider fetch + parse to web-capture's normalized `search()`
 * contract and adapts the result to this library's {@link SearchResult} shape.
 *
 * `@link-assistant/web-capture` is an optional dependency: it is imported
 * dynamically so the core package stays lightweight, and the provider degrades
 * gracefully (returns `[]` and warns once) when it is not installed. A custom
 * `searchImpl` can be injected for deterministic testing without the dependency
 * or the network.
 *
 * @module providers/web-capture
 */

import { BaseSearchProvider } from './base.js';
import { request } from '../transport.js';

/** Providers exposed by web-capture's search contract. */
const SUPPORTED = ['wikipedia', 'duckduckgo', 'google', 'bing', 'brave'];

let cachedModule;
let warned = false;

/**
 * Lazily import the web-capture search function. Returns `null` when the
 * optional dependency is not installed.
 *
 * @returns {Promise<Function|null>} web-capture `search` or null
 */
async function loadWebCaptureSearch() {
  if (cachedModule !== undefined) {
    return cachedModule;
  }
  try {
    const mod = await import('@link-assistant/web-capture');
    cachedModule = typeof mod.search === 'function' ? mod.search : null;
  } catch {
    cachedModule = null;
  }
  return cachedModule;
}

/**
 * Provider that delegates to the web-capture component library.
 *
 * @extends BaseSearchProvider
 */
export class WebCaptureProvider extends BaseSearchProvider {
  /** Providers understood by web-capture's search contract. */
  static SUPPORTED_PROVIDERS = SUPPORTED;

  /**
   * @param {Object} [config]
   * @param {string} [config.engine] - web-capture provider id (default 'wikipedia')
   * @param {Function} [config.fetchImpl] - Injectable fetch passed to web-capture
   * @param {Function} [config.searchImpl] - Injectable web-capture `search` (for tests)
   */
  constructor(config = {}) {
    super(`wc:${config.engine || 'wikipedia'}`);
    this.engine = config.engine || 'wikipedia';
    this.fetchImpl = config.fetchImpl;
    this.transport = config.transport || config.fetchImpl;
    this.searchImpl = config.searchImpl;
  }

  /**
   * Perform a search via web-capture.
   *
   * @param {string} query - Search query
   * @param {import('./base.js').SearchOptions} [options] - Search options
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async search(query, options = {}) {
    if (!query || typeof query !== 'string' || query.trim().length === 0) {
      return [];
    }

    const search = this.searchImpl || (await loadWebCaptureSearch());
    if (!search) {
      if (!warned) {
        warned = true;
        console.warn(
          'WebCaptureProvider: @link-assistant/web-capture is not installed; ' +
            'install it to enable web-capture-backed providers.'
        );
      }
      return [];
    }

    try {
      const transport = options.transport || this.transport;
      const fetchImpl = (url, init = {}) =>
        request(this.transport, url, init, {
          ...options,
          transport,
        });
      const result = await search({
        query,
        provider: this.engine,
        limit: options.limit || 10,
        fetchImpl,
        ...(options.signal ? { signal: options.signal } : {}),
      });
      return this.adapt(result);
    } catch (error) {
      if (options.throwOnError) {
        throw error;
      }
      console.error(
        `WebCaptureProvider (${this.engine}) error: ${error.message}`
      );
      return [];
    }
  }

  /**
   * Adapt a web-capture normalized result to {@link SearchResult}[].
   *
   * @param {Object} result - web-capture search result
   * @returns {import('./base.js').SearchResult[]}
   */
  adapt(result) {
    const items = Array.isArray(result?.results) ? result.results : [];
    return items.map((item, index) => ({
      title: item.title || 'Untitled',
      url: item.url,
      snippet: item.snippet || '',
      source: this.name,
      rank: item.rank || index + 1,
    }));
  }
}

/**
 * Create a {@link WebCaptureProvider}.
 *
 * @param {Object} [config] - Provider configuration
 * @returns {WebCaptureProvider}
 */
export function createWebCaptureProvider(config) {
  return new WebCaptureProvider(config);
}
