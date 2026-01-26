//! Web Search CLI and Server
//!
//! Usage:
//!   web-search <query> [options]       Search the web
//!   web-search --serve [--port <port>] Start as API server

use std::collections::HashMap;
use std::net::SocketAddr;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

use web_search::{MergeOptions, MergeStrategy, SearchOptions, WebSearchEngine};

#[derive(Parser)]
#[command(name = "web-search")]
#[command(about = "Multi-provider web search aggregator")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Search query (when not using subcommands)
    #[arg(trailing_var_arg = true)]
    query: Vec<String>,

    /// Providers to use (comma-separated)
    #[arg(long, value_delimiter = ',')]
    providers: Option<Vec<String>>,

    /// Maximum results per provider
    #[arg(short, long, default_value = "10")]
    limit: usize,

    /// Merge strategy (rrf, weighted, interleave)
    #[arg(long, default_value = "rrf")]
    strategy: String,

    /// Output format (text, json, urls)
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Language code
    #[arg(long)]
    language: Option<String>,

    /// Region code
    #[arg(long)]
    region: Option<String>,

    /// Enable safe search
    #[arg(long)]
    safe: bool,

    /// Verbose output
    #[arg(short = 'V', long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start as HTTP API server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
}

#[derive(Clone)]
struct AppState {
    engine: std::sync::Arc<WebSearchEngine>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
    query: Option<String>,
    providers: Option<String>,
    limit: Option<usize>,
    strategy: Option<String>,
    language: Option<String>,
    region: Option<String>,
    #[serde(rename = "safeSearch")]
    safe_search: Option<bool>,
    safe: Option<bool>,
}

#[derive(Debug, Serialize)]
struct SearchResponse {
    query: String,
    count: usize,
    options: SearchResponseOptions,
    results: Vec<web_search::providers::SearchResult>,
}

#[derive(Debug, Serialize)]
struct SearchResponseOptions {
    providers: Vec<String>,
    strategy: String,
    limit: usize,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    providers: HashMap<String, web_search::search::ProviderStatus>,
}

async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        providers: state.engine.get_provider_status().await,
    })
}

async fn providers_handler(
    State(state): State<AppState>,
) -> Json<HashMap<String, web_search::search::ProviderStatus>> {
    Json(state.engine.get_provider_status().await)
}

async fn search_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<ErrorResponse>)> {
    let query = params.q.or(params.query).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Missing required parameter: q or query".to_string(),
                message: None,
            }),
        )
    })?;

    let providers = params
        .providers
        .map(|p| p.split(',').map(|s| s.trim().to_string()).collect());
    let limit = params.limit.unwrap_or(10);
    let strategy = match params.strategy.as_deref() {
        Some("weighted") => MergeStrategy::Weighted,
        Some("interleave") => MergeStrategy::Interleave,
        _ => MergeStrategy::Rrf,
    };
    let safe_search = params.safe_search.or(params.safe);

    let search_options = SearchOptions {
        limit: Some(limit),
        language: params.language,
        region: params.region,
        safe_search,
    };

    let merge_options = MergeOptions {
        strategy,
        weights: HashMap::new(),
        rrf_k: None,
        remove_duplicates: true,
    };

    let results = state
        .engine
        .search_with_options(
            &query,
            search_options,
            providers.clone(),
            Some(merge_options),
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Search failed".to_string(),
                    message: Some(e.to_string()),
                }),
            )
        })?;

    let strategy_str = match strategy {
        MergeStrategy::Rrf => "rrf",
        MergeStrategy::Weighted => "weighted",
        MergeStrategy::Interleave => "interleave",
    };

    Ok(Json(SearchResponse {
        query,
        count: results.len(),
        options: SearchResponseOptions {
            providers: providers.unwrap_or_else(|| state.engine.get_available_providers()),
            strategy: strategy_str.to_string(),
            limit,
        },
        results,
    }))
}

async fn search_provider_handler(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let query = params.q.or(params.query).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Missing required parameter: q or query".to_string(),
                message: None,
            }),
        )
    })?;

    let search_options = SearchOptions {
        limit: params.limit,
        language: params.language,
        region: params.region,
        safe_search: params.safe_search.or(params.safe),
    };

    let results = state
        .engine
        .search_single(&query, &provider, search_options)
        .await
        .map_err(|e| {
            let status = if e.to_string().contains("Unknown provider") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (
                status,
                Json(ErrorResponse {
                    error: e.to_string(),
                    message: None,
                }),
            )
        })?;

    Ok(Json(serde_json::json!({
        "query": query,
        "provider": provider,
        "count": results.len(),
        "results": results,
    })))
}

async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        engine: std::sync::Arc::new(WebSearchEngine::new()),
    };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/providers", get(providers_handler))
        .route("/search", get(search_handler))
        .route("/search/{provider}", get(search_provider_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Web Search API listening on http://localhost:{}", port);
    println!();
    println!("Available endpoints:");
    println!("  GET  /search?q=<query>        - Search all providers");
    println!("  GET  /search/:provider?q=<query> - Search single provider");
    println!("  GET  /providers               - List available providers");
    println!("  GET  /health                  - Health check");
    println!();
    println!("Press Ctrl+C to stop the server");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn perform_search(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let query = cli.query.join(" ");
    if query.is_empty() {
        eprintln!("Error: Missing search query");
        eprintln!("Run with --help for usage information");
        std::process::exit(1);
    }

    let engine = WebSearchEngine::new();

    let strategy = match cli.strategy.as_str() {
        "weighted" => MergeStrategy::Weighted,
        "interleave" => MergeStrategy::Interleave,
        _ => MergeStrategy::Rrf,
    };

    let search_options = SearchOptions {
        limit: Some(cli.limit),
        language: cli.language.clone(),
        region: cli.region.clone(),
        safe_search: if cli.safe { Some(true) } else { None },
    };

    let merge_options = MergeOptions {
        strategy,
        weights: HashMap::new(),
        rrf_k: None,
        remove_duplicates: true,
    };

    if cli.verbose {
        eprintln!("Searching for: \"{}\"", query);
        eprintln!(
            "Providers: {}",
            cli.providers
                .as_ref()
                .map(|p| p.join(", "))
                .unwrap_or_else(|| "all".to_string())
        );
        eprintln!("Strategy: {}", cli.strategy);
        eprintln!();
    }

    let results = engine
        .search_with_options(
            &query,
            search_options,
            cli.providers.clone(),
            Some(merge_options),
        )
        .await?;

    match cli.format.as_str() {
        "json" => {
            let response = serde_json::json!({
                "query": query,
                "count": results.len(),
                "results": results,
            });
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
        "urls" => {
            for result in &results {
                println!("{}", result.url);
            }
        }
        _ => {
            if results.is_empty() {
                println!("No results found.");
                return Ok(());
            }

            println!("Found {} results for \"{}\":\n", results.len(), query);

            for result in &results {
                println!("{}. {}", result.rank, result.title);
                println!("   {}", result.url);
                if !result.snippet.is_empty() {
                    let snippet = if result.snippet.len() > 150 {
                        format!("{}...", &result.snippet[..150])
                    } else {
                        result.snippet.clone()
                    };
                    println!("   {}", snippet);
                }
                let sources = result
                    .sources
                    .as_ref()
                    .map(|s| s.join(", "))
                    .unwrap_or_else(|| result.source.clone());
                println!("   [{}]", sources);
                println!();
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Serve { port }) => {
            start_server(*port).await?;
        }
        None => {
            perform_search(&cli).await?;
        }
    }

    Ok(())
}
