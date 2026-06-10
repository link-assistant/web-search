/**
 * Web Search REST API Server
 * Express.js microservice for web search aggregation
 */

import express from 'express';
import { fileURLToPath } from 'url';
import { WebSearchEngine } from './search.js';
import {
  CATEGORIES,
  getRegistry,
  getProviderIds,
} from './providers/registry.js';

const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());

const searchEngine = new WebSearchEngine({
  google: {
    apiKey: process.env.GOOGLE_API_KEY,
    searchEngineId: process.env.GOOGLE_CX,
  },
  bing: {
    apiKey: process.env.BING_API_KEY,
  },
});

/**
 * Health check endpoint
 * GET /health
 */
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    providers: searchEngine.getProviderStatus(),
  });
});

/**
 * Main search endpoint
 * GET /search?q=<query>&providers=<providers>&limit=<limit>&strategy=<strategy>
 */
app.get('/search', async (req, res) => {
  const { q, query } = req.query;
  const searchQuery = q || query;

  if (!searchQuery) {
    return res.status(400).json({
      error: 'Missing required parameter: q or query',
    });
  }

  const options = {
    limit: parseInt(req.query.limit, 10) || 10,
    language: req.query.language || req.query.lang,
    region: req.query.region,
    safeSearch:
      req.query.safeSearch === 'true' ||
      req.query.safe === 'true' ||
      req.query.safeSearch === '1',
    strategy: req.query.strategy || 'rrf',
  };

  if (req.query.providers) {
    options.providers = req.query.providers.split(',').map((p) => p.trim());
  }

  if (req.query.weights) {
    try {
      options.weights = JSON.parse(req.query.weights);
    } catch {
      return res.status(400).json({
        error: 'Invalid weights parameter: must be valid JSON',
      });
    }
  }

  try {
    const results = await searchEngine.search(searchQuery, options);
    res.json({
      query: searchQuery,
      count: results.length,
      options: {
        providers: options.providers || searchEngine.getAvailableProviders(),
        strategy: options.strategy,
        limit: options.limit,
      },
      results,
    });
  } catch (error) {
    console.error('Search error:', error);
    res.status(500).json({
      error: 'Search failed',
      message: error.message,
    });
  }
});

/**
 * POST /search - Search with JSON body
 */
app.post('/search', async (req, res) => {
  const {
    query,
    q,
    providers,
    limit,
    language,
    region,
    safeSearch,
    strategy,
    weights,
  } = req.body;
  const searchQuery = query || q;

  if (!searchQuery) {
    return res.status(400).json({
      error: 'Missing required parameter: query or q',
    });
  }

  const options = {
    providers,
    limit: limit || 10,
    language,
    region,
    safeSearch,
    strategy: strategy || 'rrf',
    weights,
  };

  try {
    const results = await searchEngine.search(searchQuery, options);
    res.json({
      query: searchQuery,
      count: results.length,
      options: {
        providers: options.providers || searchEngine.getAvailableProviders(),
        strategy: options.strategy,
        limit: options.limit,
      },
      results,
    });
  } catch (error) {
    console.error('Search error:', error);
    res.status(500).json({
      error: 'Search failed',
      message: error.message,
    });
  }
});

/**
 * Search with single provider
 * GET /search/:provider?q=<query>
 */
app.get('/search/:provider', async (req, res) => {
  const { provider } = req.params;
  const { q, query } = req.query;
  const searchQuery = q || query;

  if (!searchQuery) {
    return res.status(400).json({
      error: 'Missing required parameter: q or query',
    });
  }

  const options = {
    limit: parseInt(req.query.limit, 10) || 10,
    language: req.query.language || req.query.lang,
    region: req.query.region,
    safeSearch:
      req.query.safeSearch === 'true' ||
      req.query.safe === 'true' ||
      req.query.safeSearch === '1',
  };

  try {
    const results = await searchEngine.searchSingle(
      searchQuery,
      provider,
      options
    );
    res.json({
      query: searchQuery,
      provider,
      count: results.length,
      results,
    });
  } catch (error) {
    if (error.message.includes('Unknown provider')) {
      return res.status(400).json({
        error: error.message,
        availableProviders: searchEngine.getAvailableProviders(),
      });
    }
    console.error(`Search error (${provider}):`, error);
    res.status(500).json({
      error: 'Search failed',
      message: error.message,
    });
  }
});

/**
 * Get available providers
 * GET /providers?category=<category>
 */
app.get('/providers', (req, res) => {
  const { category } = req.query;
  if (category && !CATEGORIES.includes(category)) {
    return res.status(400).json({
      error: `Unknown category: ${category}`,
      categories: CATEGORIES,
    });
  }

  const status = searchEngine.getProviderStatus();
  const registry = getRegistry().filter(
    (e) => !category || e.category === category
  );

  res.json({
    categories: CATEGORIES,
    count: registry.length,
    providers: category
      ? Object.fromEntries(
          Object.entries(status).filter(([, s]) => s.category === category)
        )
      : status,
    registry,
  });
});

/**
 * Get provider categories and the providers within each
 * GET /categories
 */
app.get('/categories', (req, res) => {
  const categories = {};
  for (const category of CATEGORIES) {
    categories[category] = getProviderIds(category);
  }
  res.json({ categories });
});

const isMainModule =
  process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1];

let server;
if (isMainModule) {
  console.log('Process PID:', process.pid);
  server = app.listen(port, () => {
    console.log(`Web Search API listening on http://localhost:${port}`);
    console.log('');
    console.log('Available endpoints:');
    console.log('  GET  /search?q=<query>        - Search all providers');
    console.log('  POST /search                  - Search with JSON body');
    console.log('  GET  /search/:provider?q=<query> - Search single provider');
    console.log('  GET  /providers               - List available providers');
    console.log('  GET  /categories              - List provider categories');
    console.log('  GET  /health                  - Health check');
    console.log('');
    console.log('Query parameters:');
    console.log('  q, query     - Search query (required)');
    console.log('  providers    - Comma-separated list of providers');
    console.log('  limit        - Max results per provider (default: 10)');
    console.log('  strategy     - Merge strategy: rrf, weighted, interleave');
    console.log('  weights      - JSON object with provider weights');
    console.log('  language     - Language code (e.g., en, de)');
    console.log('  region       - Region code (e.g., us, de)');
    console.log('  safeSearch   - Enable safe search (true/false)');
    console.log('');
    console.log('Press Ctrl+C to stop the server');
  });

  function shutdown(signal) {
    console.log(`Received shutdown signal (${signal}), closing server...`);
    server.close(() => {
      console.log('Server closed. Exiting process.');
      process.exit(0);
    });
    setTimeout(() => {
      console.error('Force exiting after 2s');
      process.exit(1);
    }, 2000);
  }

  process.on('SIGTERM', () => shutdown('SIGTERM'));
  process.on('SIGINT', () => shutdown('SIGINT'));

  process.on('exit', (code) => {
    console.log('Process exit event with code:', code);
  });
  process.on('uncaughtException', (err) => {
    console.error('Uncaught Exception:', err);
  });
  process.on('unhandledRejection', (reason, promise) => {
    console.error('Unhandled Rejection at:', promise, 'reason:', reason);
  });
}

export { app, searchEngine };
