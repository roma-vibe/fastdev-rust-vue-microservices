//! SQL over the `notes` table (owned by this service). Every query is scoped by the owner.

use app_core::{new_id, now_ms};
use sqlx::SqlitePool;

use crate::model::Note;

const COLUMNS: &str = "id, title, body, pinned, created_at, updated_at";

/// Notes of `owner`: pinned first, then the most recently updated.
pub async fn list(db: &SqlitePool, owner: &str) -> sqlx::Result<Vec<Note>> {
    sqlx::query_as(&format!(
        "SELECT {COLUMNS} FROM notes WHERE owner_id = ? ORDER BY pinned DESC, updated_at DESC"
    ))
    .bind(owner)
    .fetch_all(db)
    .await
}

/// The note `id` of `owner`, if it exists.
pub async fn get(db: &SqlitePool, owner: &str, id: &str) -> sqlx::Result<Option<Note>> {
    sqlx::query_as(&format!(
        "SELECT {COLUMNS} FROM notes WHERE id = ? AND owner_id = ?"
    ))
    .bind(id)
    .bind(owner)
    .fetch_optional(db)
    .await
}

pub async fn insert(
    db: &SqlitePool,
    owner: &str,
    title: &str,
    body: &str,
    pinned: bool,
) -> sqlx::Result<Note> {
    let now = now_ms();
    let note = Note {
        id: new_id(),
        title: title.to_owned(),
        body: body.to_owned(),
        pinned,
        created_at: now,
        updated_at: now,
    };
    sqlx::query(
        "INSERT INTO notes (id, owner_id, title, body, pinned, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&note.id)
    .bind(owner)
    .bind(&note.title)
    .bind(&note.body)
    .bind(note.pinned)
    .bind(now)
    .bind(now)
    .execute(db)
    .await?;
    Ok(note)
}

/// Saves the fields of `note` (title, body, pinned) and bumps `updated_at`.
pub async fn update(db: &SqlitePool, owner: &str, note: &mut Note) -> sqlx::Result<()> {
    note.updated_at = now_ms();
    sqlx::query(
        "UPDATE notes SET title = ?, body = ?, pinned = ?, updated_at = ?
         WHERE id = ? AND owner_id = ?",
    )
    .bind(&note.title)
    .bind(&note.body)
    .bind(note.pinned)
    .bind(note.updated_at)
    .bind(&note.id)
    .bind(owner)
    .execute(db)
    .await?;
    Ok(())
}

/// Deletes the note; `false` when `owner` has no note `id`.
pub async fn delete(db: &SqlitePool, owner: &str, id: &str) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM notes WHERE id = ? AND owner_id = ?")
        .bind(id)
        .bind(owner)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}
