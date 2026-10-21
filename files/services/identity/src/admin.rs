//! `/api/admin/users`: user management for administrators.

use app_core::{AdminUser, ApiError, ApiResult, Ctx, Json, now_ms};
use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AdminUserRow {
    id: String,
    email: String,
    name: String,
    role: String,
    created_at: i64,
    disabled: bool,
    key_count: i64,
}

const SELECT: &str = "SELECT u.id, u.email, u.name, u.role, u.created_at, u.disabled,
       (SELECT COUNT(*) FROM api_keys k WHERE k.user_id = u.id AND k.revoked_at IS NULL) AS key_count
     FROM users u";

/// `GET /api/admin/users` → `{items: (User & {disabled, keyCount})[]}`, oldest first.
pub async fn list(State(ctx): State<Ctx>, _admin: AdminUser) -> ApiResult<Json<Value>> {
    let items: Vec<AdminUserRow> = sqlx::query_as(&format!("{SELECT} ORDER BY u.created_at"))
        .fetch_all(&ctx.db)
        .await?;
    Ok(Json(json!({ "items": items })))
}

#[derive(Deserialize)]
pub struct UserPatch {
    role: Option<String>,
    disabled: Option<bool>,
}

/// `PATCH /api/admin/users/{id}` `{role?, disabled?}` → the updated row. Admins cannot change
/// their own role or disable themselves; disabling signs the user out everywhere.
pub async fn update(
    State(ctx): State<Ctx>,
    AdminUser(admin): AdminUser,
    Path(id): Path<String>,
    Json(patch): Json<UserPatch>,
) -> ApiResult<Json<AdminUserRow>> {
    if id == admin.id {
        return Err(ApiError::bad_request(
            "You cannot change your own account here",
        ));
    }
    if let Some(role) = &patch.role {
        if role != "user" && role != "admin" {
            return Err(ApiError::bad_request("Role must be user or admin"));
        }
        sqlx::query("UPDATE users SET role = ?, updated_at = ? WHERE id = ?")
            .bind(role)
            .bind(now_ms())
            .bind(&id)
            .execute(&ctx.db)
            .await?;
    }
    if let Some(disabled) = patch.disabled {
        sqlx::query("UPDATE users SET disabled = ?, updated_at = ? WHERE id = ?")
            .bind(disabled)
            .bind(now_ms())
            .bind(&id)
            .execute(&ctx.db)
            .await?;
        if disabled {
            sqlx::query("DELETE FROM sessions WHERE user_id = ?")
                .bind(&id)
                .execute(&ctx.db)
                .await?;
        }
    }
    let row: Option<AdminUserRow> = sqlx::query_as(&format!("{SELECT} WHERE u.id = ?"))
        .bind(&id)
        .fetch_optional(&ctx.db)
        .await?;
    row.map(Json)
        .ok_or_else(|| ApiError::not_found("User not found"))
}
