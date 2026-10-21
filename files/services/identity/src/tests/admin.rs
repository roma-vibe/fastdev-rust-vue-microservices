use axum::http::{Method, StatusCode};
use serde_json::json;

use super::{Browser, ctx};

#[tokio::test]
async fn admins_list_and_manage_users() {
    let ctx = ctx().await;
    let mut admin = Browser::new(&ctx);
    let me = admin.register("admin@example.com").await;
    let mut user = Browser::new(&ctx);
    let other = user.register("user@example.com").await;

    let (status, _) = user.send(Method::GET, "/api/admin/users", None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    let (status, list) = admin.send(Method::GET, "/api/admin/users", None).await;
    assert_eq!(status, StatusCode::OK);
    let items = list["items"].as_array().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[1]["keyCount"], 0);
    assert_eq!(items[1]["disabled"], false);

    let own = format!("/api/admin/users/{}", me["id"].as_str().unwrap());
    let (status, _) = admin
        .send(Method::PATCH, &own, Some(json!({ "disabled": true })))
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let path = format!("/api/admin/users/{}", other["id"].as_str().unwrap());
    let (status, row) = admin
        .send(Method::PATCH, &path, Some(json!({ "disabled": true })))
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(row["disabled"], true);
    let (status, _) = user.send(Method::GET, "/api/me", None).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "disabled users are signed out"
    );
    let (status, _) = user
        .send(
            Method::POST,
            "/api/auth/login",
            Some(json!({ "email": "user@example.com", "password": "password123" })),
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    let (_, row) = admin
        .send(
            Method::PATCH,
            &path,
            Some(json!({ "role": "admin", "disabled": false })),
        )
        .await;
    assert_eq!(row["role"], "admin");
    let (status, _) = admin
        .send(Method::PATCH, &path, Some(json!({ "role": "owner" })))
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
