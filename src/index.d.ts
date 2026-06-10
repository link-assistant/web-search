/**
 * Web Search - Multi-provider web search aggregator
 * TypeScript type definitions
 */

/**
 * A single search result
 */
export interface SearchResult {
  /** The title of the search result */
  title: string;
  /** The URL of the search result */
  url: string;
  /** The description/snippet of the search result */
  snippet: string;
  /** The search provider that returned this result */
  source: string;
  /** The rank position in the original results (1-based) */
  rank: number;
  /** Computed score after merging (optional) */
  score?: number;
  /** Sources that returned this result (after deduplication) */
  sources?: string[];
}

/**
 * Options for search queries
 */
export interface SearchOptions {
  /** Maximum number of results to return per provider */
  limit?: number;
  /** Language code (e.g., 'en', 'de') */
  language?: string;
  /** Region code (e.g., 'us', 'de') */
  region?: string;
  /** Enable safe search filtering */
  safeSearch?: boolean;
  /** Providers to use for this search */
  providers?: string[];
  /** Merge strategy */
  strategy?: 'rrf' | 'weighted' | 'interleave';
  /** Provider weights */
  weights?: Record<string, number>;
}

/**
 * Options for merging search results
 */
export interface MergeOptions {
  /** Merge strategy */
  strategy?: 'rrf' | 'weighted' | 'interleave';
  /** Provider weights */
  weights?: Record<string, number>;
  /** RRF k parameter (default: 60) */
  rrfK?: number;
  /** Remove duplicate URLs (default: true) */
  removeDuplicates?: boolean;
}

/**
 * Provider category, mirroring `formal-ai`'s `web_search_core` registry.
 */
export type ProviderCategory = 'search' | 'knowledge' | 'papers' | 'code';

/**
 * How a provider obtains its results.
 */
export type ProviderAccess =
  | 'api'
  | 'html'
  | 'hybrid'
  | 'browser'
  | 'component'
  | 'unknown';

/**
 * Provider status information, enriched with registry metadata.
 */
export interface ProviderStatus {
  /** Whether the provider is enabled */
  enabled: boolean;
  /** Provider weight for reranking */
  weight: number;
  /** Whether the provider has API credentials */
  hasApi: boolean;
  /** Provider category */
  category: ProviderCategory;
  /** Human-readable label */
  label: string;
  /** Whether the endpoint is browser-CORS readable */
  corsReadable: boolean;
  /** How results are obtained */
  access: ProviderAccess;
}

/**
 * A single registry entry describing a provider.
 */
export interface RegistryEntry {
  /** Stable provider id */
  id: string;
  /** Human-readable label */
  label: string;
  /** Provider category */
  category: ProviderCategory;
  /** Whether the endpoint is browser-CORS readable */
  corsReadable: boolean;
  /** Whether this is its category's default */
  defaultForCategory: boolean;
  /** How results are obtained */
  access: ProviderAccess;
}

/**
 * A descriptor for a descriptor-driven (generic) engine.
 */
export interface EngineDescriptor {
  id: string;
  label: string;
  category: ProviderCategory;
  kind: 'json' | 'text' | 'html';
  corsReadable?: boolean;
  defaultForCategory?: boolean;
  method?: 'GET' | 'POST';
  buildUrl(query: string, options?: SearchOptions): string;
  buildBody?(query: string, options?: SearchOptions): string;
  headers?(options?: SearchOptions): Record<string, string>;
  parse(
    payload: unknown,
    limit: number,
    options?: SearchOptions
  ): SearchResult[];
}

/**
 * Configuration for Google provider
 */
export interface GoogleConfig {
  /** Google Custom Search API key */
  apiKey?: string;
  /** Google Custom Search Engine ID */
  searchEngineId?: string;
}

/**
 * Configuration for Bing provider
 */
export interface BingConfig {
  /** Bing Search API key */
  apiKey?: string;
}

/**
 * Configuration for WebSearchEngine
 */
export interface WebSearchConfig {
  /** Default providers to use */
  providers?: string[];
  /** Google provider configuration */
  google?: GoogleConfig;
  /** Bing provider configuration */
  bing?: BingConfig;
  /** Default weights for providers */
  weights?: Record<string, number>;
  /** Default merge strategy */
  mergeStrategy?: 'rrf' | 'weighted' | 'interleave';
  /** Injectable fetch implementation (primarily for testing) */
  fetchImpl?: typeof fetch;
}

/**
 * Base class for search providers
 */
export declare abstract class BaseSearchProvider {
  /** Provider name */
  readonly name: string;
  /** Whether the provider is enabled */
  enabled: boolean;
  /** Provider weight for reranking */
  weight: number;

  constructor(name: string);

  /**
   * Search for results using this provider
   */
  abstract search(
    query: string,
    options?: SearchOptions
  ): Promise<SearchResult[]>;

  /**
   * Check if the provider is available/configured
   */
  isAvailable(): boolean;

  /**
   * Get the provider name
   */
  getName(): string;

  /**
   * Get the provider weight
   */
  getWeight(): number;

  /**
   * Set the provider weight
   */
  setWeight(weight: number): void;
}

/**
 * Google search provider
 */
export declare class GoogleProvider extends BaseSearchProvider {
  constructor(config?: GoogleConfig);
  hasApiCredentials(): boolean;
  search(query: string, options?: SearchOptions): Promise<SearchResult[]>;
}

/**
 * DuckDuckGo search provider
 */
export declare class DuckDuckGoProvider extends BaseSearchProvider {
  constructor();
  search(query: string, options?: SearchOptions): Promise<SearchResult[]>;
}

/**
 * Bing search provider
 */
export declare class BingProvider extends BaseSearchProvider {
  constructor(config?: BingConfig);
  hasApiCredentials(): boolean;
  search(query: string, options?: SearchOptions): Promise<SearchResult[]>;
}

/**
 * Browser-based search provider using browser-commander
 */
export declare class BrowserSearchProvider extends BaseSearchProvider {
  constructor(config?: {
    engine?: 'google' | 'duckduckgo' | 'bing';
    browserCommander?: unknown;
    browserOptions?: Record<string, unknown>;
  });
  search(query: string, options?: SearchOptions): Promise<SearchResult[]>;
}

/**
 * Generic, descriptor-driven search provider. A single implementation that can
 * speak to any engine described by a catalog descriptor.
 */
export declare class GenericProvider extends BaseSearchProvider {
  constructor(
    descriptor: EngineDescriptor,
    config?: { fetchImpl?: typeof fetch }
  );
  readonly category: ProviderCategory;
  readonly label: string;
  readonly corsReadable: boolean;
  search(query: string, options?: SearchOptions): Promise<SearchResult[]>;
}

/**
 * Provider that delegates a single-provider fetch + parse to the optional
 * `@link-assistant/web-capture` component library.
 */
export declare class WebCaptureProvider extends BaseSearchProvider {
  static readonly SUPPORTED_PROVIDERS: string[];
  constructor(config?: {
    engine?: string;
    fetchImpl?: typeof fetch;
    searchImpl?: (args: {
      query: string;
      provider: string;
      limit: number;
      fetchImpl?: typeof fetch;
    }) => Promise<{ results?: SearchResult[] }>;
  });
  readonly engine: string;
  search(query: string, options?: SearchOptions): Promise<SearchResult[]>;
}

/**
 * Web Search Engine - main class for multi-provider search
 */
export declare class WebSearchEngine {
  constructor(config?: WebSearchConfig);

  /**
   * Search across multiple providers
   */
  search(query: string, options?: SearchOptions): Promise<SearchResult[]>;

  /**
   * Search with a single provider
   */
  searchSingle(
    query: string,
    providerName: string,
    options?: SearchOptions
  ): Promise<SearchResult[]>;

  /**
   * Get available provider names
   */
  getAvailableProviders(): string[];

  /**
   * Get provider status information
   */
  getProviderStatus(): Record<string, ProviderStatus>;

  /**
   * Set provider weight
   */
  setProviderWeight(name: string, weight: number): void;

  /**
   * Enable or disable a provider
   */
  setProviderEnabled(name: string, enabled: boolean): void;

  /**
   * Get a provider by name
   */
  getProvider(name: string): BaseSearchProvider | undefined;
}

/**
 * Create a default web search engine instance
 */
export declare function createSearchEngine(
  config?: WebSearchConfig
): WebSearchEngine;

/**
 * Get list of available provider names (every registered engine)
 */
export declare function getAvailableProviders(): string[];

/**
 * Create a browser search provider
 */
export declare function createBrowserProvider(config?: {
  engine?: 'google' | 'duckduckgo' | 'bing';
  browserCommander?: unknown;
  browserOptions?: Record<string, unknown>;
}): BrowserSearchProvider;

/**
 * Create a descriptor-driven generic provider
 */
export declare function createGenericProvider(
  descriptor: EngineDescriptor,
  config?: { fetchImpl?: typeof fetch }
): GenericProvider;

/**
 * Create a web-capture-backed provider
 */
export declare function createWebCaptureProvider(config?: {
  engine?: string;
  fetchImpl?: typeof fetch;
}): WebCaptureProvider;

/** Provider categories, mirroring `formal-ai`'s `web_search_core` registry. */
export declare const CATEGORIES: ProviderCategory[];

/** All API-based engine descriptors keyed by id. */
export declare const API_ENGINES: Record<string, EngineDescriptor>;

/** All HTML-scraping engine descriptors keyed by id. */
export declare const HTML_ENGINES: Record<string, EngineDescriptor>;

/** Build the full registry of provider entries. */
export declare function getRegistry(): RegistryEntry[];

/** Get all provider ids, optionally filtered by category. */
export declare function getProviderIds(category?: ProviderCategory): string[];

/** Get the default provider ids used when the caller does not specify any. */
export declare function getDefaultProviderIds(): string[];

/** Instantiate every registered provider. */
export declare function buildProviders(
  config?: WebSearchConfig
): Map<string, BaseSearchProvider>;

/** Decode the common HTML entities (named + numeric) in search results. */
export declare function decodeHtmlEntities(text: string): string;

/** Strip HTML tags from a string and trim the result. */
export declare function stripHtml(html: string): string;

/** Strip tags, decode entities, and collapse whitespace. */
export declare function cleanText(text: string): string;

/** Generic HTML result-list parser driven by a per-engine regex. */
export declare function parseAnchorList(
  html: string,
  config: {
    itemRegex: RegExp;
    source: string;
    limit: number;
    urlGroup: number;
    titleGroup: number;
    snippetGroup?: number;
    urlTransform?: (url: string) => string;
    skip?: (url: string) => boolean;
  }
): SearchResult[];

/**
 * Merge search results using the specified strategy
 */
export declare function mergeResults(
  resultsByProvider: Record<string, SearchResult[]>,
  options?: MergeOptions
): SearchResult[];

/**
 * Merge results using Reciprocal Rank Fusion
 */
export declare function mergeWithRRF(
  resultsByProvider: Record<string, SearchResult[]>,
  options?: MergeOptions
): SearchResult[];

/**
 * Merge results using weighted scoring
 */
export declare function mergeWithWeights(
  resultsByProvider: Record<string, SearchResult[]>,
  options?: MergeOptions
): SearchResult[];

/**
 * Merge results using interleaving (round-robin)
 */
export declare function mergeWithInterleave(
  resultsByProvider: Record<string, SearchResult[]>,
  options?: MergeOptions
): SearchResult[];
