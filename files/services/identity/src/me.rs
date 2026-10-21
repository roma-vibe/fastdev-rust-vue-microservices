//! `/api/me`: the signed-in user's own account.

use app_core::{
    ApiError, ApiResult, AuthUser, AuthVia, Ctx, Json, auth::request_token, crypto::hash_token,
    now_ms,
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;

use crate::{password, users};

/// `GET /api/me` → `User`.
pub async fn show(State(ctx): State<Ctx>, user: AuthUser) -> ApiResult<Json<users::User>> {
    load(&ctx, &user.id).await.map(Json)
}

#[derive(Deserialize)]
pub struct MePatch {
    name: Option<String>,
}

/// `PATCH /api/me` `{name?}` → `User`.
pub async fn update(
    State(ctx): State<Ctx>,
    user: AuthUser,
    Json(patch): Json<MePatch>,
) -> ApiResult<Json<users::User>> {
    if let Some(name) = patch.name {
        let name = users::clean_name(&name)?;
        sqlx::query("UPDATE users SET name = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(now_ms())
            .bind(&user.id)
            .execute(&ctx.db)
            .await?;
    }
    load(&ctx, &user.id).await.map(Json)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChange {
    current_password: String,
    new_password: String,
}

/// `POST /api/me/password` `{currentPassword, newPassword}` → 204. Signs out every other session.
pub async fn change_password(
    State(ctx): State<Ctx>,
    user: AuthUser,
    headers: HeaderMap,
    Json(input): Json<PasswordChange>,
) -> ApiResult<StatusCode> {
    password::check(&input.new_password)?;
    let (stored,): (String,) = sqlx::query_as("SELECT password_hash FROM users WHERE id = ?")
        .bind(&user.id)
        .fetch_one(&ctx.db)
        .await?;
    if !password::verify(&input.current_password, &stored).await {
        return Err(ApiError::bad_request("The current password is wrong"));
    }
    let hash = password::hash(&input.new_password).await?;
    sqlx::query("UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?")
        .bind(hash)
        .bind(now_ms())
        .bind(&user.id)
        .execute(&ctx.db)
        .await?;
    let current = match user.via {
        AuthVia::Session => request_token(&headers, &ctx.config).map(|t| hash_token(&t)),
        AuthVia::ApiKey { .. } => None,
    };
    sqlx::query("DELETE FROM sessions WHERE user_id = ? AND token_hash != ?")
        .bind(&user.id)
        .bind(current.unwrap_or_default())
        .execute(&ctx.db)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn load(ctx: &Ctx, id: &str) -> ApiResult<users::User> {
    users::by_id(&ctx.db, id)
        .await?
        .ok_or_else(|| ApiError::not_found("User not found"))
}
