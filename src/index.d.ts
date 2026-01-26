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
 * Provider status information
 */
export interface ProviderStatus {
  /** Whether the provider is enabled */
  enabled: boolean;
  /** Provider weight for reranking */
  weight: number;
  /** Whether the provider has API credentials */
  hasApi: boolean;
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
  abstract search(query: string, options?: SearchOptions): Promise<SearchResult[]>;

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
export declare function createSearchEngine(config?: WebSearchConfig): WebSearchEngine;

/**
 * Get list of available provider names
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
