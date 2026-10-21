//! HTTP routes of `/api/notes`: extract the caller, validate the input, call the store.

use app_core::{ApiError, ApiResult, Caller, Ctx, Json};
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde_json::{Value, json};

use crate::{
    model::{NewNote, Note, NotePatch, check_body, clean_title},
    store,
};

pub fn router(ctx: Ctx) -> Router {
    Router::new()
        .route("/api/notes", get(list).post(create))
        .route("/api/notes/{id}", get(show).patch(update).delete(remove))
        .with_state(ctx)
}

/// `GET /api/notes` → `{items: Note[]}`.
async fn list(State(ctx): State<Ctx>, caller: Caller) -> ApiResult<Json<Value>> {
    let items = store::list(&ctx.db, &caller.user_id).await?;
    Ok(Json(json!({ "items": items })))
}

/// `POST /api/notes` `{title, body?, pinned?}` → 201 `Note`.
async fn create(
    State(ctx): State<Ctx>,
    caller: Caller,
    Json(input): Json<NewNote>,
) -> ApiResult<(StatusCode, Json<Note>)> {
    let title = clean_title(&input.title)?;
    check_body(&input.body)?;
    let note = store::insert(&ctx.db, &caller.user_id, &title, &input.body, input.pinned).await?;
    Ok((StatusCode::CREATED, Json(note)))
}

/// `GET /api/notes/{id}` → `Note`.
async fn show(
    State(ctx): State<Ctx>,
    caller: Caller,
    Path(id): Path<String>,
) -> ApiResult<Json<Note>> {
    Ok(Json(find(&ctx, &caller, &id).await?))
}

/// `PATCH /api/notes/{id}` `{title?, body?, pinned?}` → `Note`.
async fn update(
    State(ctx): State<Ctx>,
    caller: Caller,
    Path(id): Path<String>,
    Json(patch): Json<NotePatch>,
) -> ApiResult<Json<Note>> {
    let mut note = find(&ctx, &caller, &id).await?;
    if let Some(title) = patch.title {
        note.title = clean_title(&title)?;
    }
    if let Some(body) = patch.body {
        check_body(&body)?;
        note.body = body;
    }
    if let Some(pinned) = patch.pinned {
        note.pinned = pinned;
    }
    store::update(&ctx.db, &caller.user_id, &mut note).await?;
    Ok(Json(note))
}

/// `DELETE /api/notes/{id}` → 204.
async fn remove(
    State(ctx): State<Ctx>,
    caller: Caller,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    if store::delete(&ctx.db, &caller.user_id, &id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::not_found("Note not found"))
    }
}

async fn find(ctx: &Ctx, caller: &Caller, id: &str) -> ApiResult<Note> {
    store::get(&ctx.db, &caller.user_id, id)
        .await?
        .ok_or_else(|| ApiError::not_found("Note not found"))
}
