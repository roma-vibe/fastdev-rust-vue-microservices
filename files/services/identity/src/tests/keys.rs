use axum::http::{Method, StatusCode};
use serde_json::json;

use super::{Browser, ctx};

#[tokio::test]
async fn keys_are_shown_once_and_work_until_revoked() {
    let ctx = ctx().await;
    let mut browser = Browser::new(&ctx);
    browser.register("ann@example.com").await;

    let (status, created) = browser
        .send(Method::POST, "/api/keys", Some(json!({ "name": "Script" })))
        .await;
    assert_eq!(status, StatusCode::CREATED);
    let secret = created["secret"].as_str().unwrap().to_owned();
    assert!(secret.starts_with("sk-"));
    assert!(secret.starts_with(created["key"]["prefix"].as_str().unwrap()));

    let (_, list) = browser.send(Method::GET, "/api/keys", None).await;
    assert_eq!(list["items"].as_array().unwrap().len(), 1);
    assert!(list["items"][0].get("secret").is_none());

    let mut program = Browser::new(&ctx);
    program.bearer = Some(secret);
    let (status, me) = program.send(Method::GET, "/api/me", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(me["email"], "ann@example.com");

    let id = created["key"]["id"].as_str().unwrap();
    let (status, _) = browser
        .send(Method::DELETE, &format!("/api/keys/{id}"), None)
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _) = program.send(Method::GET, "/api/me", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _) = browser
        .send(Method::DELETE, &format!("/api/keys/{id}"), None)
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn keys_need_a_name_and_a_user() {
    let ctx = ctx().await;
    let mut anonymous = Browser::new(&ctx);
    let (status, _) = anonymous.send(Method::GET, "/api/keys", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let mut browser = Browser::new(&ctx);
    browser.register("ann@example.com").await;
    let (status, _) = browser
        .send(Method::POST, "/api/keys", Some(json!({ "name": "  " })))
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
