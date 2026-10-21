//! The `users` table: the public `User` shape, lookups and validation of account fields.

use app_core::{ApiError, new_id, now_ms};
use serde::Serialize;
use sqlx::SqlitePool;

pub const MAX_NAME: usize = 100;

/// A user as returned by the API (never the password hash).
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: i64,
}

const COLUMNS: &str = "id, email, name, role, created_at";

pub async fn by_id(db: &SqlitePool, id: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as(&format!("SELECT {COLUMNS} FROM users WHERE id = ?"))
        .bind(id)
        .fetch_optional(db)
        .await
}

/// The user with `email` (case-insensitive), with the password hash and `disabled` flag.
pub async fn for_login(db: &SqlitePool, email: &str) -> sqlx::Result<Option<(User, String, bool)>> {
    let row: Option<(String, String, String, String, i64, String, bool)> = sqlx::query_as(
        "SELECT id, email, name, role, created_at, password_hash, disabled
         FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(db)
    .await?;
    Ok(
        row.map(|(id, email, name, role, created_at, hash, disabled)| {
            let user = User {
                id,
                email,
                name,
                role,
                created_at,
            };
            (user, hash, disabled)
        }),
    )
}

pub async fn count(db: &SqlitePool) -> sqlx::Result<i64> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await?;
    Ok(count)
}

/// Inserts a user; the database rejects a taken email (409 through `ApiError`).
pub async fn insert(
    db: &SqlitePool,
    email: &str,
    name: &str,
    password_hash: &str,
    role: &str,
) -> Result<User, ApiError> {
    let now = now_ms();
    let user = User {
        id: new_id(),
        email: email.to_owned(),
        name: name.to_owned(),
        role: role.to_owned(),
        created_at: now,
    };
    let result = sqlx::query(
        "INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&user.id)
    .bind(&user.email)
    .bind(&user.name)
    .bind(password_hash)
    .bind(&user.role)
    .bind(now)
    .bind(now)
    .execute(db)
    .await;
    match result {
        Ok(_) => Ok(user),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            Err(ApiError::conflict("This email is already registered"))
        }
        Err(e) => Err(e.into()),
    }
}

/// A trimmed, lowercase email that looks like `local@domain`.
pub fn clean_email(email: &str) -> Result<String, ApiError> {
    let email = email.trim().to_lowercase();
    let valid = email.len() <= 254
        && email
            .split_once('@')
            .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'));
    if valid {
        Ok(email)
    } else {
        Err(ApiError::bad_request("Enter a valid email address"))
    }
}

/// A trimmed display name of at most [`MAX_NAME`] characters.
pub fn clean_name(name: &str) -> Result<String, ApiError> {
    let name = name.trim();
    if name.chars().count() > MAX_NAME {
        return Err(ApiError::bad_request(format!(
            "Name is longer than {MAX_NAME} characters"
        )));
    }
    Ok(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emails_are_normalized_and_checked() {
        assert_eq!(clean_email(" Ann@Example.COM ").unwrap(), "ann@example.com");
        assert!(clean_email("ann").is_err());
        assert!(clean_email("@example.com").is_err());
        assert!(clean_email("ann@localhost").is_err());
    }

    #[test]
    fn names_are_trimmed() {
        assert_eq!(clean_name("  Ann ").unwrap(), "Ann");
        assert!(clean_name(&"x".repeat(MAX_NAME + 1)).is_err());
    }
}
