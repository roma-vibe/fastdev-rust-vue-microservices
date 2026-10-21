//! Reverse proxy: forwards a request to the service that owns its path.

use app_core::{ApiError, registry};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use futures::TryStreamExt as _;

use crate::{EdgeState, spa};

/// Hop-by-hop headers are never forwarded (RFC 9110 §7.6.1).
const HOP_BY_HOP: [&str; 8] = [
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailer",
    "transfer-encoding",
    "upgrade",
];

fn copy_headers(from: &HeaderMap, to: &mut HeaderMap) {
    for (name, value) in from {
        if HOP_BY_HOP.contains(&name.as_str()) || name == header::HOST {
            continue;
        }
        to.append(name.clone(), value.clone());
    }
}

/// Fallback handler: a service route, a JSON 404 for unknown `/api` paths, else the SPA.
pub async fn handle(State(state): State<EdgeState>, req: Request) -> Response {
    let path = req.uri().path().to_owned();
    match registry::route(&path) {
        Some(service) => forward(&state, &service.name, req).await,
        None if path == "/api" || path.starts_with("/api/") => {
            ApiError::not_found(format!("No service handles {path}")).into_response()
        }
        None => spa::serve(&state, req).await,
    }
}

async fn forward(state: &EdgeState, service: &str, req: Request) -> Response {
    let base = state.ctx.config.service_url(service);
    let path_and_query = req
        .uri()
        .path_and_query()
        .map_or("/", |p| p.as_str())
        .to_owned();
    let url = format!("{base}{path_and_query}");
    let (parts, body) = req.into_parts();

    let mut headers = HeaderMap::new();
    copy_headers(&parts.headers, &mut headers);
    if let Some(host) = parts.headers.get(header::HOST) {
        headers.insert(HeaderName::from_static("x-forwarded-host"), host.clone());
    }
    headers.insert(
        HeaderName::from_static("x-forwarded-proto"),
        HeaderValue::from_static("http"),
    );

    let upstream = state
        .client
        .request(parts.method, &url)
        .headers(headers)
        .body(reqwest::Body::wrap_stream(body.into_data_stream()))
        .send()
        .await;
    let upstream = match upstream {
        Ok(resp) => resp,
        Err(err) => {
            tracing::warn!(%service, %url, error = %err, "service unreachable");
            return ApiError::unavailable(format!("Service {service} is unavailable"))
                .into_response();
        }
    };

    let mut response = Response::builder().status(upstream.status().as_u16());
    if let Some(out) = response.headers_mut() {
        copy_headers(upstream.headers(), out);
    }
    let stream = upstream.bytes_stream().map_err(std::io::Error::other);
    response
        .body(Body::from_stream(stream))
        .unwrap_or_else(|_| StatusCode::BAD_GATEWAY.into_response())
}
