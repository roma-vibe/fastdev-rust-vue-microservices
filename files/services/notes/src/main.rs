//! `app-notes` — example service: notes of the current user (`/api/notes`).
//!
//! It shows every layer of a service: `routes` (HTTP: extract, validate, respond), `model` (types
//! and validation), `store` (SQL over the `notes` table this service owns).

mod model;
mod routes;
mod store;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app_core::service::run("notes", |ctx| async move { Ok(routes::router(ctx)) }).await
}
