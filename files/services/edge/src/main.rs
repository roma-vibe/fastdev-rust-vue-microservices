//! `app-edge` — the single public entry point.
//!
//! - Forwards requests to the service that owns the path (longest prefix from `services.json`),
//!   streaming bodies both ways.
//! - `GET /api/health`: health of every service and the app name, for the web app.
//! - Everything else: the built SPA from `WEB_DIST` (`web/dist`) with history fallback to
//!   `index.html`. In development Vite serves the SPA and proxies `/api` here.

mod health;
mod proxy;
mod spa;
#[cfg(test)]
mod tests;

use app_core::Ctx;
use axum::{Router, routing::get};

/// Shared state of the edge: the service context and an HTTP client that does not follow redirects.
#[derive(Clone)]
pub struct EdgeState {
    pub ctx: Ctx,
    pub client: reqwest::Client,
}

/// The edge router (also used by the tests).
///
/// # Errors
/// When the HTTP client cannot be built.
pub fn router(ctx: Ctx) -> anyhow::Result<Router> {
    let client = reqwest::Client::builder()
        // Redirects go back to the browser unchanged.
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(std::time::Duration::from_secs(5))
        .build()?;
    let state = EdgeState { ctx, client };
    Ok(Router::new()
        .route("/api/health", get(health::health_all))
        .fallback(proxy::handle)
        .with_state(state))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app_core::service::run("edge", |ctx| async move { router(ctx) }).await
}
