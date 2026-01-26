/**
 * Web Search Engine
 * Main class for performing multi-provider web searches with merging and reranking
 */

import { GoogleProvider } from './providers/google.js';
import { DuckDuckGoProvider } from './providers/duckduckgo.js';
import { BingProvider } from './providers/bing.js';
import { mergeResults } from './merger.js';

/**
 * @typedef {import('./providers/base.js').SearchResult} SearchResult
 * @typedef {import('./providers/base.js').SearchOptions} SearchOptions
 * @typedef {import('./merger.js').MergeOptions} MergeOptions
 */

/**
 * @typedef {Object} WebSearchConfig
 * @property {string[]} [providers] - List of providers to use
 * @property {Object} [google] - Google provider configuration
 * @property {string} [google.apiKey] - Google API key
 * @property {string} [google.searchEngineId] - Google Custom Search Engine ID
 * @property {Object} [bing] - Bing provider configuration
 * @property {string} [bing.apiKey] - Bing API key
 * @property {Object<string, number>} [weights] - Weights for each provider
 * @property {'rrf' | 'weighted' | 'interleave'} [mergeStrategy] - Default merge strategy
 */

/**
 * @typedef {Object} SearchEngineOptions
 * @property {SearchOptions & MergeOptions} options
 * @property {string[]} [providers] - Override providers list
 */

/**
 * Web Search Engine class
 */
export class WebSearchEngine {
  /**
   * @param {WebSearchConfig} [config]
   */
  constructor(config = {}) {
    this.providers = new Map();
    this.defaultProviders = config.providers || [
      'duckduckgo',
      'google',
      'bing',
    ];
    this.defaultWeights = config.weights || {};
    this.defaultMergeStrategy = config.mergeStrategy || 'rrf';

    this.initializeProviders(config);
  }

  /**
   * Initialize search providers based on configuration
   * @param {WebSearchConfig} config
   */
  initializeProviders(config) {
    this.providers.set('google', new GoogleProvider(config.google));
    this.providers.set('duckduckgo', new DuckDuckGoProvider());
    this.providers.set('bing', new BingProvider(config.bing));

    for (const [name, weight] of Object.entries(this.defaultWeights)) {
      const provider = this.providers.get(name);
      if (provider) {
        provider.setWeight(weight);
      }
    }
  }

  /**
   * Get a provider by name
   * @param {string} name
   * @returns {import('./providers/base.js').BaseSearchProvider|undefined}
   */
  getProvider(name) {
    return this.providers.get(name.toLowerCase());
  }

  /**
   * Set provider weight
   * @param {string} name - Provider name
   * @param {number} weight - Weight value (0.0 to 1.0)
   */
  setProviderWeight(name, weight) {
    const provider = this.providers.get(name.toLowerCase());
    if (provider) {
      provider.setWeight(weight);
    }
  }

  /**
   * Enable or disable a provider
   * @param {string} name - Provider name
   * @param {boolean} enabled - Whether to enable the provider
   */
  setProviderEnabled(name, enabled) {
    const provider = this.providers.get(name.toLowerCase());
    if (provider) {
      provider.enabled = enabled;
    }
  }

  /**
   * Perform a search across multiple providers
   * @param {string} query - Search query
   * @param {Object} [options] - Search and merge options
   * @param {number} [options.limit] - Maximum results per provider
   * @param {string} [options.language] - Language code
   * @param {string} [options.region] - Region code
   * @param {boolean} [options.safeSearch] - Enable safe search
   * @param {string[]} [options.providers] - Providers to use
   * @param {'rrf' | 'weighted' | 'interleave'} [options.strategy] - Merge strategy
   * @param {Object<string, number>} [options.weights] - Provider weights
   * @returns {Promise<SearchResult[]>}
   */
  async search(query, options = {}) {
    if (!query || typeof query !== 'string' || query.trim().length === 0) {
      return [];
    }

    const providersToUse = options.providers || this.defaultProviders;
    const weights = options.weights || this.defaultWeights;
    const strategy = options.strategy || this.defaultMergeStrategy;

    const searchPromises = [];
    const providerNames = [];

    for (const name of providersToUse) {
      const provider = this.providers.get(name.toLowerCase());
      if (provider && provider.isAvailable()) {
        providerNames.push(name);
        searchPromises.push(
          provider.search(query, {
            limit: options.limit,
            language: options.language,
            region: options.region,
            safeSearch: options.safeSearch,
          })
        );
      }
    }

    const results = await Promise.allSettled(searchPromises);
    const resultsByProvider = {};

    for (let i = 0; i < results.length; i++) {
      const result = results[i];
      const providerName = providerNames[i];

      if (result.status === 'fulfilled' && Array.isArray(result.value)) {
        resultsByProvider[providerName] = result.value;
      } else {
        console.error(
          `Provider ${providerName} failed:`,
          result.status === 'rejected' ? result.reason : 'Invalid result'
        );
        resultsByProvider[providerName] = [];
      }
    }

    return mergeResults(resultsByProvider, {
      strategy,
      weights,
      removeDuplicates: true,
    });
  }

  /**
   * Search with a single provider only
   * @param {string} query - Search query
   * @param {string} providerName - Provider name
   * @param {SearchOptions} [options] - Search options
   * @returns {Promise<SearchResult[]>}
   */
  async searchSingle(query, providerName, options = {}) {
    const provider = this.providers.get(providerName.toLowerCase());
    if (!provider) {
      throw new Error(`Unknown provider: ${providerName}`);
    }

    if (!provider.isAvailable()) {
      throw new Error(`Provider ${providerName} is not available`);
    }

    return await provider.search(query, options);
  }

  /**
   * Get list of available provider names
   * @returns {string[]}
   */
  getAvailableProviders() {
    return Array.from(this.providers.keys());
  }

  /**
   * Get provider status information
   * @returns {Object<string, {enabled: boolean, weight: number, hasApi: boolean}>}
   */
  getProviderStatus() {
    const status = {};
    for (const [name, provider] of this.providers) {
      status[name] = {
        enabled: provider.enabled,
        weight: provider.getWeight(),
        hasApi:
          typeof provider.hasApiCredentials === 'function'
            ? provider.hasApiCredentials()
            : false,
      };
    }
    return status;
  }
}

/**
 * Create a default web search engine instance
 * @param {WebSearchConfig} [config]
 * @returns {WebSearchEngine}
 */
export function createSearchEngine(config) {
  return new WebSearchEngine(config);
}
