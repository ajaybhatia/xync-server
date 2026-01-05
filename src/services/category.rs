use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{Category, CreateCategory, UpdateCategory};

pub struct CategoryService;

impl CategoryService {
    pub async fn create(pool: &PgPool, user_id: Uuid, input: CreateCategory) -> Result<Category> {
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM categories WHERE user_id = $1 AND name = $2",
        )
        .bind(user_id)
        .bind(&input.name)
        .fetch_one(pool)
        .await?;

        if existing > 0 {
            return Err(AppError::Conflict("Category already exists".to_string()));
        }

        if let Some(parent_id) = input.parent_id {
            Self::get_by_id(pool, user_id, parent_id).await?;
        }

        let category = sqlx::query_as::<_, Category>(
            r#"
            INSERT INTO categories (id, user_id, name, description, parent_id, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(input.parent_id)
        .fetch_one(pool)
        .await?;

        Ok(category)
    }

    pub async fn get_by_id(pool: &PgPool, user_id: Uuid, category_id: Uuid) -> Result<Category> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1 AND user_id = $2")
            .bind(category_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))
    }

    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<Category>> {
        let categories = sqlx::query_as::<_, Category>(
            "SELECT * FROM categories WHERE user_id = $1 ORDER BY name ASC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }

    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        category_id: Uuid,
        input: UpdateCategory,
    ) -> Result<Category> {
        Self::get_by_id(pool, user_id, category_id).await?;

        if let Some(parent_id) = input.parent_id {
            if parent_id == category_id {
                return Err(AppError::Validation(
                    "Category cannot be its own parent".to_string(),
                ));
            }
            Self::get_by_id(pool, user_id, parent_id).await?;
        }

        let category = sqlx::query_as::<_, Category>(
            r#"
            UPDATE categories
            SET name = COALESCE($3, name),
                description = COALESCE($4, description),
                parent_id = COALESCE($5, parent_id)
            WHERE id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(category_id)
        .bind(user_id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(input.parent_id)
        .fetch_one(pool)
        .await?;

        Ok(category)
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid, category_id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM categories WHERE id = $1 AND user_id = $2")
            .bind(category_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Category not found".to_string()));
        }

        Ok(())
    }
}
