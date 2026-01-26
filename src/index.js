/**
 * Web Search - Multi-provider web search aggregator
 *
 * A microservice that aggregates search results from multiple search engines
 * (Google, DuckDuckGo, Bing) with support for result merging and reranking.
 */

export { WebSearchEngine, createSearchEngine } from './search.js';
export {
  BaseSearchProvider,
  GoogleProvider,
  DuckDuckGoProvider,
  BingProvider,
  getAvailableProviders,
} from './providers/index.js';
export {
  mergeResults,
  mergeWithRRF,
  mergeWithWeights,
  mergeWithInterleave,
} from './merger.js';
