//! Small helpers used everywhere: ids and timestamps.

use std::time::{SystemTime, UNIX_EPOCH};

/// Current time as Unix epoch milliseconds. Every `*_at` column and JSON field uses this unit.
#[must_use]
pub fn now_ms() -> i64 {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX)
}

/// A new time-sortable id (UUID v7, lowercase hyphenated) for TEXT primary keys.
#[must_use]
pub fn new_id() -> String {
    uuid::Uuid::now_v7().to_string()
}

/// Masks a secret for display: keeps up to 4 leading and 4 trailing characters.
/// `"sk-abcdef123456"` → `"sk-a…3456"`. Short secrets become `"••••"`.
#[must_use]
pub fn mask_secret(secret: &str) -> String {
    let chars: Vec<char> = secret.chars().collect();
    if chars.len() <= 10 {
        return "••••".to_owned();
    }
    let head: String = chars[..4].iter().collect();
    let tail: String = chars[chars.len() - 4..].iter().collect();
    format!("{head}…{tail}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique_and_sortable() {
        let a = new_id();
        let b = new_id();
        assert_ne!(a, b);
        assert!(a < b);
    }

    #[test]
    fn masks_secrets() {
        assert_eq!(mask_secret("sk-abcdef123456"), "sk-a…3456");
        assert_eq!(mask_secret("short"), "••••");
    }
}
