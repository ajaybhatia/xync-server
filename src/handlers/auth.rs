use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use validator::Validate;

use crate::auth::{AuthUser, JwtManager};
use crate::error::{AppError, Result};
use crate::models::{CreateUser, LoginUser, UserResponse};
use crate::services::UserService;

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already registered")
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(pool, jwt, input))]
pub async fn register(
    State(pool): State<PgPool>,
    State(jwt): State<JwtManager>,
    Json(input): Json<CreateUser>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user = UserService::create(&pool, input).await?;
    let token = jwt.generate_token(user.id, &user.email)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: user.into(),
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginUser,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(pool, jwt, input))]
pub async fn login(
    State(pool): State<PgPool>,
    State(jwt): State<JwtManager>,
    Json(input): Json<LoginUser>,
) -> Result<Json<AuthResponse>> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user = UserService::authenticate(&pool, &input.email, &input.password).await?;
    let token = jwt.generate_token(user.id, &user.email)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "Current user info", body = UserResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
#[tracing::instrument(skip(pool, auth), fields(user_id = %auth.user_id))]
pub async fn me(State(pool): State<PgPool>, auth: AuthUser) -> Result<Json<UserResponse>> {
    let user = UserService::get_by_id(&pool, auth.user_id).await?;
    Ok(Json(user.into()))
}
