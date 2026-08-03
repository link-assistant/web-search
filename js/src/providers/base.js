/**
 * Base search provider class
 * All search providers must implement this interface
 */

/**
 * @typedef {Object} SearchResult
 * @property {string} title - The title of the search result
 * @property {string} url - The URL of the search result
 * @property {string} snippet - The description/snippet of the search result
 * @property {string} source - The search provider that returned this result
 * @property {number} [rank] - The rank position in the original results
 */

/**
 * @typedef {Object} SearchOptions
 * @property {number} [limit] - Maximum number of results to return
 * @property {string} [language] - Language code (e.g., 'en', 'de')
 * @property {string} [region] - Region code (e.g., 'us', 'de')
 * @property {boolean} [safeSearch] - Enable safe search filtering
 * @property {AbortSignal} [signal] - Caller cancellation signal
 * @property {Function|{fetch: Function}} [transport] - Per-search caller-owned transport
 * @property {(receipt: unknown) => void} [captureResponse] - Response capture sink
 * @property {boolean} [throwOnError] - Preserve provider errors for detailed outcomes
 */

/**
 * Base class for search providers
 * @abstract
 */
export class BaseSearchProvider {
  /**
   * @param {string} name - The name of the search provider
   */
  constructor(name) {
    if (new.target === BaseSearchProvider) {
      throw new Error('BaseSearchProvider is an abstract class');
    }
    this.name = name;
    this.enabled = true;
    this.weight = 1.0;
  }

  /**
   * Search for results using this provider
   * @abstract
   * @param {string} query - The search query
   * @param {SearchOptions} [options] - Search options
   * @returns {Promise<SearchResult[]>} - Array of search results
   */
  // eslint-disable-next-line require-await -- Abstract method, implementations will use await
  async search() {
    throw new Error('search() must be implemented by subclass');
  }

  /**
   * Check if the provider is available/configured
   * @returns {boolean}
   */
  isAvailable() {
    return this.enabled;
  }

  /**
   * Get the provider name
   * @returns {string}
   */
  getName() {
    return this.name;
  }

  /**
   * Get the provider weight for reranking
   * @returns {number}
   */
  getWeight() {
    return this.weight;
  }

  /**
   * Set the provider weight for reranking
   * @param {number} weight - Weight value (0.0 to 1.0)
   */
  setWeight(weight) {
    this.weight = Math.max(0, Math.min(1, weight));
  }
}
