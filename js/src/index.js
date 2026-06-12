/**
 * Web Search - Multi-provider web search aggregator
 *
 * A library + microservice that aggregates results from many search engines and
 * knowledge/paper/code APIs, with support for result merging and reranking
 * (Reciprocal Rank Fusion, weighted, interleave). Providers are organized by a
 * typed registry into the four categories `formal-ai` consumes
 * (`search`, `knowledge`, `papers`, `code`), and the optional
 * `@link-assistant/web-capture` component library can back any provider.
 */

export { WebSearchEngine, createSearchEngine } from './search.js';
export {
  BaseSearchProvider,
  GoogleProvider,
  DuckDuckGoProvider,
  BingProvider,
  BrowserSearchProvider,
  createBrowserProvider,
  GenericProvider,
  createGenericProvider,
  WebCaptureProvider,
  createWebCaptureProvider,
  decodeHtmlEntities,
  stripHtml,
  cleanText,
  parseAnchorList,
  API_ENGINES,
  HTML_ENGINES,
  CATEGORIES,
  getRegistry,
  getProviderIds,
  getDefaultProviderIds,
  buildProviders,
  getAvailableProviders,
} from './providers/index.js';
export {
  mergeResults,
  mergeWithRRF,
  mergeWithWeights,
  mergeWithInterleave,
} from './merger.js';
