pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod metrics;
pub mod models;
pub mod services;
pub mod telemetry;

#[cfg(test)]
mod error_tests;

use axum::extract::FromRef;
use sqlx::PgPool;

pub use config::Config;
pub use db::Database;
pub use error::{AppError, Result};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt: auth::JwtManager,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for auth::JwtManager {
    fn from_ref(state: &AppState) -> Self {
        state.jwt.clone()
    }
}
