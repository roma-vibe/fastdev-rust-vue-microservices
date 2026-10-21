//! Authentication shared by all services (the identity service creates users, sessions and keys).
//!
//! A request is signed in by
//! - the **session cookie** (`<APP_SLUG>-session`, HttpOnly, SameSite=Lax), set by the identity
//!   service at login and sent by the browser automatically, or
//! - a **personal API key** (`sk-…`) in `Authorization: Bearer <key>` or `x-api-key: <key>`, for
//!   programs and scripts (a session token in `Authorization: Bearer` works too).
//!
//! Tokens and keys are stored only as SHA-256 hashes (`sessions.token_hash`, `api_keys.key_hash`).
//! Handlers take [`AuthUser`] (any signed-in user) or [`AdminUser`] (role `admin`) as an argument.

mod cookie;
mod token;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{HeaderMap, header::AUTHORIZATION, request::Parts},
};
use serde::{Deserialize, Serialize};

pub use cookie::{clear_session_cookie, read_cookie, session_cookie, session_cookie_name};
pub use token::{
    API_KEY_PREFIX, SESSION_PREFIX, SESSION_TTL_MS, authenticate, create_session, delete_session,
};

use crate::{config::Config, error::ApiError, service::Ctx};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Admin,
}

impl Role {
    #[must_use]
    pub fn parse(value: &str) -> Self {
        if value == "admin" {
            Self::Admin
        } else {
            Self::User
        }
    }

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Admin => "admin",
        }
    }
}

/// How the request was signed in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthVia {
    Session,
    ApiKey { key_id: String },
}

/// The signed-in user of a request.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: Role,
    pub via: AuthVia,
}

impl AuthUser {
    #[must_use]
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }
}

/// A signed-in user with role `admin`; other users get 403.
#[derive(Debug, Clone)]
pub struct AdminUser(pub AuthUser);

/// The token of a request: `Authorization: Bearer x`, `x-api-key: x`, else the session cookie.
#[must_use]
pub fn request_token(headers: &HeaderMap, config: &Config) -> Option<String> {
    let header = headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok());
    if let Some(token) = header.and_then(|h| {
        h.strip_prefix("Bearer ")
            .or_else(|| h.strip_prefix("bearer "))
    }) {
        return Some(token.trim().to_owned());
    }
    if let Some(key) = headers.get("x-api-key").and_then(|v| v.to_str().ok()) {
        return Some(key.trim().to_owned());
    }
    read_cookie(headers, &session_cookie_name(config))
}

impl<S> FromRequestParts<S> for AuthUser
where
    Ctx: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = Ctx::from_ref(state);
        let Some(token) = request_token(&parts.headers, &ctx.config) else {
            return Err(ApiError::unauthorized("Sign in or pass an API key"));
        };
        authenticate(&ctx.db, &token)
            .await?
            .ok_or_else(|| ApiError::unauthorized("Invalid or expired session"))
    }
}

impl<S> FromRequestParts<S> for AdminUser
where
    Ctx: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthUser::from_request_parts(parts, state).await?;
        if !user.is_admin() {
            return Err(ApiError::forbidden("Administrator role required"));
        }
        Ok(Self(user))
    }
}
