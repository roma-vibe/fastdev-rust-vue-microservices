//! Password hashing with argon2id (on the blocking pool: hashing takes tens of milliseconds).

use anyhow::anyhow;
use app_core::ApiError;
use argon2::{
    Argon2, PasswordHash, PasswordHasher as _, PasswordVerifier as _,
    password_hash::{SaltString, rand_core::OsRng},
};

pub const MIN_LENGTH: usize = 8;
pub const MAX_LENGTH: usize = 256;

/// Rejects passwords that are too short or too long.
pub fn check(password: &str) -> Result<(), ApiError> {
    let len = password.chars().count();
    if len < MIN_LENGTH {
        return Err(ApiError::bad_request(format!(
            "Password must have at least {MIN_LENGTH} characters"
        )));
    }
    if len > MAX_LENGTH {
        return Err(ApiError::bad_request("Password is too long"));
    }
    Ok(())
}

/// The argon2id PHC string of `password`.
pub async fn hash(password: &str) -> Result<String, ApiError> {
    let password = password.to_owned();
    let hashed = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| anyhow!("hashing a password: {e}"))
    })
    .await
    .map_err(anyhow::Error::from)??;
    Ok(hashed)
}

/// Whether `password` matches the PHC string `hash` (an empty or broken hash never matches).
pub async fn verify(password: &str, hash: &str) -> bool {
    let password = password.to_owned();
    let hash = hash.to_owned();
    tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash).is_ok_and(|parsed| {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok()
        })
    })
    .await
    .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hashes_verify_only_the_same_password() {
        let hashed = hash("correct horse").await.unwrap();
        assert!(hashed.starts_with("$argon2id$"));
        assert!(verify("correct horse", &hashed).await);
        assert!(!verify("wrong horse", &hashed).await);
        assert!(!verify("anything", "").await);
    }

    #[test]
    fn length_limits() {
        assert!(check("1234567").is_err());
        assert!(check("12345678").is_ok());
        assert!(check(&"x".repeat(MAX_LENGTH + 1)).is_err());
    }
}
