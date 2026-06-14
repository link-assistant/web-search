#!/usr/bin/env node
/**
 * Web Search CLI
 * Command-line interface for web search aggregation
 *
 * Usage:
 *   web-search <query> [options]      Search the web
 *   web-search --serve [--port 3000]  Start as API server
 */

import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// eslint-disable-next-line complexity -- CLI argument parsing inherently has many branches
function parseArgs(args) {
  const result = {
    query: null,
    serve: false,
    port: 3000,
    providers: null,
    limit: 10,
    strategy: 'rrf',
    format: 'text',
    language: null,
    region: null,
    safeSearch: false,
    help: false,
    version: false,
    verbose: false,
    listProviders: false,
  };

  const positional = [];

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg === '--serve' || arg === '-s') {
      result.serve = true;
    } else if (arg === '--port' || arg === '-p') {
      result.port = parseInt(args[++i], 10) || 3000;
    } else if (arg === '--providers') {
      result.providers = args[++i]?.split(',').map((p) => p.trim());
    } else if (arg === '--limit' || arg === '-l') {
      result.limit = parseInt(args[++i], 10) || 10;
    } else if (arg === '--strategy') {
      result.strategy = args[++i] || 'rrf';
    } else if (arg === '--format' || arg === '-f') {
      result.format = args[++i] || 'text';
    } else if (arg === '--language' || arg === '--lang') {
      result.language = args[++i];
    } else if (arg === '--region') {
      result.region = args[++i];
    } else if (arg === '--safe' || arg === '--safeSearch') {
      result.safeSearch = true;
    } else if (arg === '--help' || arg === '-h') {
      result.help = true;
    } else if (arg === '--version' || arg === '-v') {
      result.version = true;
    } else if (arg === '--verbose' || arg === '-V') {
      result.verbose = true;
    } else if (arg === '--list-providers') {
      result.listProviders = true;
    } else if (!arg.startsWith('-')) {
      positional.push(arg);
    }
  }

  // Support a `serve` subcommand as an alias for `--serve`, mirroring the Rust
  // CLI (`web-search serve --port <port>`) so both languages share the same
  // HTTP-server entry-point syntax.
  if (positional[0] === 'serve') {
    result.serve = true;
    positional.shift();
  }

  if (positional.length > 0) {
    result.query = positional.join(' ');
  }

  return result;
}

function showHelp() {
  console.log(`
web-search - Multi-provider web search aggregator

Usage:
  web-search <query> [options]       Search the web
  web-search serve [--port <port>]   Start as API server
  web-search --serve [--port <port>] Start as API server (flag alias)

Search Options:
  --providers <list>   Comma-separated list of providers (e.g. google,duckduckgo,wikipedia)
                       Run --list-providers to see every available engine.
  --limit, -l <n>      Maximum results per provider (default: 10)
  --strategy <name>    Merge strategy: rrf, weighted, interleave (default: rrf)
  --language <code>    Language code (e.g., en, de)
  --region <code>      Region code (e.g., us, de)
  --safe               Enable safe search filtering

Output Options:
  --format, -f <fmt>   Output format: text, json, urls (default: text)
  --verbose, -V        Show detailed output

Discovery Options:
  --list-providers     List every available provider grouped by category

Server Options:
  --serve, -s          Start as HTTP API server
  --port, -p <port>    Port to listen on (default: 3000)

General Options:
  --help, -h           Show this help message
  --version, -v        Show version number

Provider Categories:
  search     Web search engines (google, bing, duckduckgo, brave, ...)
  knowledge  Knowledge bases (wikipedia, wikidata)
  papers     Scholarly works (crossref, openalex, arxiv)
  code       Code & developer search (github, hackernews)

Environment Variables:
  GOOGLE_API_KEY       Google Custom Search API key
  GOOGLE_CX            Google Custom Search Engine ID
  BING_API_KEY         Bing Search API key
  GITHUB_TOKEN         GitHub token (raises GitHub search rate limits)
  PORT                 Server port (default: 3000)

Examples:
  web-search "javascript tutorial"
  web-search "rust programming" --providers google,duckduckgo --limit 5
  web-search "neural networks" --providers arxiv,crossref --format json | jq .
  web-search --list-providers
  web-search --serve --port 8080

API Endpoints (in server mode):
  GET  /search?q=<query>           Search all providers
  POST /search                     Search with JSON body
  GET  /search/:provider?q=<query> Search single provider
  GET  /providers                  List available providers
  GET  /categories                 List provider categories
  GET  /health                     Health check
`);
}

async function showVersion() {
  const fs = await import('fs');
  const packagePath = resolve(__dirname, '..', 'package.json');
  const packageJson = JSON.parse(fs.readFileSync(packagePath, 'utf-8'));
  console.log(`web-search v${packageJson.version}`);
}

async function showProviders() {
  const { CATEGORIES, getRegistry } = await import('../src/providers/index.js');
  const registry = getRegistry();

  console.log('Available providers by category:\n');
  for (const category of CATEGORIES) {
    const entries = registry.filter((e) => e.category === category);
    if (entries.length === 0) {
      continue;
    }
    console.log(`${category}:`);
    for (const entry of entries) {
      const flags = [
        entry.defaultForCategory ? 'default' : null,
        entry.corsReadable ? 'cors' : null,
        entry.access,
      ]
        .filter(Boolean)
        .join(', ');
      console.log(`  ${entry.id.padEnd(16)} ${entry.label} (${flags})`);
    }
    console.log('');
  }
}

async function startServer(port) {
  const { app } = await import('../src/server.js');

  return new Promise((resolve, reject) => {
    const server = app.listen(port, () => {
      console.log(`Web Search API listening on http://localhost:${port}`);
      console.log('');
      console.log('Available endpoints:');
      console.log('  GET  /search?q=<query>        - Search all providers');
      console.log('  POST /search                  - Search with JSON body');
      console.log(
        '  GET  /search/:provider?q=<query> - Search single provider'
      );
      console.log('  GET  /providers               - List available providers');
      console.log('  GET  /health                  - Health check');
      console.log('');
      console.log('Press Ctrl+C to stop the server');
      resolve(server);
    });

    server.on('error', reject);

    function shutdown(signal) {
      console.log(`\nReceived ${signal}, shutting down...`);
      server.close(() => {
        console.log('Server closed');
        process.exit(0);
      });
      setTimeout(() => {
        console.error('Force exiting after 2s');
        process.exit(1);
      }, 2000);
    }

    process.on('SIGTERM', () => shutdown('SIGTERM'));
    process.on('SIGINT', () => shutdown('SIGINT'));
  });
}

async function performSearch(query, options) {
  const { WebSearchEngine } = await import('../src/search.js');

  const searchEngine = new WebSearchEngine({
    providers: options.providers || undefined,
    google: {
      apiKey: process.env.GOOGLE_API_KEY,
      searchEngineId: process.env.GOOGLE_CX,
    },
    bing: {
      apiKey: process.env.BING_API_KEY,
    },
  });

  const searchOptions = {
    providers: options.providers,
    limit: options.limit,
    language: options.language,
    region: options.region,
    safeSearch: options.safeSearch,
    strategy: options.strategy,
  };

  if (options.verbose) {
    console.error(`Searching for: "${query}"`);
    console.error(`Providers: ${options.providers?.join(', ') || 'all'}`);
    console.error(`Strategy: ${options.strategy}`);
    console.error('');
  }

  const results = await searchEngine.search(query, searchOptions);

  if (options.format === 'json') {
    console.log(
      JSON.stringify(
        {
          query,
          count: results.length,
          results,
        },
        null,
        2
      )
    );
  } else if (options.format === 'urls') {
    for (const result of results) {
      console.log(result.url);
    }
  } else {
    if (results.length === 0) {
      console.log('No results found.');
      return;
    }

    console.log(`Found ${results.length} results for "${query}":\n`);

    for (const result of results) {
      console.log(`${result.rank}. ${result.title}`);
      console.log(`   ${result.url}`);
      if (result.snippet) {
        console.log(
          `   ${result.snippet.slice(0, 150)}${result.snippet.length > 150 ? '...' : ''}`
        );
      }
      console.log(`   [${result.sources?.join(', ') || result.source}]`);
      console.log('');
    }
  }
}

async function main() {
  const args = parseArgs(process.argv.slice(2));

  if (args.help) {
    showHelp();
    return;
  }

  if (args.version) {
    await showVersion();
    return;
  }

  if (args.listProviders) {
    await showProviders();
    return;
  }

  if (args.serve) {
    await startServer(args.port);
    return;
  }

  if (!args.query) {
    console.error('Error: Missing search query');
    console.error('Run with --help for usage information');
    process.exit(1);
  }

  try {
    await performSearch(args.query, args);
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

// Only auto-run the CLI when this file is executed directly, so the argument
// parser can be unit-tested by importing it.
const isMain = process.argv[1] === __filename;
if (isMain) {
  main().catch((err) => {
    console.error('Fatal error:', err.message);
    process.exit(1);
  });
}

export { parseArgs };
