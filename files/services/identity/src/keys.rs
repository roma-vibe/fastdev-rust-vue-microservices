//! `/api/keys`: personal API keys. The secret is shown once at creation; only its SHA-256 hash and
//! a short prefix are stored.

use app_core::{
    ApiError, ApiResult, AuthUser, Ctx, Json,
    auth::API_KEY_PREFIX,
    crypto::{hash_token, random_token},
    new_id, now_ms,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const MAX_KEYS: i64 = 50;
const SHOWN_PREFIX: usize = 8;

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    id: String,
    name: String,
    prefix: String,
    created_at: i64,
    last_used_at: Option<i64>,
    revoked_at: Option<i64>,
}

/// `GET /api/keys` → `{items: ApiKey[]}` (own keys, newest first).
pub async fn list(State(ctx): State<Ctx>, user: AuthUser) -> ApiResult<Json<Value>> {
    let items: Vec<ApiKey> = sqlx::query_as(
        "SELECT id, name, prefix, created_at, last_used_at, revoked_at
         FROM api_keys WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(&user.id)
    .fetch_all(&ctx.db)
    .await?;
    Ok(Json(json!({ "items": items })))
}

#[derive(Deserialize)]
pub struct NewKey {
    name: String,
}

/// `POST /api/keys` `{name}` → 201 `{key: ApiKey, secret}`.
pub async fn create(
    State(ctx): State<Ctx>,
    user: AuthUser,
    Json(input): Json<NewKey>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let name = input.name.trim();
    if name.is_empty() || name.chars().count() > 100 {
        return Err(ApiError::bad_request(
            "Give the key a name (up to 100 characters)",
        ));
    }
    let (active,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM api_keys WHERE user_id = ? AND revoked_at IS NULL")
            .bind(&user.id)
            .fetch_one(&ctx.db)
            .await?;
    if active >= MAX_KEYS {
        return Err(ApiError::conflict(format!(
            "At most {MAX_KEYS} active keys; revoke one first"
        )));
    }
    let secret = random_token(API_KEY_PREFIX);
    let key = ApiKey {
        id: new_id(),
        name: name.to_owned(),
        prefix: secret.chars().take(SHOWN_PREFIX).collect(),
        created_at: now_ms(),
        last_used_at: None,
        revoked_at: None,
    };
    sqlx::query(
        "INSERT INTO api_keys (id, user_id, name, prefix, key_hash, created_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&key.id)
    .bind(&user.id)
    .bind(&key.name)
    .bind(&key.prefix)
    .bind(hash_token(&secret))
    .bind(key.created_at)
    .execute(&ctx.db)
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({ "key": key, "secret": secret })),
    ))
}

/// `DELETE /api/keys/{id}` → 204 (sets `revokedAt`; the key stops working at once).
pub async fn revoke(
    State(ctx): State<Ctx>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    let result = sqlx::query(
        "UPDATE api_keys SET revoked_at = ? WHERE id = ? AND user_id = ? AND revoked_at IS NULL",
    )
    .bind(now_ms())
    .bind(&id)
    .bind(&user.id)
    .execute(&ctx.db)
    .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Key not found"));
    }
    Ok(StatusCode::NO_CONTENT)
}
