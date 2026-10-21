//! `/api/auth/*`: registration, login and logout. A successful login sets the session cookie
//! (`app_core::auth::session_cookie`); the browser sends it with every later request.

use app_core::{
    ApiError, ApiResult, Ctx, Json,
    auth::{clear_session_cookie, create_session, delete_session, request_token, session_cookie},
    settings,
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{password, users};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfig {
    registration_open: bool,
    has_users: bool,
}

/// `GET /api/auth/config` (public) → `{registrationOpen, hasUsers}`.
pub async fn config(State(ctx): State<Ctx>) -> ApiResult<Json<AuthConfig>> {
    let has_users = users::count(&ctx.db).await? > 0;
    let registration_open = !has_users || registration_open(&ctx).await?;
    Ok(Json(AuthConfig {
        registration_open,
        has_users,
    }))
}

async fn registration_open(ctx: &Ctx) -> sqlx::Result<bool> {
    settings::get_or(&ctx.db, "registration_open", true).await
}

#[derive(Deserialize)]
pub struct RegisterInput {
    email: String,
    password: String,
    #[serde(default)]
    name: String,
}

/// `POST /api/auth/register` `{email, password, name?}` → 201 `{user}` + session cookie.
/// The first user becomes `admin`; later ones need the `registration_open` setting.
pub async fn register(
    State(ctx): State<Ctx>,
    headers: HeaderMap,
    Json(input): Json<RegisterInput>,
) -> ApiResult<Response> {
    let email = users::clean_email(&input.email)?;
    let name = users::clean_name(&input.name)?;
    password::check(&input.password)?;
    let first = users::count(&ctx.db).await? == 0;
    if !first && !registration_open(&ctx).await? {
        return Err(ApiError::forbidden("Registration is closed"));
    }
    let hash = password::hash(&input.password).await?;
    let role = if first { "admin" } else { "user" };
    let user = users::insert(&ctx.db, &email, &name, &hash, role).await?;
    signed_in(&ctx, &headers, StatusCode::CREATED, user).await
}

#[derive(Deserialize)]
pub struct LoginInput {
    email: String,
    password: String,
}

/// `POST /api/auth/login` `{email, password}` → `{user}` + session cookie; 401 on wrong credentials.
pub async fn login(
    State(ctx): State<Ctx>,
    headers: HeaderMap,
    Json(input): Json<LoginInput>,
) -> ApiResult<Response> {
    let wrong = || ApiError::unauthorized("Wrong email or password");
    let email = input.email.trim().to_lowercase();
    let Some((user, hash, disabled)) = users::for_login(&ctx.db, &email).await? else {
        return Err(wrong());
    };
    if !password::verify(&input.password, &hash).await {
        return Err(wrong());
    }
    if disabled {
        return Err(ApiError::forbidden("This account is disabled"));
    }
    signed_in(&ctx, &headers, StatusCode::OK, user).await
}

/// `POST /api/auth/logout` → 204; deletes the current session and the cookie.
pub async fn logout(State(ctx): State<Ctx>, headers: HeaderMap) -> ApiResult<Response> {
    if let Some(token) = request_token(&headers, &ctx.config) {
        delete_session(&ctx.db, &token).await?;
    }
    let cookie = clear_session_cookie(&ctx.config);
    Ok((StatusCode::NO_CONTENT, [(header::SET_COOKIE, cookie)]).into_response())
}

async fn signed_in(
    ctx: &Ctx,
    headers: &HeaderMap,
    status: StatusCode,
    user: users::User,
) -> ApiResult<Response> {
    let agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.chars().take(300).collect::<String>());
    let token = create_session(&ctx.db, &user.id, agent.as_deref()).await?;
    let cookie = session_cookie(&ctx.config, &token);
    Ok((
        status,
        [(header::SET_COOKIE, cookie)],
        Json(json!({ "user": user })),
    )
        .into_response())
}
