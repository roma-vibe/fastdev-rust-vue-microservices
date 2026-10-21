//! Types of the notes API and their validation.

use app_core::ApiError;
use serde::{Deserialize, Serialize};

pub const MAX_TITLE: usize = 200;
pub const MAX_BODY: usize = 20_000;

/// A note as returned by the API.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub pinned: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// `POST /api/notes` body.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNote {
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub pinned: bool,
}

/// `PATCH /api/notes/{id}` body: only the given fields change.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotePatch {
    pub title: Option<String>,
    pub body: Option<String>,
    pub pinned: Option<bool>,
}

/// A trimmed, non-empty title of at most [`MAX_TITLE`] characters.
pub fn clean_title(title: &str) -> Result<String, ApiError> {
    let title = title.trim();
    if title.is_empty() {
        return Err(ApiError::bad_request("Title is required"));
    }
    if title.chars().count() > MAX_TITLE {
        return Err(ApiError::bad_request(format!(
            "Title is longer than {MAX_TITLE} characters"
        )));
    }
    Ok(title.to_owned())
}

/// A body of at most [`MAX_BODY`] characters.
pub fn check_body(body: &str) -> Result<(), ApiError> {
    if body.chars().count() > MAX_BODY {
        return Err(ApiError::bad_request(format!(
            "Text is longer than {MAX_BODY} characters"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titles_are_trimmed_and_required() {
        assert_eq!(clean_title("  Hello ").unwrap(), "Hello");
        assert!(clean_title("   ").is_err());
        assert!(clean_title(&"x".repeat(MAX_TITLE + 1)).is_err());
        assert!(clean_title(&"я".repeat(MAX_TITLE)).is_ok());
    }

    #[test]
    fn bodies_have_a_limit() {
        assert!(check_body("").is_ok());
        assert!(check_body(&"x".repeat(MAX_BODY + 1)).is_err());
    }
}
