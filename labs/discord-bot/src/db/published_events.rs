use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone, Debug, Deserialize)]
pub struct ScheduledEvent {
    pub id: Uuid,
    pub discord_id: String,
    pub name: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub creator_id: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ScheduledEvent {
    pub async fn upsert(
        discord_event_id: String,
        name: String,
        description: Option<String>,
        start_time: DateTime<Utc>,
        end_time: Option<DateTime<Utc>>,
        db_pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO published_events (
                discord_id, title, description, start_time, end_time,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4::timestamptz, $5::timestamptz, $6::timestamptz, $7::timestamptz)
            ON CONFLICT (discord_id) DO UPDATE
            SET title = EXCLUDED.title,
                description = EXCLUDED.description,
                start_time = EXCLUDED.start_time,
                end_time = EXCLUDED.end_time,
                updated_at = EXCLUDED.updated_at",
        )
        .bind(discord_event_id)
        .bind(name)
        .bind(description)
        .bind(start_time)
        .bind(end_time)
        .bind(now)
        .bind(now)
        .execute(db_pool)
        .await?;

        Ok(())
    }
}
