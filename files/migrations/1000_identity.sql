-- [identity] accounts. The first registered user becomes admin.
CREATE TABLE users (
  id            TEXT PRIMARY KEY,
  email         TEXT NOT NULL UNIQUE COLLATE NOCASE,
  name          TEXT NOT NULL DEFAULT '',
  password_hash TEXT NOT NULL,                       -- argon2id PHC string
  role          TEXT NOT NULL DEFAULT 'user' CHECK (role IN ('user', 'admin')),
  disabled      INTEGER NOT NULL DEFAULT 0,
  created_at    INTEGER NOT NULL,
  updated_at    INTEGER NOT NULL
);

-- [identity] web sessions; the token is stored as SHA-256 hex (app_core::crypto::hash_token).
-- Every service may update last_seen_at / expires_at when it authenticates a request.
CREATE TABLE sessions (
  token_hash   TEXT PRIMARY KEY,
  user_id      TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  created_at   INTEGER NOT NULL,
  expires_at   INTEGER NOT NULL,
  last_seen_at INTEGER NOT NULL,
  user_agent   TEXT
);
CREATE INDEX sessions_user ON sessions (user_id);

-- [identity] personal API keys "sk-…" for programs. Every service may update last_used_at.
CREATE TABLE api_keys (
  id           TEXT PRIMARY KEY,
  user_id      TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  name         TEXT NOT NULL,
  prefix       TEXT NOT NULL,                        -- first characters, shown in the UI ("sk-AbCd…")
  key_hash     TEXT NOT NULL UNIQUE,
  created_at   INTEGER NOT NULL,
  last_used_at INTEGER,
  revoked_at   INTEGER
);
CREATE INDEX api_keys_user ON api_keys (user_id);

-- [identity] whether anyone may register (the first user always can).
INSERT INTO settings (key, value, updated_at) VALUES ('registration_open', 'true', 0);
