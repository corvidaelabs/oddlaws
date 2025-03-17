use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone, Debug, Deserialize)]
pub struct PublishedMember {
    pub id: Uuid,
    discord_id: String,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl PublishedMember {
    pub async fn by_discord_id(id: String, db_pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        match sqlx::query_as::<_, Self>("SELECT * FROM published_members WHERE discord_id = $1")
            .bind(id)
            .fetch_one(db_pool)
            .await
        {
            Ok(member) => Ok(Some(member)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
