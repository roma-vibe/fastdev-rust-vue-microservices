//! `app-identity` — accounts, sessions and personal API keys (`/api/auth`, `/api/me`, `/api/keys`,
//! `/api/admin/users`). Owns the `users`, `sessions` and `api_keys` tables; every other service
//! authenticates requests through `app_core::auth` (`AuthUser`, `AdminUser`, `Caller`).

mod account;
mod admin;
mod keys;
mod me;
mod password;
mod routes;
#[cfg(test)]
mod tests;
mod users;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app_core::service::run("identity", |ctx| async move { Ok(routes::router(ctx)) }).await
}
