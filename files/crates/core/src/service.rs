//! Service bootstrap: every `services/<name>/src/main.rs` is essentially
//!
//! ```ignore
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     app_core::service::run("notes", |ctx| async move { Ok(routes::router(ctx)) }).await
//! }
//! ```
//!
//! [`run`] loads the config, sets up tracing, opens the database (applying migrations), builds a
//! [`Ctx`], adds `GET /health`, request tracing and graceful shutdown, and listens on the port of
//! the service from `services.json`.

use std::{future::Future, ops::Deref, sync::Arc, time::Duration};

use anyhow::Context as _;
use axum::{Router, routing::get};
use serde_json::json;
use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::{config::Config, crypto::Crypto, db, error::Json};

/// Shared state of a service: config, database pool, secret encryption and an HTTP client.
/// Cheap to clone. Use it as the axum state (or derive `FromRef<YourState> for Ctx`).
#[derive(Clone, Debug)]
pub struct Ctx(Arc<CtxInner>);

#[derive(Debug)]
pub struct CtxInner {
    /// Name of the running service (from `services.json`).
    pub service: String,
    pub config: Config,
    pub db: SqlitePool,
    pub crypto: Crypto,
    /// Shared outbound HTTP client (connection pooling); per-request timeouts are set by callers.
    pub http: reqwest::Client,
}

impl Deref for Ctx {
    type Target = CtxInner;
    fn deref(&self) -> &CtxInner {
        &self.0
    }
}

impl Ctx {
    /// Builds a context from a loaded config: opens the database and applies migrations.
    ///
    /// # Errors
    /// When the database or the master key cannot be used.
    pub async fn new(service: &str, config: Config) -> anyhow::Result<Self> {
        let db = db::connect(&config.db_path).await?;
        Self::with_db(service, config, db)
    }

    /// A context over an existing pool (tests use [`db::connect_memory`]).
    ///
    /// # Errors
    /// When the master key is invalid.
    pub fn with_db(service: &str, config: Config, db: SqlitePool) -> anyhow::Result<Self> {
        let crypto = Crypto::new(&config.master_key)?;
        let http = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .user_agent(concat!("app/", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self(Arc::new(CtxInner {
            service: service.to_owned(),
            config,
            db,
            crypto,
            http,
        })))
    }

    /// A ready context for tests: in-memory database with migrations, random master key.
    ///
    /// # Errors
    /// When migrations fail.
    pub async fn for_tests(service: &str) -> anyhow::Result<Self> {
        Self::for_tests_with(service, &[]).await
    }

    /// Like [`Ctx::for_tests`], with extra config values (e.g. `("NOTES_URL", "http://…")`).
    ///
    /// # Errors
    /// When migrations fail.
    pub async fn for_tests_with(service: &str, vars: &[(&str, &str)]) -> anyhow::Result<Self> {
        let mut map: std::collections::HashMap<String, String> = vars
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        map.entry("MASTER_KEY".to_owned())
            .or_insert_with(crate::crypto::generate_master_key);
        let config = Config::from_vars(std::env::temp_dir(), map)?;
        Self::with_db(service, config, db::connect_memory().await?)
    }
}

/// Initializes tracing (`LOG_LEVEL` / `RUST_LOG`) once per process.
pub fn init_tracing(config: &Config) {
    let filter = EnvFilter::try_new(&config.log_level).unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .try_init();
}

/// Runs service `name`: see the module docs. `build` gets the [`Ctx`] and returns the service's
/// routes; it may also spawn background tasks (`tokio::spawn`).
///
/// # Errors
/// When startup fails (config, database, port in use) or the server stops with an error.
pub async fn run<F, Fut>(name: &str, build: F) -> anyhow::Result<()>
where
    F: FnOnce(Ctx) -> Fut,
    Fut: Future<Output = anyhow::Result<Router>>,
{
    let config = Config::load()?;
    init_tracing(&config);
    let port = config.port(name);
    let addr = format!("{}:{port}", config.bind_host);
    let ctx = Ctx::new(name, config).await?;
    let health_route = Router::new()
        .route("/health", get(health))
        .with_state(name.to_owned());
    let router = build(ctx.clone())
        .await?
        .merge(health_route)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("{name}: cannot listen on {addr}"))?;
    tracing::info!("{name} listening on http://{addr}");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    tracing::info!("{name} stopped");
    Ok(())
}

async fn health(
    axum::extract::State(name): axum::extract::State<String>,
) -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": name, "version": env!("CARGO_PKG_VERSION") }))
}

/// Resolves on Ctrl+C or SIGTERM.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut s) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
