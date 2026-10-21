use axum::http::{Method, StatusCode};
use serde_json::json;

use super::{Browser, ctx};

#[tokio::test]
async fn the_first_user_is_admin_and_gets_a_session() {
    let ctx = ctx().await;
    let mut browser = Browser::new(&ctx);
    let (_, config) = browser.send(Method::GET, "/api/auth/config", None).await;
    assert_eq!(
        config,
        json!({ "registrationOpen": true, "hasUsers": false })
    );

    let admin = browser.register("Admin@Example.com").await;
    assert_eq!(admin["role"], "admin");
    assert_eq!(admin["email"], "admin@example.com");
    assert!(admin.get("passwordHash").is_none());
    assert!(
        browser
            .cookie
            .as_deref()
            .unwrap()
            .starts_with("app-session=sess-")
    );

    let (status, me) = browser.send(Method::GET, "/api/me", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(me["id"], admin["id"]);

    let mut second = Browser::new(&ctx);
    assert_eq!(second.register("user@example.com").await["role"], "user");
}

#[tokio::test]
async fn registration_validates_and_rejects_taken_emails() {
    let ctx = ctx().await;
    let mut browser = Browser::new(&ctx);
    let (status, _) = browser
        .send(
            Method::POST,
            "/api/auth/register",
            Some(json!({ "email": "a@example.com", "password": "short" })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    browser.register("a@example.com").await;
    let mut other = Browser::new(&ctx);
    let (status, body) = other
        .send(
            Method::POST,
            "/api/auth/register",
            Some(json!({ "email": "A@example.com", "password": "password123" })),
        )
        .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "conflict");
}

#[tokio::test]
async fn closed_registration_still_allows_the_first_user() {
    let ctx = ctx().await;
    app_core::settings::set(&ctx.db, "registration_open", &false)
        .await
        .unwrap();
    let mut first = Browser::new(&ctx);
    first.register("first@example.com").await;
    let mut second = Browser::new(&ctx);
    let (status, _) = second
        .send(
            Method::POST,
            "/api/auth/register",
            Some(json!({ "email": "second@example.com", "password": "password123" })),
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn login_logout_and_wrong_passwords() {
    let ctx = ctx().await;
    Browser::new(&ctx).register("ann@example.com").await;

    let mut browser = Browser::new(&ctx);
    let (status, _) = browser
        .send(
            Method::POST,
            "/api/auth/login",
            Some(json!({ "email": "ann@example.com", "password": "wrong-password" })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, body) = browser
        .send(
            Method::POST,
            "/api/auth/login",
            Some(json!({ "email": " ANN@example.com", "password": "password123" })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["user"]["email"], "ann@example.com");
    let session = browser.cookie.clone();

    let (status, _) = browser.send(Method::POST, "/api/auth/logout", None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert!(browser.cookie.is_none());

    browser.cookie = session;
    let (status, _) = browser.send(Method::GET, "/api/me", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "the old session is gone");
}

#[tokio::test]
async fn profile_and_password_changes() {
    let ctx = ctx().await;
    let mut browser = Browser::new(&ctx);
    browser.register("ann@example.com").await;
    let mut other_device = Browser::new(&ctx);
    other_device
        .send(
            Method::POST,
            "/api/auth/login",
            Some(json!({ "email": "ann@example.com", "password": "password123" })),
        )
        .await;

    let (_, me) = browser
        .send(Method::PATCH, "/api/me", Some(json!({ "name": "  Анна " })))
        .await;
    assert_eq!(me["name"], "Анна");

    let (status, _) = browser
        .send(
            Method::POST,
            "/api/me/password",
            Some(json!({ "currentPassword": "nope-nope", "newPassword": "newpassword1" })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, _) = browser
        .send(
            Method::POST,
            "/api/me/password",
            Some(json!({ "currentPassword": "password123", "newPassword": "newpassword1" })),
        )
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = browser.send(Method::GET, "/api/me", None).await;
    assert_eq!(status, StatusCode::OK, "the current session stays");
    let (status, _) = other_device.send(Method::GET, "/api/me", None).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "other sessions are signed out"
    );
}
