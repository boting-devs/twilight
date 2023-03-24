use crate::{
    config::ResourceType,
    interfaces::{
        ChannelInterface, CurrentUserInterface, EmojiInterface, GuildIntegrationInterface,
        GuildInterface, MemberInterface, MessageInterface, PresenceInterface, RoleInterface,
        StageInstanceInterface, StickerInterface, UserInterface, VoiceStateInterface,
    },
    InMemoryCache, UpdateCache,
};
use twilight_model::gateway::payload::incoming::{
    ThreadCreate, ThreadDelete, ThreadListSync, ThreadUpdate,
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
    > for ThreadCreate
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
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
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
    > for ThreadDelete
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
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.delete_channel(self.id);
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
    > for ThreadListSync
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
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channels(self.threads.clone());
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
    > for ThreadUpdate
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
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
    }
}
