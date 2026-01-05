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
use crate::models::{CreateTag, Tag, UpdateTag};
use crate::services::TagService;

#[utoipa::path(
    post,
    path = "/api/tags",
    request_body = CreateTag,
    responses(
        (status = 201, description = "Tag created", body = Tag),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Tag already exists"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "tags"
)]
#[tracing::instrument(skip(pool, auth, input), fields(user_id = %auth.user_id))]
pub async fn create_tag(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Json(input): Json<CreateTag>,
) -> Result<(StatusCode, Json<Tag>)> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let tag = TagService::create(&pool, auth.user_id, input).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

#[utoipa::path(
    get,
    path = "/api/tags",
    responses(
        (status = 200, description = "List of tags", body = Vec<Tag>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "tags"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id))]
pub async fn list_tags(State(pool): State<PgPool>, auth: AuthUser) -> Result<Json<Vec<Tag>>> {
    let tags = TagService::list(&pool, auth.user_id).await?;
    Ok(Json(tags))
}

#[utoipa::path(
    get,
    path = "/api/tags/{id}",
    params(
        ("id" = Uuid, Path, description = "Tag ID")
    ),
    responses(
        (status = 200, description = "Tag found", body = Tag),
        (status = 404, description = "Tag not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "tags"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id, tag_id = %id))]
pub async fn get_tag(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Tag>> {
    let tag = TagService::get_by_id(&pool, auth.user_id, id).await?;
    Ok(Json(tag))
}

#[utoipa::path(
    put,
    path = "/api/tags/{id}",
    params(
        ("id" = Uuid, Path, description = "Tag ID")
    ),
    request_body = UpdateTag,
    responses(
        (status = 200, description = "Tag updated", body = Tag),
        (status = 404, description = "Tag not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "tags"
)]
#[tracing::instrument(skip(pool, auth, input), fields(user_id = %auth.user_id, tag_id = %id))]
pub async fn update_tag(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTag>,
) -> Result<Json<Tag>> {
    let tag = TagService::update(&pool, auth.user_id, id, input).await?;
    Ok(Json(tag))
}

#[utoipa::path(
    delete,
    path = "/api/tags/{id}",
    params(
        ("id" = Uuid, Path, description = "Tag ID")
    ),
    responses(
        (status = 204, description = "Tag deleted"),
        (status = 404, description = "Tag not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "tags"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id, tag_id = %id))]
pub async fn delete_tag(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    TagService::delete(&pool, auth.user_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
