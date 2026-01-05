use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Bookmark, CreateBookmark, UpdateBookmark};

pub struct BookmarkService;

impl BookmarkService {
    pub async fn create(pool: &PgPool, user_id: Uuid, input: CreateBookmark) -> Result<Bookmark> {
        let bookmark = sqlx::query_as::<_, Bookmark>(
            r#"
            INSERT INTO bookmarks (id, user_id, url, title, description, category_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(&input.url)
        .bind(&input.title)
        .bind(&input.description)
        .bind(input.category_id)
        .fetch_one(pool)
        .await?;

        if let Some(tag_ids) = input.tag_ids {
            for tag_id in tag_ids {
                sqlx::query(
                    "INSERT INTO bookmark_tags (bookmark_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
                )
                .bind(bookmark.id)
                .bind(tag_id)
                .execute(pool)
                .await?;
            }
        }

        Ok(bookmark)
    }

    pub async fn get_by_id(pool: &PgPool, user_id: Uuid, bookmark_id: Uuid) -> Result<Bookmark> {
        sqlx::query_as::<_, Bookmark>("SELECT * FROM bookmarks WHERE id = $1 AND user_id = $2")
            .bind(bookmark_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Bookmark not found".to_string()))
    }

    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<Bookmark>> {
        let bookmarks = sqlx::query_as::<_, Bookmark>(
            "SELECT * FROM bookmarks WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(bookmarks)
    }

    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        bookmark_id: Uuid,
        input: UpdateBookmark,
    ) -> Result<Bookmark> {
        Self::get_by_id(pool, user_id, bookmark_id).await?;

        let bookmark = sqlx::query_as::<_, Bookmark>(
            r#"
            UPDATE bookmarks
            SET url = COALESCE($3, url),
                title = COALESCE($4, title),
                description = COALESCE($5, description),
                category_id = COALESCE($6, category_id),
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(bookmark_id)
        .bind(user_id)
        .bind(&input.url)
        .bind(&input.title)
        .bind(&input.description)
        .bind(input.category_id)
        .fetch_one(pool)
        .await?;

        if let Some(tag_ids) = input.tag_ids {
            sqlx::query("DELETE FROM bookmark_tags WHERE bookmark_id = $1")
                .bind(bookmark_id)
                .execute(pool)
                .await?;

            for tag_id in tag_ids {
                sqlx::query("INSERT INTO bookmark_tags (bookmark_id, tag_id) VALUES ($1, $2)")
                    .bind(bookmark_id)
                    .bind(tag_id)
                    .execute(pool)
                    .await?;
            }
        }

        Ok(bookmark)
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid, bookmark_id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM bookmarks WHERE id = $1 AND user_id = $2")
            .bind(bookmark_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Bookmark not found".to_string()));
        }

        Ok(())
    }
}
