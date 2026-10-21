# Changelog

## 1.0.0 — 2026-10-21

- Initial version: Rust microservices (axum, sqlx/SQLite) behind an edge reverse proxy, with a Vue 3 SPA (Vite, Pinia, Vue Router, vue-i18n, Tailwind CSS 4).
- Shared core crate: .env config, services.json registry, SQLite pool and migrations, JSON errors, AES-GCM crypto, settings and a one-call service bootstrap.
- Notes example service and web module that go through every layer, with Rust and web tests.
- Optional Accounts feature (on by default): identity service with registration (first user is admin), sessions, profile, personal API keys and an administration section with a user list; off means one local user.
- Public landing page at / and the application under /app; sidebar that can be hidden; smooth fade between pages and a smooth modal blur; delayed loader instead of a stretched spinner.
- Liquid glass design with an orange accent and favicon, light/dark/system themes, Russian and English UI, shared UI kit and auto-discovered feature modules.
- Docker modes none or run (production image with all service binaries and the SPA), data in a Docker volume or the project folder.
- Verified: default choices, No Docker, Docker with the project folder; Accounts off checked by hand with a project named Test "Shop" Магазин.
