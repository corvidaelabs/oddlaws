use serenity::all::{Context, GuildId, Member, Message};
use sqlx::PgPool;
use std::{collections::HashSet, env};

use crate::db::{published_events, published_members::PublishedMember};

pub struct Handler {
    pub enabled_guilds: HashSet<GuildId>,
    pub db_pool: PgPool,
}

impl Handler {
    pub fn new(pool: PgPool) -> Self {
        let guild_ids = env::var("ALLOWED_GUILD_IDS")
            .map(|ids| {
                ids.split(',')
                    .map(|id| {
                        GuildId::new(id.trim().parse::<u64>().expect("Guild ID must be a number"))
                    })
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default();

        Self {
            enabled_guilds: guild_ids,
            db_pool: pool,
        }
    }

    pub async fn handle_screenshot(&self, msg: &Message, target_role_id: u64) {
        const SCREENSHOTS_CHANNEL_ID: u64 = 1254537681422254202;
        // Check if message is in the screenshots channel
        if msg.channel_id.get() != SCREENSHOTS_CHANNEL_ID {
            return;
        }

        // Check if user has target role
        if let Some(member) = &msg.member {
            if !member.roles.iter().any(|role| role.get() == target_role_id) {
                return;
            }

            // Get attachments/media from message
            for attachment in &msg.attachments {
                if let Err(e) = self
                    .save_screenshot(
                        msg.author.id.to_string(),
                        msg.author.name.clone(),
                        attachment.url.clone(),
                    )
                    .await
                {
                    tracing::error!("Failed to save screenshot: {:?}", e);
                }
            }
        }
    }

    /// Saves a screenshot to the database
    async fn save_screenshot(
        &self,
        discord_id: String,
        username: String,
        url: String,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!("Saving screenshot from user: {}", username);

        let Some(member) = PublishedMember::by_discord_id(discord_id, &self.db_pool).await? else {
            return Err(sqlx::Error::RowNotFound);
        };
        let now = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO member_screenshots (
                    url,
                    member_id,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (id) DO UPDATE
                SET url = EXCLUDED.url,
                    updated_at = EXCLUDED.updated_at",
        )
        .bind(url)
        .bind(member.id)
        .bind(now)
        .bind(now)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn get_members(&self, ctx: Context, with_role_id: u64) -> Vec<Member> {
        let mut to_publish_members = Vec::new();

        for guild_id in &self.enabled_guilds {
            tracing::debug!("Attempting to fetch guild {}", guild_id);

            match guild_id.members(&ctx.http, None, None).await {
                Ok(members) => {
                    // Filter out only users with target role
                    let members_with_roles: Vec<_> = members
                        .into_iter()
                        .filter(|member| member.roles.iter().any(|role| role.get() == with_role_id))
                        .collect();
                    if members_with_roles.len() == 0 {
                        tracing::warn!("No members with target role found in guild {}", guild_id);
                        return to_publish_members;
                    }
                    to_publish_members.extend(members_with_roles.into_iter());
                }
                Err(e) => {
                    tracing::error!("Failed to fetch members for guild {}: {:?}", guild_id, e)
                }
            }
        }

        to_publish_members
    }

    /// Upserts a published member into the database
    pub async fn upsert_published_member(
        &self,
        discord_id: String,
        name: String,
    ) -> Result<(), sqlx::Error> {
        tracing::debug!("Upserting published member with name: {}", name);
        let now = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO published_members (discord_id, name, created_at, updated_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (discord_id) DO UPDATE
             SET name = EXCLUDED.name,
                 updated_at = EXCLUDED.updated_at",
        )
        .bind(discord_id)
        .bind(name)
        .bind(now)
        .bind(now)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn save_scheduled_events(
        &self,
        ctx: &Context,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for guild_id in &self.enabled_guilds {
            match guild_id.scheduled_events(&ctx.http, true).await {
                Ok(events) => {
                    for event in events {
                        let creator_id = event
                            .creator_id
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "unknown".to_string());

                        let start_time: chrono::DateTime<chrono::Utc> = *event.start_time;
                        let end_time: Option<chrono::DateTime<chrono::Utc>> =
                            event.end_time.map(|t| *t);

                        if let Err(e) = published_events::ScheduledEvent::upsert(
                            event.id.to_string(),
                            event.name,
                            event.description,
                            start_time,
                            end_time,
                            &self.db_pool,
                        )
                        .await
                        {
                            tracing::error!("Failed to save scheduled event: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to fetch scheduled events for guild {}: {:?}",
                        guild_id,
                        e
                    );
                }
            }
        }
        Ok(())
    }
}
