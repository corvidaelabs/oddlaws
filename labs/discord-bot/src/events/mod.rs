use handler::Handler;
use serenity::all::{
    ChannelId, GuildId, GuildMemberUpdateEvent, GuildScheduledEventUserAddEvent,
    GuildScheduledEventUserRemoveEvent, MessageId, Reaction, ScheduledEvent,
};
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
    /// Handles messages sent in guilds (servers) but not direct messages.
    /// Specifically checks for screenshots and handles them according to target role.
    async fn message(&self, _ctx: Context, msg: Message) {
        tracing::debug!("Received message: {:?}", msg);
        self.handle_screenshot(&msg, TARGET_ROLE_ID).await;
    }

    /// Triggered when a user's presence (status, activity) changes.
    /// Requires GUILD_PRESENCES intent to function.
    // Try it by changing your status.
    async fn presence_update(&self, _ctx: Context, new_data: Presence) {
        tracing::debug!("Presence Update, new date {:?}", new_data);
    }

    /// Triggered when the bot successfully connects to Discord and is ready to receive events.
    /// Initializes bot state and performs startup tasks like fetching initial member lists.
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

    /// Triggered when a guild member is updated (roles, nickname, etc).
    /// Provides both old and new member data if available.
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

    /// Triggered when a new role is created in a guild.
    async fn guild_role_create(&self, _ctx: Context, new: Role) {
        tracing::debug!("New role created: {} in guild", new.name);
    }

    /// Triggered when a role is deleted from a guild.
    /// Provides the role data if it was cached before deletion.
    async fn guild_role_delete(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _removed_role_id: RoleId,
        removed_role_data: Option<Role>,
    ) {
        tracing::debug!("Role deleted: {:?} in guild", removed_role_data);
    }

    /// Triggered when a role's properties are updated (name, permissions, color, etc).
    /// Provides both old and new role data if available.
    async fn guild_role_update(&self, _ctx: Context, _old_if_available: Option<Role>, new: Role) {
        tracing::debug!("Role updated: {} in guild", new.name);
    }

    /// Triggered when a user adds a reaction to a message.
    async fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        tracing::debug!("Reaction added: {:?}", reaction);
    }

    /// Triggered when a user removes their reaction from a message.
    async fn reaction_remove(&self, _ctx: Context, reaction: Reaction) {
        tracing::debug!("Reaction removed: {:?}", reaction);
    }

    /// Triggered when all reactions are removed from a message at once.
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

    /// Triggered when all reactions of a specific emoji are removed from a message.
    async fn reaction_remove_emoji(&self, _ctx: Context, reaction: Reaction) {
        tracing::debug!("All {:?} reactions removed from message", reaction.emoji);
    }

    /// Triggered when a new scheduled event is created in a guild.
    async fn guild_scheduled_event_create(&self, _ctx: Context, _event: ScheduledEvent) {
        // Handle new event creation
        todo!("Implement guild_scheduled_event_create")
    }

    /// Triggered when a scheduled event's details are updated.
    async fn guild_scheduled_event_update(&self, _ctx: Context, event: ScheduledEvent) {
        // Handle event updates
        tracing::debug!("Scheduled event updated: {:?}", event);
    }

    /// Triggered when a scheduled event is deleted from a guild.
    async fn guild_scheduled_event_delete(&self, _ctx: Context, _event: ScheduledEvent) {
        // Handle event deletion
        todo!("Implement guild_scheduled_event_delete")
    }

    /// Triggered when a user subscribes to (or joins) a scheduled event.
    async fn guild_scheduled_event_user_add(
        &self,
        _ctx: Context,
        _event: GuildScheduledEventUserAddEvent,
    ) {
        // Handle user adding themselves to an event
        todo!("Implement guild_scheduled_event_user_add")
    }

    /// Triggered when a user unsubscribes from (or leaves) a scheduled event.
    async fn guild_scheduled_event_user_remove(
        &self,
        _ctx: Context,
        _event: GuildScheduledEventUserRemoveEvent,
    ) {
        // Handle user removing themselves from an event
        todo!("Implement guild_scheduled_event_user_remove")
    }
}
