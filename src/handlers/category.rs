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
use crate::models::{Category, CreateCategory, UpdateCategory};
use crate::services::CategoryService;

#[utoipa::path(
    post,
    path = "/api/categories",
    request_body = CreateCategory,
    responses(
        (status = 201, description = "Category created", body = Category),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Category already exists"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "categories"
)]
#[tracing::instrument(skip(pool, auth, input), fields(user_id = %auth.user_id))]
pub async fn create_category(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Json(input): Json<CreateCategory>,
) -> Result<(StatusCode, Json<Category>)> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let category = CategoryService::create(&pool, auth.user_id, input).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

#[utoipa::path(
    get,
    path = "/api/categories",
    responses(
        (status = 200, description = "List of categories", body = Vec<Category>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "categories"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id))]
pub async fn list_categories(
    State(pool): State<PgPool>,
    auth: AuthUser,
) -> Result<Json<Vec<Category>>> {
    let categories = CategoryService::list(&pool, auth.user_id).await?;
    Ok(Json(categories))
}

#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    params(
        ("id" = Uuid, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category found", body = Category),
        (status = 404, description = "Category not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "categories"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id, category_id = %id))]
pub async fn get_category(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Category>> {
    let category = CategoryService::get_by_id(&pool, auth.user_id, id).await?;
    Ok(Json(category))
}

#[utoipa::path(
    put,
    path = "/api/categories/{id}",
    params(
        ("id" = Uuid, Path, description = "Category ID")
    ),
    request_body = UpdateCategory,
    responses(
        (status = 200, description = "Category updated", body = Category),
        (status = 404, description = "Category not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "categories"
)]
#[tracing::instrument(skip(pool, auth, input), fields(user_id = %auth.user_id, category_id = %id))]
pub async fn update_category(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateCategory>,
) -> Result<Json<Category>> {
    let category = CategoryService::update(&pool, auth.user_id, id, input).await?;
    Ok(Json(category))
}

#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    params(
        ("id" = Uuid, Path, description = "Category ID")
    ),
    responses(
        (status = 204, description = "Category deleted"),
        (status = 404, description = "Category not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "categories"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id, category_id = %id))]
pub async fn delete_category(
    State(pool): State<PgPool>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    CategoryService::delete(&pool, auth.user_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
