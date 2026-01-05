use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::auth::AuthUser;
use crate::error::{AppError, Result};
use crate::models::{Bookmark, CreateBookmark, UpdateBookmark};
use crate::services::BookmarkService;

#[derive(serde::Serialize, ToSchema)]
pub struct BookmarkWithTags {
    #[serde(flatten)]
    pub bookmark: Bookmark,
    pub tags: Vec<crate::models::Tag>,
}

#[utoipa::path(
    post,
    path = "/api/bookmarks",
    request_body = CreateBookmark,
    responses(
        (status = 201, description = "Bookmark created", body = Bookmark),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "bookmarks"
)]
#[tracing::instrument(skip(pool, auth, input), fields(user_id = %auth.user_id))]
pub async fn create_bookmark(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Json(input): Json<CreateBookmark>,
) -> Result<(StatusCode, Json<Bookmark>)> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let bookmark = BookmarkService::create(&pool, auth.user_id, input).await?;

    Ok((StatusCode::CREATED, Json(bookmark)))
}

#[utoipa::path(
    get,
    path = "/api/bookmarks",
    responses(
        (status = 200, description = "List of bookmarks", body = Vec<Bookmark>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "bookmarks"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id))]
pub async fn list_bookmarks(
    State(pool): State<PgPool>,
    auth: AuthUser,
) -> Result<Json<Vec<Bookmark>>> {
    let bookmarks = BookmarkService::list(&pool, auth.user_id).await?;
    Ok(Json(bookmarks))
}

#[utoipa::path(
    get,
    path = "/api/bookmarks/{id}",
    params(
        ("id" = Uuid, Path, description = "Bookmark ID")
    ),
    responses(
        (status = 200, description = "Bookmark found", body = Bookmark),
        (status = 404, description = "Bookmark not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "bookmarks"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id, bookmark_id = %id))]
pub async fn get_bookmark(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Bookmark>> {
    let bookmark = BookmarkService::get_by_id(&pool, auth.user_id, id).await?;
    Ok(Json(bookmark))
}

#[utoipa::path(
    put,
    path = "/api/bookmarks/{id}",
    params(
        ("id" = Uuid, Path, description = "Bookmark ID")
    ),
    request_body = UpdateBookmark,
    responses(
        (status = 200, description = "Bookmark updated", body = Bookmark),
        (status = 404, description = "Bookmark not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "bookmarks"
)]
#[tracing::instrument(skip(pool, auth, input), fields(user_id = %auth.user_id, bookmark_id = %id))]
pub async fn update_bookmark(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateBookmark>,
) -> Result<Json<Bookmark>> {
    let bookmark = BookmarkService::update(&pool, auth.user_id, id, input).await?;
    Ok(Json(bookmark))
}

#[utoipa::path(
    delete,
    path = "/api/bookmarks/{id}",
    params(
        ("id" = Uuid, Path, description = "Bookmark ID")
    ),
    responses(
        (status = 204, description = "Bookmark deleted"),
        (status = 404, description = "Bookmark not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "bookmarks"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id, bookmark_id = %id))]
pub async fn delete_bookmark(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    BookmarkService::delete(&pool, auth.user_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
