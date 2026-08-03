/**
 * Generic, descriptor-driven search provider.
 *
 * A single provider implementation that can speak to any engine described by a
 * catalog descriptor (see {@link module:providers/api-engines} and
 * {@link module:providers/html-engines}). This keeps the fetch/normalize/error
 * plumbing in one place while each engine only declares its URL, request kind,
 * and parser.
 *
 * @module providers/generic
 */

import { BaseSearchProvider } from './base.js';
import { request } from '../transport.js';

const USER_AGENT =
  'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 ' +
  '(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36';

/**
 * @typedef {Object} EngineDescriptor
 * @property {string} id - Stable provider id
 * @property {string} label - Human-readable label
 * @property {'search'|'knowledge'|'papers'|'code'} category - Provider category
 * @property {'json'|'text'|'html'} kind - How the response body is decoded
 * @property {boolean} [corsReadable] - Whether the endpoint is browser-CORS readable
 * @property {boolean} [defaultForCategory] - Default provider for its category
 * @property {'GET'|'POST'} [method] - HTTP method (default GET)
 * @property {(query: string, options: Object) => string} buildUrl - Build request URL
 * @property {(query: string, options: Object) => string} [buildBody] - Build POST body
 * @property {(options: Object) => Object} [headers] - Extra request headers
 * @property {(payload: *, limit: number, options: Object) => import('./base.js').SearchResult[]} parse - Parse response
 */

/**
 * Descriptor-driven provider.
 *
 * @extends BaseSearchProvider
 */
export class GenericProvider extends BaseSearchProvider {
  /**
   * @param {EngineDescriptor} descriptor - Engine descriptor
   * @param {Object} [config] - Provider configuration
   * @param {Function} [config.fetchImpl] - Injectable fetch implementation (for tests)
   */
  constructor(descriptor, config = {}) {
    super(descriptor.id);
    this.descriptor = descriptor;
    this.category = descriptor.category;
    this.label = descriptor.label;
    this.corsReadable = Boolean(descriptor.corsReadable);
    this.transport = config.transport || config.fetchImpl;
    this.fetchImpl = this.transport || globalThis.fetch;
  }

  /**
   * Perform a search against the descriptor's engine.
   *
   * @param {string} query - Search query
   * @param {import('./base.js').SearchOptions} [options] - Search options
   * @returns {Promise<import('./base.js').SearchResult[]>}
   */
  async search(query, options = {}) {
    if (!query || typeof query !== 'string' || query.trim().length === 0) {
      return [];
    }

    const limit = options.limit || 10;
    const d = this.descriptor;
    const url = d.buildUrl(query, options);
    const method = d.method || 'GET';

    const headers = {
      'User-Agent': USER_AGENT,
      'Accept-Language': 'en-US,en;q=0.9',
      ...(d.kind === 'json'
        ? { Accept: 'application/json' }
        : { Accept: 'text/html,application/xhtml+xml,application/xml;q=0.9' }),
      ...(typeof d.headers === 'function' ? d.headers(options) : {}),
    };

    const init = { method, headers };
    if (method === 'POST' && typeof d.buildBody === 'function') {
      init.body = d.buildBody(query, options);
      headers['Content-Type'] = 'application/x-www-form-urlencoded';
    }

    try {
      const response = await request(this.transport, url, init, options);
      if (!response.ok) {
        throw new Error(`${d.id} returned status ${response.status}`);
      }
      const payload =
        d.kind === 'json' ? await response.json() : await response.text();
      return d.parse(payload, limit, options);
    } catch (error) {
      if (options.throwOnError) {
        throw error;
      }
      console.error(`${d.id} search error: ${error.message}`);
      return [];
    }
  }
}

/**
 * Create a {@link GenericProvider} from a descriptor.
 *
 * @param {EngineDescriptor} descriptor - Engine descriptor
 * @param {Object} [config] - Provider configuration
 * @returns {GenericProvider}
 */
export function createGenericProvider(descriptor, config) {
  return new GenericProvider(descriptor, config);
}
