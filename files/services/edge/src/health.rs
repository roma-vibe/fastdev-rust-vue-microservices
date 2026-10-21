//! `GET /api/health`: `{status: "ok"|"degraded", appName, services: [{name, ok, latencyMs, version}]}`.

use std::time::{Duration, Instant};

use app_core::{Json, registry};
use axum::extract::State;
use serde::Serialize;

use crate::EdgeState;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceHealth {
    name: String,
    ok: bool,
    latency_ms: u64,
    version: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthAll {
    status: &'static str,
    app_name: String,
    services: Vec<ServiceHealth>,
}

pub async fn health_all(State(state): State<EdgeState>) -> Json<HealthAll> {
    let checks = registry::services()
        .iter()
        .filter(|s| s.name != "edge")
        .map(|s| {
            let url = format!("{}/health", state.ctx.config.service_url(&s.name));
            check(state.client.clone(), s.name.clone(), url)
        });
    let mut services = futures::future::join_all(checks).await;
    services.sort_by(|a, b| a.name.cmp(&b.name));
    let status = if services.iter().all(|s| s.ok) {
        "ok"
    } else {
        "degraded"
    };
    Json(HealthAll {
        status,
        app_name: state.ctx.config.app_name.clone(),
        services,
    })
}

async fn check(client: reqwest::Client, name: String, url: String) -> ServiceHealth {
    let started = Instant::now();
    let result = client.get(url).timeout(Duration::from_secs(2)).send().await;
    let latency_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let (ok, version) = match result {
        Ok(resp) if resp.status().is_success() => {
            let body: serde_json::Value = resp.json().await.unwrap_or_default();
            let version = body
                .get("version")
                .and_then(|v| v.as_str())
                .map(str::to_owned);
            (true, version)
        }
        _ => (false, None),
    };
    ServiceHealth {
        name,
        ok,
        latency_ms,
        version,
    }
}
