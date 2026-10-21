//! Session tokens and personal API keys: creating sessions and resolving a token to its user.

use sqlx::SqlitePool;

use super::{AuthUser, AuthVia, Role};
use crate::{
    crypto::{hash_token, random_token},
    util::now_ms,
};

pub const SESSION_PREFIX: &str = "sess-";
pub const API_KEY_PREFIX: &str = "sk-";
/// Sessions live 30 days from the last use.
pub const SESSION_TTL_MS: i64 = 30 * 24 * 60 * 60 * 1000;
/// `last_used_at` / `last_seen_at` are written at most once per minute per token.
const TOUCH_INTERVAL_MS: i64 = 60_000;

type UserRow = (String, String, String, String);

/// Resolves a session token or API key to its user, or `None` when unknown, expired or revoked.
///
/// # Errors
/// On database errors.
pub async fn authenticate(db: &SqlitePool, token: &str) -> sqlx::Result<Option<AuthUser>> {
    if token.starts_with(API_KEY_PREFIX) {
        authenticate_key(db, token).await
    } else {
        authenticate_session(db, token).await
    }
}

async fn authenticate_key(db: &SqlitePool, key: &str) -> sqlx::Result<Option<AuthUser>> {
    let now = now_ms();
    let row: Option<(String, String, String, String, String, Option<i64>)> = sqlx::query_as(
        "SELECT u.id, u.email, u.name, u.role, k.id, k.last_used_at
         FROM api_keys k JOIN users u ON u.id = k.user_id
         WHERE k.key_hash = ? AND k.revoked_at IS NULL AND u.disabled = 0",
    )
    .bind(hash_token(key))
    .fetch_optional(db)
    .await?;
    let Some((id, email, name, role, key_id, last_used)) = row else {
        return Ok(None);
    };
    if last_used.is_none_or(|t| now - t > TOUCH_INTERVAL_MS) {
        sqlx::query("UPDATE api_keys SET last_used_at = ? WHERE id = ?")
            .bind(now)
            .bind(&key_id)
            .execute(db)
            .await?;
    }
    Ok(Some(user(
        (id, email, name, role),
        AuthVia::ApiKey { key_id },
    )))
}

async fn authenticate_session(db: &SqlitePool, token: &str) -> sqlx::Result<Option<AuthUser>> {
    let now = now_ms();
    let hash = hash_token(token);
    let row: Option<(String, String, String, String, i64)> = sqlx::query_as(
        "SELECT u.id, u.email, u.name, u.role, s.last_seen_at
         FROM sessions s JOIN users u ON u.id = s.user_id
         WHERE s.token_hash = ? AND s.expires_at > ? AND u.disabled = 0",
    )
    .bind(&hash)
    .bind(now)
    .fetch_optional(db)
    .await?;
    let Some((id, email, name, role, last_seen)) = row else {
        return Ok(None);
    };
    if now - last_seen > TOUCH_INTERVAL_MS {
        sqlx::query("UPDATE sessions SET last_seen_at = ?, expires_at = ? WHERE token_hash = ?")
            .bind(now)
            .bind(now + SESSION_TTL_MS)
            .bind(&hash)
            .execute(db)
            .await?;
    }
    Ok(Some(user((id, email, name, role), AuthVia::Session)))
}

fn user((id, email, name, role): UserRow, via: AuthVia) -> AuthUser {
    AuthUser {
        id,
        email,
        name,
        role: Role::parse(&role),
        via,
    }
}

/// Creates a session for `user_id` and returns its token (sent to the client once, as a cookie).
///
/// # Errors
/// On database errors.
pub async fn create_session(
    db: &SqlitePool,
    user_id: &str,
    user_agent: Option<&str>,
) -> sqlx::Result<String> {
    let token = random_token(SESSION_PREFIX);
    let now = now_ms();
    sqlx::query(
        "INSERT INTO sessions (token_hash, user_id, created_at, expires_at, last_seen_at, user_agent)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(hash_token(&token))
    .bind(user_id)
    .bind(now)
    .bind(now + SESSION_TTL_MS)
    .bind(now)
    .bind(user_agent)
    .execute(db)
    .await?;
    Ok(token)
}

/// Deletes the session of `token` (logout). Unknown tokens are ignored.
///
/// # Errors
/// On database errors.
pub async fn delete_session(db: &SqlitePool, token: &str) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM sessions WHERE token_hash = ?")
        .bind(hash_token(token))
        .execute(db)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::connect_memory, util::new_id};

    async fn insert_user(db: &SqlitePool, role: &str) -> String {
        let id = new_id();
        sqlx::query(
            "INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at)
             VALUES (?, ?, 'T', 'x', ?, 0, 0)",
        )
        .bind(&id)
        .bind(format!("{id}@example.com"))
        .bind(role)
        .execute(db)
        .await
        .unwrap();
        id
    }

    #[tokio::test]
    async fn sessions_authenticate_until_deleted() {
        let db = connect_memory().await.unwrap();
        let uid = insert_user(&db, "admin").await;
        let session = create_session(&db, &uid, None).await.unwrap();
        let found = authenticate(&db, &session).await.unwrap().unwrap();
        assert_eq!(found.id, uid);
        assert!(found.is_admin());
        assert_eq!(found.via, AuthVia::Session);
        delete_session(&db, &session).await.unwrap();
        assert!(authenticate(&db, &session).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn api_keys_authenticate_until_revoked() {
        let db = connect_memory().await.unwrap();
        let uid = insert_user(&db, "user").await;
        let key = random_token(API_KEY_PREFIX);
        sqlx::query(
            "INSERT INTO api_keys (id, user_id, name, prefix, key_hash, created_at)
             VALUES ('k1', ?, 'n', 'p', ?, 0)",
        )
        .bind(&uid)
        .bind(hash_token(&key))
        .execute(&db)
        .await
        .unwrap();
        let found = authenticate(&db, &key).await.unwrap().unwrap();
        assert_eq!(
            found.via,
            AuthVia::ApiKey {
                key_id: "k1".into()
            }
        );
        assert!(!found.is_admin());

        sqlx::query("UPDATE api_keys SET revoked_at = 1")
            .execute(&db)
            .await
            .unwrap();
        assert!(authenticate(&db, &key).await.unwrap().is_none());
        assert!(authenticate(&db, "sess-unknown").await.unwrap().is_none());
    }
}
