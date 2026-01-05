use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Serialize, ToSchema)]
pub struct ReadinessResponse {
    pub status: String,
    pub database: String,
}

#[utoipa::path(
    get,
    path = "/health/live",
    responses(
        (status = 200, description = "Service is alive", body = HealthResponse)
    ),
    tag = "health"
)]
pub async fn liveness() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[utoipa::path(
    get,
    path = "/health/ready",
    responses(
        (status = 200, description = "Service is ready", body = ReadinessResponse),
        (status = 503, description = "Service is not ready", body = ReadinessResponse)
    ),
    tag = "health"
)]
pub async fn readiness(State(pool): State<PgPool>) -> (StatusCode, Json<ReadinessResponse>) {
    let db_status = match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => "connected".to_string(),
        Err(e) => {
            tracing::error!(error = %e, "Database health check failed");
            format!("error: {}", e)
        }
    };

    let is_healthy = db_status == "connected";

    let response = ReadinessResponse {
        status: if is_healthy { "ready" } else { "not_ready" }.to_string(),
        database: db_status,
    };

    let status_code = if is_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(response))
}
