//! Typed access to the `settings` table (`key` → JSON `value`). Each key belongs to one service,
//! which seeds its default in a migration and is the only one that writes it; document the keys in
//! `docs/api.md`.

use serde::{Serialize, de::DeserializeOwned};
use sqlx::SqlitePool;

use crate::util::now_ms;

/// The value of `key`, or `None` when it is missing or does not parse as `T`.
///
/// # Errors
/// On database errors.
pub async fn get<T: DeserializeOwned>(db: &SqlitePool, key: &str) -> sqlx::Result<Option<T>> {
    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM settings WHERE key = ?")
        .bind(key)
        .fetch_optional(db)
        .await?;
    Ok(row.and_then(|(v,)| serde_json::from_str(&v).ok()))
}

/// The value of `key`, or `default`.
///
/// # Errors
/// On database errors.
pub async fn get_or<T: DeserializeOwned>(
    db: &SqlitePool,
    key: &str,
    default: T,
) -> sqlx::Result<T> {
    Ok(get(db, key).await?.unwrap_or(default))
}

/// Inserts or replaces `key`.
///
/// # Errors
/// On database errors.
pub async fn set<T: Serialize + Sync>(db: &SqlitePool, key: &str, value: &T) -> sqlx::Result<()> {
    let json = serde_json::to_string(value).map_err(|e| sqlx::Error::Encode(Box::new(e)))?;
    sqlx::query(
        "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, ?)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
    )
    .bind(key)
    .bind(json)
    .bind(now_ms())
    .execute(db)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect_memory;

    #[tokio::test]
    async fn missing_values_defaults_and_updates() {
        let db = connect_memory().await.unwrap();
        assert_eq!(get::<bool>(&db, "test_flag").await.unwrap(), None);
        assert_eq!(get_or(&db, "test_limit", 10u64).await.unwrap(), 10);
        set(&db, "test_limit", &120u64).await.unwrap();
        assert_eq!(get_or(&db, "test_limit", 0u64).await.unwrap(), 120);
        set(&db, "test_limit", &5u64).await.unwrap();
        assert_eq!(get_or(&db, "test_limit", 0u64).await.unwrap(), 5);
    }
}
