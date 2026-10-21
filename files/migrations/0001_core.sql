-- Core schema. Conventions for every migration:
-- - The owner of each table is named in brackets: only that service WRITES it (others may read).
-- - Times are Unix epoch milliseconds, ids are UUID v7 text, booleans INTEGER 0/1, JSON as text.
-- - Numbering: 0001-0999 core, then one block of 1000 per service (see crates/core/src/db.rs).

-- [each key's service] application settings (key → JSON value). Each service seeds its own keys
-- in its migration and documents them in docs/api.md; read them with app_core::settings.
CREATE TABLE settings (
  key        TEXT PRIMARY KEY,
  value      TEXT NOT NULL,
  updated_at INTEGER NOT NULL
);
