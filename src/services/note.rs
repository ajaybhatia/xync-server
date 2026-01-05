use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{CreateNote, Note, UpdateNote};

pub struct NoteService;

impl NoteService {
    pub async fn create(pool: &PgPool, user_id: Uuid, input: CreateNote) -> Result<Note> {
        let note = sqlx::query_as::<_, Note>(
            r#"
            INSERT INTO notes (id, user_id, title, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(&input.title)
        .bind(&input.content)
        .fetch_one(pool)
        .await?;

        Ok(note)
    }

    pub async fn get_by_id(pool: &PgPool, user_id: Uuid, note_id: Uuid) -> Result<Note> {
        sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1 AND user_id = $2")
            .bind(note_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Note not found".to_string()))
    }

    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<Note>> {
        let notes = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE user_id = $1 ORDER BY updated_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(notes)
    }

    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        note_id: Uuid,
        input: UpdateNote,
    ) -> Result<Note> {
        Self::get_by_id(pool, user_id, note_id).await?;

        let note = sqlx::query_as::<_, Note>(
            r#"
            UPDATE notes
            SET title = COALESCE($3, title),
                content = COALESCE($4, content),
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(note_id)
        .bind(user_id)
        .bind(&input.title)
        .bind(&input.content)
        .fetch_one(pool)
        .await?;

        Ok(note)
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid, note_id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM notes WHERE id = $1 AND user_id = $2")
            .bind(note_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Note not found".to_string()));
        }

        Ok(())
    }
}
