//! The routes of the identity service; handlers live in one module per resource.

use app_core::Ctx;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{account, admin, keys, me};

pub fn router(ctx: Ctx) -> Router {
    Router::new()
        .route("/api/auth/config", get(account::config))
        .route("/api/auth/register", post(account::register))
        .route("/api/auth/login", post(account::login))
        .route("/api/auth/logout", post(account::logout))
        .route("/api/me", get(me::show).patch(me::update))
        .route("/api/me/password", post(me::change_password))
        .route("/api/keys", get(keys::list).post(keys::create))
        .route("/api/keys/{id}", delete(keys::revoke))
        .route("/api/admin/users", get(admin::list))
        .route("/api/admin/users/{id}", patch(admin::update))
        .with_state(ctx)
}
