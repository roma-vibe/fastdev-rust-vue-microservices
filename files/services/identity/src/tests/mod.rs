//! Route tests over an in-memory database. [`Browser`] keeps the session cookie like a browser.

mod account;
mod admin;
mod keys;

use app_core::Ctx;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode, header},
};
use serde_json::{Value, json};
use tower::ServiceExt as _;

pub struct Browser {
    app: Router,
    pub cookie: Option<String>,
    pub bearer: Option<String>,
}

impl Browser {
    pub fn new(ctx: &Ctx) -> Self {
        Self {
            app: crate::routes::router(ctx.clone()),
            cookie: None,
            bearer: None,
        }
    }

    pub async fn send(
        &mut self,
        method: Method,
        uri: &str,
        body: Option<Value>,
    ) -> (StatusCode, Value) {
        let mut request = Request::builder().method(method).uri(uri);
        if let Some(cookie) = &self.cookie {
            request = request.header(header::COOKIE, cookie);
        }
        if let Some(bearer) = &self.bearer {
            request = request.header(header::AUTHORIZATION, format!("Bearer {bearer}"));
        }
        let body = match body {
            Some(json) => {
                request = request.header(header::CONTENT_TYPE, "application/json");
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
        if let Some(set) = response.headers().get(header::SET_COOKIE) {
            let pair = set.to_str().unwrap().split(';').next().unwrap().to_owned();
            self.cookie = if pair.ends_with('=') {
                None
            } else {
                Some(pair)
            };
        }
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 1 << 20).await.unwrap();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }

    /// Registers `email` with the password `password123` and keeps its session.
    pub async fn register(&mut self, email: &str) -> Value {
        let (status, body) = self
            .send(
                Method::POST,
                "/api/auth/register",
                Some(json!({ "email": email, "password": "password123", "name": "Test" })),
            )
            .await;
        assert_eq!(status, StatusCode::CREATED, "{body}");
        body["user"].clone()
    }
}

pub async fn ctx() -> Ctx {
    Ctx::for_tests("identity").await.unwrap()
}
