//! The session cookie: its name, reading it from requests and the `Set-Cookie` values.
//!
//! The name contains `APP_SLUG` because browsers share cookies between ports of one host: two
//! projects on `localhost` must not overwrite each other's sessions.

use axum::http::{HeaderMap, HeaderValue, header::COOKIE};

use super::token::SESSION_TTL_MS;
use crate::config::Config;

/// `<APP_SLUG>-session`, limited to characters that are valid in cookie names.
#[must_use]
pub fn session_cookie_name(config: &Config) -> String {
    let slug: String = config
        .app_slug
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    if slug.is_empty() {
        "session".to_owned()
    } else {
        format!("{slug}-session")
    }
}

/// The value of cookie `name` in the `Cookie` headers of a request.
#[must_use]
pub fn read_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get_all(COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(key, _)| *key == name)
        .map(|(_, value)| value.trim_matches('"').to_owned())
        .filter(|value| !value.is_empty())
}

/// `Set-Cookie` value that stores the session `token` (HttpOnly, `Secure` behind HTTPS).
#[must_use]
pub fn session_cookie(config: &Config, token: &str) -> HeaderValue {
    build(config, token, SESSION_TTL_MS / 1000)
}

/// `Set-Cookie` value that deletes the session cookie.
#[must_use]
pub fn clear_session_cookie(config: &Config) -> HeaderValue {
    build(config, "", 0)
}

fn build(config: &Config, value: &str, max_age_secs: i64) -> HeaderValue {
    let secure = if config.public_url.starts_with("https://") {
        "; Secure"
    } else {
        ""
    };
    let cookie = format!(
        "{}={value}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age_secs}{secure}",
        session_cookie_name(config)
    );
    // Tokens are alphanumeric and the name is sanitized, so the value is always a valid header.
    HeaderValue::from_str(&cookie).unwrap_or_else(|_| HeaderValue::from_static(""))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use super::*;

    fn config(slug: &str, public_url: &str) -> Config {
        let vars = HashMap::from([
            ("MASTER_KEY".to_owned(), "x".to_owned()),
            ("APP_SLUG".to_owned(), slug.to_owned()),
            ("PUBLIC_URL".to_owned(), public_url.to_owned()),
        ]);
        Config::from_vars(PathBuf::from("/"), vars).unwrap()
    }

    #[test]
    fn names_follow_the_slug() {
        assert_eq!(
            session_cookie_name(&config("my-app", "http://x")),
            "my-app-session"
        );
        assert_eq!(session_cookie_name(&config("§", "http://x")), "session");
    }

    #[test]
    fn reads_the_cookie_among_others() {
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_static("a=1; app-session=tok; b=2"),
        );
        assert_eq!(read_cookie(&headers, "app-session").as_deref(), Some("tok"));
        assert_eq!(read_cookie(&headers, "missing"), None);
    }

    #[test]
    fn secure_only_behind_https() {
        let plain = session_cookie(&config("app", "http://localhost:1"), "t");
        assert!(
            plain
                .to_str()
                .unwrap()
                .starts_with("app-session=t; Path=/; HttpOnly")
        );
        assert!(!plain.to_str().unwrap().contains("Secure"));
        let secure = session_cookie(&config("app", "https://example.com"), "t");
        assert!(secure.to_str().unwrap().ends_with("; Secure"));
        let cleared = clear_session_cookie(&config("app", "http://x"));
        assert!(cleared.to_str().unwrap().contains("Max-Age=0"));
    }
}
