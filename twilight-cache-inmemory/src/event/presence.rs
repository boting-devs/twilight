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
    gateway::{payload::incoming::PresenceUpdate, presence::Presence},
    id::{marker::GuildMarker, Id},
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
    pub(crate) fn cache_presences(
        &self,
        guild_id: Id<GuildMarker>,
        presences: impl IntoIterator<Item = Presence>,
    ) {
        for presence in presences {
            self.cache_presence(guild_id, presence);
        }
    }

    fn cache_presence(&self, guild_id: Id<GuildMarker>, presence: Presence) {
        self.guild_presences
            .entry(guild_id)
            .or_default()
            .insert(presence.user.id());

        self.presences.insert(
            (guild_id, presence.user.id()),
            CachedPresence::from(presence),
        );
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
    > for PresenceUpdate
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
        if !cache.wants(ResourceType::PRESENCE) {
            return;
        }

        cache.cache_presence(self.guild_id, self.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, DefaultInMemoryCache};
    use twilight_model::{
        gateway::{
            event::Event,
            payload::incoming::PresenceUpdate,
            presence::{ClientStatus, Presence, Status, UserOrId},
        },
        id::Id,
    };

    #[test]
    fn presence_update() {
        let cache = DefaultInMemoryCache::new();

        let guild_id = Id::new(1);
        let user_id = Id::new(1);

        let payload = PresenceUpdate(Presence {
            activities: Vec::new(),
            client_status: ClientStatus {
                desktop: Some(Status::Online),
                mobile: None,
                web: None,
            },
            guild_id,
            status: Status::Online,
            user: UserOrId::User(test::user(user_id)),
        });
        cache.update(&Event::PresenceUpdate(Box::new(payload)));

        assert_eq!(1, cache.presences.len());
        assert_eq!(1, cache.guild_presences.len());
        assert!(cache
            .guild_presences
            .get(&guild_id)
            .unwrap()
            .contains(&user_id));
    }
}
