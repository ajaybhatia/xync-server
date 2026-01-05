use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::AuthUser;
use crate::error::{AppError, Result};
use crate::models::{CreateNote, Note, UpdateNote};
use crate::services::NoteService;

#[utoipa::path(
    post,
    path = "/api/notes",
    request_body = CreateNote,
    responses(
        (status = 201, description = "Note created", body = Note),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "notes"
)]
#[tracing::instrument(skip(pool, auth, input), fields(user_id = %auth.user_id))]
pub async fn create_note(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Json(input): Json<CreateNote>,
) -> Result<(StatusCode, Json<Note>)> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let note = NoteService::create(&pool, auth.user_id, input).await?;
    Ok((StatusCode::CREATED, Json(note)))
}

#[utoipa::path(
    get,
    path = "/api/notes",
    responses(
        (status = 200, description = "List of notes", body = Vec<Note>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "notes"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id))]
pub async fn list_notes(State(pool): State<PgPool>, auth: AuthUser) -> Result<Json<Vec<Note>>> {
    let notes = NoteService::list(&pool, auth.user_id).await?;
    Ok(Json(notes))
}

#[utoipa::path(
    get,
    path = "/api/notes/{id}",
    params(
        ("id" = Uuid, Path, description = "Note ID")
    ),
    responses(
        (status = 200, description = "Note found", body = Note),
        (status = 404, description = "Note not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "notes"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id, note_id = %id))]
pub async fn get_note(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Note>> {
    let note = NoteService::get_by_id(&pool, auth.user_id, id).await?;
    Ok(Json(note))
}

#[utoipa::path(
    put,
    path = "/api/notes/{id}",
    params(
        ("id" = Uuid, Path, description = "Note ID")
    ),
    request_body = UpdateNote,
    responses(
        (status = 200, description = "Note updated", body = Note),
        (status = 404, description = "Note not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "notes"
)]
#[tracing::instrument(skip(pool, auth, input), fields(user_id = %auth.user_id, note_id = %id))]
pub async fn update_note(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateNote>,
) -> Result<Json<Note>> {
    let note = NoteService::update(&pool, auth.user_id, id, input).await?;
    Ok(Json(note))
}

#[utoipa::path(
    delete,
    path = "/api/notes/{id}",
    params(
        ("id" = Uuid, Path, description = "Note ID")
    ),
    responses(
        (status = 204, description = "Note deleted"),
        (status = 404, description = "Note not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "notes"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id, note_id = %id))]
pub async fn delete_note(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    NoteService::delete(&pool, auth.user_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
