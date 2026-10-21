//! Route tests: other services are replaced by `wiremock` servers through `<NAME>_URL`.

use app_core::{Ctx, registry};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
};
use serde_json::Value;
use tower::ServiceExt as _;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

/// An edge whose every service URL points at `mock`, serving the SPA from `web_dist`.
async fn edge(mock: &MockServer, web_dist: &str) -> Router {
    edge_at(&mock.uri(), web_dist).await
}

/// An edge whose every service URL is `base_url`.
async fn edge_at(base_url: &str, web_dist: &str) -> Router {
    let mut vars = vec![
        ("WEB_DIST".to_owned(), web_dist.to_owned()),
        ("APP_NAME".to_owned(), "Test \"Shop\" Магазин".to_owned()),
    ];
    for service in registry::services() {
        let key = format!("{}_URL", service.name.to_uppercase().replace('-', "_"));
        vars.push((key, base_url.to_owned()));
    }
    let pairs: Vec<(&str, &str)> = vars.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let ctx = Ctx::for_tests_with("edge", &pairs).await.unwrap();
    super::router(ctx).unwrap()
}

async fn body_json(response: axum::response::Response) -> Value {
    serde_json::from_slice(&to_bytes(response.into_body(), 1 << 20).await.unwrap()).unwrap()
}

#[tokio::test]
async fn forwards_service_routes_with_headers_and_body() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/notes"))
        .and(header("x-test", "1"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({ "id": "n1" })))
        .mount(&mock)
        .await;
    let response = edge(&mock, "/nonexistent")
        .await
        .oneshot(
            Request::post("/api/notes")
                .header("x-test", "1")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"title":"x"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 201);
    assert_eq!(body_json(response).await["id"], "n1");
}

#[tokio::test]
async fn unknown_api_paths_are_json_404() {
    let mock = MockServer::start().await;
    let response = edge(&mock, "/nonexistent")
        .await
        .oneshot(Request::get("/api/nothing").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
    assert_eq!(body_json(response).await["error"]["code"], "not_found");
}

#[tokio::test]
async fn unreachable_services_are_503() {
    // A port that was free a moment ago: nothing listens there. (A dropped MockServer would not
    // do: wiremock keeps pooled servers listening.)
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let response = edge_at(&format!("http://127.0.0.1:{port}"), "/nonexistent")
        .await
        .oneshot(Request::get("/api/notes").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 503);
}

#[tokio::test]
async fn health_reports_every_service_and_the_app_name() {
    let mock = MockServer::start().await;
    Mock::given(path("/health"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({ "version": "1.2.3" })),
        )
        .mount(&mock)
        .await;
    let response = edge(&mock, "/nonexistent")
        .await
        .oneshot(Request::get("/api/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = body_json(response).await;
    assert_eq!(body["status"], "ok");
    assert_eq!(body["appName"], "Test \"Shop\" Магазин");
    let services = body["services"].as_array().unwrap();
    assert_eq!(services.len(), registry::services().len() - 1);
    assert!(services.iter().all(|s| s["version"] == "1.2.3"));
}

#[tokio::test]
async fn serves_the_spa_with_history_fallback() {
    let mock = MockServer::start().await;
    let dist = std::env::temp_dir().join(format!("app-edge-dist-{}", app_core::new_id()));
    std::fs::create_dir_all(&dist).unwrap();
    std::fs::write(dist.join("index.html"), "<html>spa</html>").unwrap();
    let app = edge(&mock, dist.to_str().unwrap()).await;
    for uri in ["/", "/notes/123"] {
        let response = app
            .clone()
            .oneshot(Request::get(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), 200, "{uri}");
        let body = to_bytes(response.into_body(), 1 << 20).await.unwrap();
        assert_eq!(&body[..], b"<html>spa</html>");
    }
    std::fs::remove_dir_all(dist).unwrap();
}

#[tokio::test]
async fn explains_a_missing_build() {
    let mock = MockServer::start().await;
    let response = edge(&mock, "/nonexistent")
        .await
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}
