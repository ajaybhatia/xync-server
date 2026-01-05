use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Bookmark {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookmark {
    #[validate(url(message = "Invalid URL format"))]
    pub url: String,
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateBookmark {
    #[validate(url(message = "Invalid URL format"))]
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>,
}
