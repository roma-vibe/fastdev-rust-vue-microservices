//! The service registry (`/services.json`), embedded at compile time.
//!
//! Adding a microservice: add an entry to `services.json` (name, package, port env var, default
//! port, edge routes), create `services/<name>` and call [`crate::service::run`] with that name.

use std::sync::OnceLock;

use serde::Deserialize;

/// One entry of `services.json`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    /// Short name, e.g. `"notes"`; also the argument of [`crate::service::run`].
    pub name: String,
    /// Cargo package (and binary) name, e.g. `"app-notes"`.
    pub package: String,
    /// Env var that overrides the port, e.g. `"NOTES_PORT"`.
    pub port_env: String,
    pub default_port: u16,
    #[serde(default)]
    pub description: String,
    /// URL path prefixes the edge forwards to this service (longest prefix wins).
    #[serde(default)]
    pub routes: Vec<String>,
}

#[derive(Deserialize)]
struct RegistryFile {
    services: Vec<ServiceInfo>,
}

const RAW: &str = include_str!("../../../services.json");

/// All services from `services.json`, in file order.
///
/// # Panics
/// If the embedded `services.json` is not valid (caught by the unit tests below).
pub fn services() -> &'static [ServiceInfo] {
    static SERVICES: OnceLock<Vec<ServiceInfo>> = OnceLock::new();
    SERVICES.get_or_init(|| {
        serde_json::from_str::<RegistryFile>(RAW)
            .expect("services.json must be valid")
            .services
    })
}

/// The registry entry of `name`, if any.
#[must_use]
pub fn service(name: &str) -> Option<&'static ServiceInfo> {
    services().iter().find(|s| s.name == name)
}

/// The service that owns `path` (longest matching route prefix), for the edge router.
#[must_use]
pub fn route(path: &str) -> Option<&'static ServiceInfo> {
    services()
        .iter()
        .flat_map(|s| s.routes.iter().map(move |r| (r, s)))
        .filter(|(prefix, _)| matches_prefix(path, prefix))
        .max_by_key(|(prefix, _)| prefix.len())
        .map(|(_, s)| s)
}

/// `/api/notes` matches `/api/notes`, `/api/notes/1` and `/api/notes?x`, but not `/api/notesets`.
fn matches_prefix(path: &str, prefix: &str) -> bool {
    path.strip_prefix(prefix)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('/') || rest.starts_with('?'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_parses_with_unique_names_and_ports() {
        let all = services();
        assert!(service("edge").is_some(), "the edge is required");
        let mut names: Vec<_> = all.iter().map(|s| &s.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), all.len());
        let mut ports: Vec<_> = all.iter().map(|s| s.default_port).collect();
        ports.sort_unstable();
        ports.dedup();
        assert_eq!(ports.len(), all.len());
    }

    #[test]
    fn every_route_resolves_to_its_service() {
        for info in services() {
            for prefix in &info.routes {
                assert_eq!(route(prefix).map(|s| &s.name), Some(&info.name), "{prefix}");
                let nested = format!("{prefix}/x");
                assert_eq!(
                    route(&nested).map(|s| &s.name),
                    Some(&info.name),
                    "{nested}"
                );
            }
        }
        assert_eq!(route("/"), None);
    }

    #[test]
    fn prefixes_match_whole_segments() {
        assert!(matches_prefix("/api/notes", "/api/notes"));
        assert!(matches_prefix("/api/notes/1", "/api/notes"));
        assert!(matches_prefix("/api/notes?x=1", "/api/notes"));
        assert!(!matches_prefix("/api/notesets", "/api/notes"));
    }
}
