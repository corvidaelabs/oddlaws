use serenity::all::{ChannelId, GuildId, GuildMemberUpdateEvent, MessageId, Reaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::{Presence, Ready};
use serenity::model::guild::{Member, Role};
use serenity::model::id::RoleId;
use serenity::prelude::*;
use std::collections::HashSet;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Default handler for the Discord bot
struct Handler {
    pub enabled_guilds: HashSet<GuildId>,
}

impl Handler {
    fn from_env() -> Self {
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
        }
    }
}
impl Default for Handler {
    fn default() -> Self {
        Self::from_env()
    }
}

#[async_trait]
impl EventHandler for Handler {
    // This event will be dispatched for guilds, but not for direct messages.
    async fn message(&self, _ctx: Context, msg: Message) {
        tracing::debug!("Received message: {:?}", msg);
    }

    // As the intents set in this example, this event shall never be dispatched.
    // Try it by changing your status.
    async fn presence_update(&self, _ctx: Context, new_data: Presence) {
        tracing::debug!("Presence Update, new date {:?}", new_data);
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("Connected as {}", ready.user.name);
        tracing::info!("Enabled guilds: {:?}", self.enabled_guilds);

        for guild_id in &self.enabled_guilds {
            tracing::info!("Attempting to fetch guild {}", guild_id);

            // Try to fetch guild members explicitly
            match guild_id.members(&ctx.http, None, None).await {
                Ok(members) => {
                    tracing::info!(
                        "Successfully fetched {} members from guild {}",
                        members.len(),
                        guild_id
                    );
                    for member in members {
                        tracing::debug!("Member: {}", member.user.name);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to fetch members for guild {}: {:?}", guild_id, e)
                }
            }
        }
    }

    // When a member's roles are updated
    async fn guild_member_update(
        &self,
        _ctx: Context,
        old_if_available: Option<Member>,
        new: Option<Member>,
        _update: GuildMemberUpdateEvent,
    ) {
        if let Some(old) = old_if_available {
            if let Some(new) = new {
                tracing::debug!(
                    "Member updated: {} ({:?} -> {:?})",
                    new.user.name,
                    old.roles,
                    new.roles
                );
            }
        }
    }

    // When a role is created
    async fn guild_role_create(&self, _ctx: Context, new: Role) {
        tracing::debug!("New role created: {} in guild", new.name);
    }

    // When a role is deleted
    async fn guild_role_delete(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _removed_role_id: RoleId,
        removed_role_data: Option<Role>,
    ) {
        tracing::debug!("Role deleted: {:?} in guild", removed_role_data);
    }

    // When a role is updated
    async fn guild_role_update(&self, _ctx: Context, _old_if_available: Option<Role>, new: Role) {
        tracing::debug!("Role updated: {} in guild", new.name);
    }

    async fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        tracing::debug!("Reaction added: {:?}", reaction);
    }

    async fn reaction_remove(&self, _ctx: Context, reaction: Reaction) {
        tracing::debug!("Reaction removed: {:?}", reaction);
    }

    async fn reaction_remove_all(
        &self,
        _ctx: Context,
        channel_id: ChannelId,
        message_id: MessageId,
    ) {
        tracing::debug!(
            "All reactions removed from message {} in channel {}",
            message_id,
            channel_id
        );
    }

    async fn reaction_remove_emoji(&self, _ctx: Context, reaction: Reaction) {
        tracing::debug!("All {:?} reactions removed from message", reaction.emoji);
    }
}

#[tokio::main]
async fn main() {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "discord_bot=debug,serenity=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Intents are a bitflag, bitwise operations can be used to dictate which intents to use
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler::default())
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
