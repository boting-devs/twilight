use crate::{
    config::ResourceType,
    interfaces::{
        ChannelInterface, CurrentUserInterface, EmojiInterface, GuildIntegrationInterface,
        GuildInterface, MemberInterface, MessageInterface, PresenceInterface, RoleInterface,
        StageInstanceInterface, StickerInterface, UserInterface, VoiceStateInterface,
    },
    InMemoryCache, UpdateCache,
};
use twilight_model::{
    gateway::payload::incoming::{IntegrationCreate, IntegrationDelete, IntegrationUpdate},
    guild::GuildIntegration,
    id::{
        marker::{GuildMarker, IntegrationMarker},
        Id,
    },
};

impl<
        CachedChannel: ChannelInterface,
        CachedCurrentUser: CurrentUserInterface,
        CachedEmoji: EmojiInterface,
        CachedGuild: GuildInterface,
        CachedGuildIntegration: GuildIntegrationInterface,
        CachedMember: MemberInterface,
        CachedMessage: MessageInterface,
        CachedPresence: PresenceInterface,
        CachedRole: RoleInterface,
        CachedStageInstance: StageInstanceInterface,
        CachedSticker: StickerInterface,
        CachedUser: UserInterface,
        CachedVoiceState: VoiceStateInterface,
    >
    InMemoryCache<
        CachedChannel,
        CachedCurrentUser,
        CachedEmoji,
        CachedGuild,
        CachedGuildIntegration,
        CachedMember,
        CachedMessage,
        CachedPresence,
        CachedRole,
        CachedStageInstance,
        CachedSticker,
        CachedUser,
        CachedVoiceState,
    >
{
    fn cache_integration(&self, guild_id: Id<GuildMarker>, integration: GuildIntegration) {
        self.guild_integrations
            .entry(guild_id)
            .or_default()
            .insert(integration.id);

        crate::upsert_guild_item(
            &self.integrations,
            guild_id,
            (guild_id, integration.id),
            CachedGuildIntegration::from(integration),
        );
    }

    fn delete_integration(&self, guild_id: Id<GuildMarker>, integration_id: Id<IntegrationMarker>) {
        if self
            .integrations
            .remove(&(guild_id, integration_id))
            .is_some()
        {
            if let Some(mut integrations) = self.guild_integrations.get_mut(&guild_id) {
                integrations.remove(&integration_id);
            }
        }
    }
}

impl<
        CachedChannel: ChannelInterface,
        CachedCurrentUser: CurrentUserInterface,
        CachedEmoji: EmojiInterface,
        CachedGuild: GuildInterface,
        CachedGuildIntegration: GuildIntegrationInterface,
        CachedMember: MemberInterface,
        CachedMessage: MessageInterface,
        CachedPresence: PresenceInterface,
        CachedRole: RoleInterface,
        CachedStageInstance: StageInstanceInterface,
        CachedSticker: StickerInterface,
        CachedUser: UserInterface,
        CachedVoiceState: VoiceStateInterface,
    >
    UpdateCache<
        CachedChannel,
        CachedCurrentUser,
        CachedEmoji,
        CachedGuild,
        CachedGuildIntegration,
        CachedMember,
        CachedMessage,
        CachedPresence,
        CachedRole,
        CachedStageInstance,
        CachedSticker,
        CachedUser,
        CachedVoiceState,
    > for IntegrationCreate
{
    fn update(
        &self,
        cache: &InMemoryCache<
            CachedChannel,
            CachedCurrentUser,
            CachedEmoji,
            CachedGuild,
            CachedGuildIntegration,
            CachedMember,
            CachedMessage,
            CachedPresence,
            CachedRole,
            CachedStageInstance,
            CachedSticker,
            CachedUser,
            CachedVoiceState,
        >,
    ) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        if let Some(guild_id) = self.guild_id {
            crate::upsert_guild_item(
                &cache.integrations,
                guild_id,
                (guild_id, self.id),
                CachedGuildIntegration::from(self.0.clone()),
            );
        }
    }
}

impl<
        CachedChannel: ChannelInterface,
        CachedCurrentUser: CurrentUserInterface,
        CachedEmoji: EmojiInterface,
        CachedGuild: GuildInterface,
        CachedGuildIntegration: GuildIntegrationInterface,
        CachedMember: MemberInterface,
        CachedMessage: MessageInterface,
        CachedPresence: PresenceInterface,
        CachedRole: RoleInterface,
        CachedStageInstance: StageInstanceInterface,
        CachedSticker: StickerInterface,
        CachedUser: UserInterface,
        CachedVoiceState: VoiceStateInterface,
    >
    UpdateCache<
        CachedChannel,
        CachedCurrentUser,
        CachedEmoji,
        CachedGuild,
        CachedGuildIntegration,
        CachedMember,
        CachedMessage,
        CachedPresence,
        CachedRole,
        CachedStageInstance,
        CachedSticker,
        CachedUser,
        CachedVoiceState,
    > for IntegrationDelete
{
    fn update(
        &self,
        cache: &InMemoryCache<
            CachedChannel,
            CachedCurrentUser,
            CachedEmoji,
            CachedGuild,
            CachedGuildIntegration,
            CachedMember,
            CachedMessage,
            CachedPresence,
            CachedRole,
            CachedStageInstance,
            CachedSticker,
            CachedUser,
            CachedVoiceState,
        >,
    ) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        cache.delete_integration(self.guild_id, self.id);
    }
}

impl<
        CachedChannel: ChannelInterface,
        CachedCurrentUser: CurrentUserInterface,
        CachedEmoji: EmojiInterface,
        CachedGuild: GuildInterface,
        CachedGuildIntegration: GuildIntegrationInterface,
        CachedMember: MemberInterface,
        CachedMessage: MessageInterface,
        CachedPresence: PresenceInterface,
        CachedRole: RoleInterface,
        CachedStageInstance: StageInstanceInterface,
        CachedSticker: StickerInterface,
        CachedUser: UserInterface,
        CachedVoiceState: VoiceStateInterface,
    >
    UpdateCache<
        CachedChannel,
        CachedCurrentUser,
        CachedEmoji,
        CachedGuild,
        CachedGuildIntegration,
        CachedMember,
        CachedMessage,
        CachedPresence,
        CachedRole,
        CachedStageInstance,
        CachedSticker,
        CachedUser,
        CachedVoiceState,
    > for IntegrationUpdate
{
    fn update(
        &self,
        cache: &InMemoryCache<
            CachedChannel,
            CachedCurrentUser,
            CachedEmoji,
            CachedGuild,
            CachedGuildIntegration,
            CachedMember,
            CachedMessage,
            CachedPresence,
            CachedRole,
            CachedStageInstance,
            CachedSticker,
            CachedUser,
            CachedVoiceState,
        >,
    ) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        if let Some(guild_id) = self.guild_id {
            cache.cache_integration(guild_id, self.0.clone());
        }
    }
}
