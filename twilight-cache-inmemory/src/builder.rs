use std::fmt::Debug;
use std::marker::PhantomData;

use twilight_model::{
    channel::{Channel, StageInstance},
    guild::{GuildIntegration, Role},
    user::{CurrentUser, User},
};

use crate::{
    model, ChannelInterface, CurrentUserInterface, EmojiInterface, GuildIntegrationInterface,
    GuildInterface, MemberInterface, MessageInterface, PresenceInterface, RoleInterface,
    StageInstanceInterface, StickerInterface, UserInterface, VoiceStateInterface,
};

use super::{
    config::{Config, ResourceType},
    InMemoryCache,
};

/// Builder to configure and construct an [`InMemoryCache`].
#[allow(clippy::type_complexity)]
#[must_use = "has no effect if not built"]
pub struct InMemoryCacheBuilder<
    CachedChannel: ChannelInterface = Channel,
    CachedCurrentUser: CurrentUserInterface = CurrentUser,
    CachedEmoji: EmojiInterface = model::CachedEmoji,
    CachedGuild: GuildInterface = model::CachedGuild,
    CachedGuildIntegration: GuildIntegrationInterface = GuildIntegration,
    CachedMember: MemberInterface = model::CachedMember,
    CachedMessage: MessageInterface = model::CachedMessage,
    CachedPresence: PresenceInterface = model::CachedPresence,
    CachedRole: RoleInterface = Role,
    CachedStageInstance: StageInstanceInterface = StageInstance,
    CachedSticker: StickerInterface = model::CachedSticker,
    CachedUser: UserInterface = User,
    CachedVoiceState: VoiceStateInterface = model::CachedVoiceState,
>(
    Config,
    PhantomData<(
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
    )>,
);

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
    InMemoryCacheBuilder<
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
    /// Creates a builder to configure and construct an [`InMemoryCache`].
    pub const fn new() -> Self {
        Self(Config::new(), PhantomData)
    }

    /// Consume the builder, returning a configured cache.
    #[allow(clippy::type_complexity)]
    pub fn build(
        self,
    ) -> InMemoryCache<
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
    > {
        InMemoryCache::new_with_config(self.0)
    }

    /// Sets the list of resource types for the cache to handle.
    ///
    /// Defaults to all types.
    pub const fn resource_types(mut self, resource_types: ResourceType) -> Self {
        self.0.resource_types = resource_types;

        self
    }

    /// Sets the number of messages to cache per channel.
    ///
    /// Defaults to 100.
    pub const fn message_cache_size(mut self, message_cache_size: usize) -> Self {
        self.0.message_cache_size = message_cache_size;

        self
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
    > Debug
    for InMemoryCacheBuilder<
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Builder").field(&self.0).finish()
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
    > Default
    for InMemoryCacheBuilder<
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
    fn default() -> Self {
        Self(Config::default(), PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryCacheBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InMemoryCacheBuilder: Debug, Default, Send, Sync);
}
