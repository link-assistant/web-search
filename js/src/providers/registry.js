/**
 * Typed provider registry.
 *
 * A single source of truth describing every search provider this library can
 * use, grouped into the four categories that `formal-ai` consumes
 * (`search`, `knowledge`, `papers`, `code`). The registry powers provider
 * discovery (CLI/server/`/providers`) and is the factory that instantiates the
 * correct provider implementation for each id.
 *
 * @module providers/registry
 */

import { GoogleProvider } from './google.js';
import { DuckDuckGoProvider } from './duckduckgo.js';
import { BingProvider } from './bing.js';
import { GenericProvider } from './generic.js';
import { WebCaptureProvider } from './web-capture.js';
import { API_ENGINES } from './api-engines.js';
import { HTML_ENGINES } from './html-engines.js';

/** Provider categories, mirroring `formal-ai`'s `web_search_core` registry. */
export const CATEGORIES = ['search', 'knowledge', 'papers', 'code'];

/**
 * @typedef {Object} RegistryEntry
 * @property {string} id - Stable provider id
 * @property {string} label - Human-readable label
 * @property {'search'|'knowledge'|'papers'|'code'} category - Provider category
 * @property {boolean} corsReadable - Whether the endpoint is browser-CORS readable
 * @property {boolean} defaultForCategory - Whether this is its category's default
 * @property {'api'|'html'|'hybrid'|'browser'|'component'} access - How results are obtained
 */

/**
 * Class-based providers that predate the descriptor catalog. They keep their
 * dedicated API + scraping fallback logic; the registry only carries metadata
 * and a factory for them.
 */
const CLASS_ENGINES = {
  google: {
    id: 'google',
    label: 'Google',
    category: 'search',
    corsReadable: false,
    defaultForCategory: false,
    access: 'hybrid',
    create: (config) => new GoogleProvider(config.google),
  },
  bing: {
    id: 'bing',
    label: 'Bing',
    category: 'search',
    corsReadable: false,
    defaultForCategory: false,
    access: 'hybrid',
    create: (config) => new BingProvider(config.bing),
  },
  duckduckgo: {
    id: 'duckduckgo',
    label: 'DuckDuckGo',
    category: 'search',
    corsReadable: false,
    defaultForCategory: true,
    access: 'html',
    create: () => new DuckDuckGoProvider(),
  },
};

/**
 * Descriptor-based engines (API + HTML), each instantiated with
 * {@link GenericProvider}. `access` is derived from the descriptor `kind`.
 */
const DESCRIPTOR_ENGINES = { ...API_ENGINES, ...HTML_ENGINES };

/**
 * The `web-capture` component-library provider. Delegates single-provider
 * fetch + parse to `@link-assistant/web-capture` (issue #3, R3). Registered
 * once per web-capture-supported engine via a `wc:<engine>` id namespace.
 */
const WEB_CAPTURE_ENGINES = WebCaptureProvider.SUPPORTED_PROVIDERS.map(
  (engine) => ({
    id: `wc:${engine}`,
    label: `web-capture (${engine})`,
    category: 'search',
    corsReadable: engine === 'wikipedia',
    defaultForCategory: false,
    access: 'component',
    create: (config) =>
      new WebCaptureProvider({ engine, fetchImpl: config.fetchImpl }),
  })
);

/**
 * Build the full registry of provider entries.
 *
 * @returns {RegistryEntry[]} All known providers with metadata
 */
export function getRegistry() {
  const entries = [];

  for (const e of Object.values(CLASS_ENGINES)) {
    entries.push(metaOf(e, e.access));
  }
  for (const d of Object.values(DESCRIPTOR_ENGINES)) {
    const access = d.kind === 'json' || d.kind === 'text' ? 'api' : 'html';
    entries.push(metaOf(d, access));
  }
  for (const w of WEB_CAPTURE_ENGINES) {
    entries.push(metaOf(w, w.access));
  }

  return entries;
}

/**
 * Project a descriptor down to its public registry metadata.
 *
 * @param {Object} e - Engine descriptor or class-engine entry
 * @param {string} access - Access mechanism label
 * @returns {RegistryEntry}
 */
function metaOf(e, access) {
  return {
    id: e.id,
    label: e.label,
    category: e.category,
    corsReadable: Boolean(e.corsReadable),
    defaultForCategory: Boolean(e.defaultForCategory),
    access,
  };
}

/**
 * Get all provider ids, optionally filtered by category.
 *
 * @param {string} [category] - One of {@link CATEGORIES}
 * @returns {string[]}
 */
export function getProviderIds(category) {
  return getRegistry()
    .filter((e) => !category || e.category === category)
    .map((e) => e.id);
}

/**
 * Get the default provider ids used when the caller does not specify providers.
 *
 * Mirrors FormalAI's live default plan (`WEB_SEARCH_PROVIDERS`): a
 * DuckDuckGo-first, CORS-readable knowledge sweep across the Wikimedia family
 * and the Internet Archive (issue #5 parity requirement).
 *
 * @returns {string[]}
 */
export function getDefaultProviderIds() {
  return [
    'duckduckgo',
    'internet-archive',
    'wikipedia',
    'wikidata',
    'wiktionary',
    'wikinews',
  ];
}

/**
 * Instantiate every registered provider.
 *
 * @param {Object} [config] - Engine configuration
 * @param {Object} [config.google] - Google provider config
 * @param {Object} [config.bing] - Bing provider config
 * @param {Function} [config.fetchImpl] - Injectable fetch (for tests)
 * @returns {Map<string, import('./base.js').BaseSearchProvider>}
 */
export function buildProviders(config = {}) {
  const providers = new Map();

  for (const e of Object.values(CLASS_ENGINES)) {
    providers.set(e.id, e.create(config));
  }
  for (const d of Object.values(DESCRIPTOR_ENGINES)) {
    providers.set(
      d.id,
      new GenericProvider(d, { fetchImpl: config.fetchImpl })
    );
  }
  for (const w of WEB_CAPTURE_ENGINES) {
    providers.set(w.id, w.create(config));
  }

  return providers;
}
