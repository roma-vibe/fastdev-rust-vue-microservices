# syntax=docker/dockerfile:1

# ---- Web: build the Vue app into web/dist ----
FROM node:24-alpine AS web
ENV NPM_CONFIG_UPDATE_NOTIFIER=false
WORKDIR /app
COPY package.json package-lock.json ./
COPY web/package.json web/
RUN npm ci --no-audit --no-fund
COPY tsconfig.base.json ./
COPY web web
# The web build reads APP_NAME (docker-compose.yml passes it from .env; .env is never copied).
ARG APP_NAME
RUN test -n "$APP_NAME" || (echo "Build argument APP_NAME is required" >&2 && exit 1)
RUN npm run build -w web

# ---- Rust: release builds of every service ----
FROM rust:1-bookworm AS rust
WORKDIR /app
COPY Cargo.toml Cargo.lock services.json ./
COPY crates crates
COPY services services
COPY migrations migrations
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked --workspace \
 && mkdir -p /out/bin \
 && find target/release -maxdepth 1 -type f -perm -u+x -name 'app-*' -exec cp {} /out/bin/ \;

# ---- Runtime: all service binaries + the web app, started by docker/entrypoint.sh ----
FROM debian:bookworm-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --system --create-home --uid 10001 app
WORKDIR /app
COPY --from=rust /out/bin bin
COPY --from=web /app/web/dist web/dist
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN mkdir -p data && chown app:app data
ENV APP_ROOT=/app \
    BIND_HOST=0.0.0.0 \
    WEB_DIST=/app/web/dist
USER app
CMD ["/usr/local/bin/entrypoint.sh"]
