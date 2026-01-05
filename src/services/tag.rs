use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{CreateTag, Tag, UpdateTag};

pub struct TagService;

impl TagService {
    pub async fn create(pool: &PgPool, user_id: Uuid, input: CreateTag) -> Result<Tag> {
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tags WHERE user_id = $1 AND name = $2",
        )
        .bind(user_id)
        .bind(&input.name)
        .fetch_one(pool)
        .await?;

        if existing > 0 {
            return Err(AppError::Conflict("Tag already exists".to_string()));
        }

        let tag = sqlx::query_as::<_, Tag>(
            r#"
            INSERT INTO tags (id, user_id, name, color, created_at)
            VALUES ($1, $2, $3, $4, NOW())
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(&input.name)
        .bind(&input.color)
        .fetch_one(pool)
        .await?;

        Ok(tag)
    }

    pub async fn get_by_id(pool: &PgPool, user_id: Uuid, tag_id: Uuid) -> Result<Tag> {
        sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE id = $1 AND user_id = $2")
            .bind(tag_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))
    }

    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<Tag>> {
        let tags =
            sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE user_id = $1 ORDER BY name ASC")
                .bind(user_id)
                .fetch_all(pool)
                .await?;

        Ok(tags)
    }

    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        tag_id: Uuid,
        input: UpdateTag,
    ) -> Result<Tag> {
        Self::get_by_id(pool, user_id, tag_id).await?;

        let tag = sqlx::query_as::<_, Tag>(
            r#"
            UPDATE tags
            SET name = COALESCE($3, name),
                color = COALESCE($4, color)
            WHERE id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(tag_id)
        .bind(user_id)
        .bind(&input.name)
        .bind(&input.color)
        .fetch_one(pool)
        .await?;

        Ok(tag)
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid, tag_id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM tags WHERE id = $1 AND user_id = $2")
            .bind(tag_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Tag not found".to_string()));
        }

        Ok(())
    }
}
