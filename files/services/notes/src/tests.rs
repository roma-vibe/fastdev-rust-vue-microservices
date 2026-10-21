//! Route tests over an in-memory database (`Ctx::for_tests`).

use app_core::{Ctx, caller::test_authorization};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
};
use serde_json::{Value, json};
use tower::ServiceExt as _;

struct Client {
    app: Router,
    auth: Option<String>,
}

impl Client {
    async fn new() -> (Ctx, Self) {
        let ctx = Ctx::for_tests("notes").await.unwrap();
        let client = Self::signed_in(&ctx, "alice@example.com").await;
        (ctx, client)
    }

    async fn signed_in(ctx: &Ctx, email: &str) -> Self {
        Self {
            app: crate::routes::router(ctx.clone()),
            auth: test_authorization(ctx, email).await.unwrap(),
        }
    }

    async fn send(&self, method: Method, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
        let mut request = Request::builder().method(method).uri(uri);
        if let Some(auth) = &self.auth {
            request = request.header("authorization", auth);
        }
        let body = match body {
            Some(json) => {
                request = request.header("content-type", "application/json");
                Body::from(json.to_string())
            }
            None => Body::empty(),
        };
        let response = self
            .app
            .clone()
            .oneshot(request.body(body).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 1 << 20).await.unwrap();
        let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        (status, json)
    }
}

#[tokio::test]
async fn creates_lists_updates_and_deletes_notes() {
    let (_, client) = Client::new().await;
    let (status, created) = client
        .send(
            Method::POST,
            "/api/notes",
            Some(json!({ "title": "  First  ", "body": "Hello" })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["title"], "First");
    assert_eq!(created["pinned"], false);
    let id = created["id"].as_str().unwrap().to_owned();

    client
        .send(
            Method::POST,
            "/api/notes",
            Some(json!({ "title": "Second" })),
        )
        .await;
    let (_, patched) = client
        .send(
            Method::PATCH,
            &format!("/api/notes/{id}"),
            Some(json!({ "pinned": true })),
        )
        .await;
    assert_eq!(patched["pinned"], true);
    assert_eq!(patched["body"], "Hello");

    let (status, list) = client.send(Method::GET, "/api/notes", None).await;
    assert_eq!(status, StatusCode::OK);
    let titles: Vec<&str> = list["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n["title"].as_str().unwrap())
        .collect();
    assert_eq!(titles, ["First", "Second"], "pinned notes come first");

    let (status, _) = client
        .send(Method::DELETE, &format!("/api/notes/{id}"), None)
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _) = client
        .send(Method::GET, &format!("/api/notes/{id}"), None)
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn rejects_invalid_input_with_the_error_shape() {
    let (_, client) = Client::new().await;
    let (status, body) = client
        .send(Method::POST, "/api/notes", Some(json!({ "title": "   " })))
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "bad_request");

    let (status, _) = client
        .send(
            Method::POST,
            "/api/notes",
            Some(json!({ "body": "no title" })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, _) = client
        .send(
            Method::PATCH,
            "/api/notes/missing",
            Some(json!({ "title": "x" })),
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn notes_are_private_to_their_owner() {
    let (ctx, alice) = Client::new().await;
    let (_, note) = alice
        .send(
            Method::POST,
            "/api/notes",
            Some(json!({ "title": "Secret" })),
        )
        .await;
    let bob = Client::signed_in(&ctx, "bob@example.com").await;
    if bob.auth.is_none() {
        // A single local user: there is nobody else to hide notes from.
        return;
    }
    let (_, list) = bob.send(Method::GET, "/api/notes", None).await;
    assert_eq!(list["items"], json!([]));
    let id = note["id"].as_str().unwrap();
    let (status, _) = bob
        .send(Method::DELETE, &format!("/api/notes/{id}"), None)
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    let anonymous = Client {
        app: crate::routes::router(ctx),
        auth: None,
    };
    let (status, body) = anonymous.send(Method::GET, "/api/notes", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "unauthorized");
}
