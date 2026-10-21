//! The shared SQLite database.
//!
//! All services open the same file (WAL mode, so readers never block the writer). Migrations live
//! in `/migrations` and are embedded here; every service applies pending ones on start, under a
//! file lock so services starting together do not race.
//!
//! Migration numbering (one range per owner, so parallel work never collides): `0001-0999` core,
//! then a block of 1000 per service (`1000` the first service, `2000` the next, …). File name:
//! `<number>_<snake_case>.sql`. Never edit a migration that was applied: add a new one.

use std::{fs::OpenOptions, path::Path, str::FromStr, time::Duration};

use anyhow::Context as _;
use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};

pub static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

/// Opens the database at `path` (creating the file and folder) and applies pending migrations.
///
/// # Errors
/// When the file cannot be opened or a migration fails.
pub async fn connect(path: &Path) -> anyhow::Result<SqlitePool> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(15));
    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await
        .with_context(|| format!("opening {}", path.display()))?;
    migrate(&pool, path).await?;
    Ok(pool)
}

/// An in-memory database with all migrations applied (tests).
///
/// # Errors
/// When a migration fails.
pub async fn connect_memory() -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")?.foreign_keys(true);
    // One connection: every connection to :memory: is a separate database.
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}

async fn migrate(pool: &SqlitePool, db_path: &Path) -> anyhow::Result<()> {
    let lock_path = db_path.with_extension("migrate.lock");
    let lock_file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&lock_path)
        .with_context(|| format!("opening {}", lock_path.display()))?;
    let lock_file = tokio::task::spawn_blocking(move || lock_file.lock().map(|()| lock_file))
        .await?
        .context("locking the migration lock file")?;
    let result = MIGRATOR.run(pool).await;
    drop(lock_file);
    result.context("applying migrations")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migrations_apply_on_a_fresh_database() {
        let pool = connect_memory().await.unwrap();
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(usize::try_from(count).unwrap(), MIGRATOR.iter().count());
    }

    #[tokio::test]
    async fn file_databases_are_created_and_reopened() {
        let dir = std::env::temp_dir().join(format!("app-core-db-{}", crate::new_id()));
        let path = dir.join("nested/test.sqlite");
        connect(&path).await.unwrap().close().await;
        connect(&path).await.unwrap().close().await;
        assert!(path.exists());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
