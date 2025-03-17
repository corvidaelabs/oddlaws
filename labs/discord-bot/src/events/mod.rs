use handler::Handler;
use serenity::all::{ChannelId, GuildId, GuildMemberUpdateEvent, MessageId, Reaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::{Presence, Ready};
use serenity::model::guild::{Member, Role};
use serenity::model::id::RoleId;
use serenity::prelude::*;

pub mod handler;

const TARGET_ROLE_ID: u64 = 1350675274924294236; // "Fake Role"

#[async_trait]
impl EventHandler for Handler {
    // This event will be dispatched for guilds, but not for direct messages.
    async fn message(&self, _ctx: Context, msg: Message) {
        tracing::debug!("Received message: {:?}", msg);
        self.handle_screenshot(&msg, TARGET_ROLE_ID).await;
    }

    // As the intents set in this example, this event shall never be dispatched.
    // Try it by changing your status.
    async fn presence_update(&self, _ctx: Context, new_data: Presence) {
        tracing::debug!("Presence Update, new date {:?}", new_data);
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("Connected as {}", ready.user.name);
        tracing::info!("Enabled guilds: {:?}", self.enabled_guilds);

        // Get members in guild with role
        let members = self.get_members(ctx, TARGET_ROLE_ID).await;
        tracing::info!("Members found {:?}", members);

        // Upsert members
        for member in members {
            self.upsert_published_member(member.user.id.to_string(), member.user.name)
                .await
                .expect("Failed to upsert member");
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
