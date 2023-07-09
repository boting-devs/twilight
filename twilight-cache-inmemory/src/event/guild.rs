use crate::{config::ResourceType, model::CachedGuild, InMemoryCache, UpdateCache};
use dashmap::DashMap;
use std::{collections::HashSet, hash::Hash};
use twilight_model::{
    gateway::payload::incoming::{GuildCreate, GuildDelete, GuildUpdate},
    guild::Guild,
    id::{marker::GuildMarker, Id},
};

impl InMemoryCache {
    #[allow(clippy::too_many_lines)]
    fn cache_guild(&self, guild: Guild) {
        let Guild {
            channels,
            id,
            member_count,
            members,
            name,
            owner_id,
            permissions,
            roles,
            threads,
            unavailable,
            voice_states,
        } = guild;

        // The map and set creation needs to occur first, so caching states and
        // objects always has a place to put them.
        if self.wants(ResourceType::CHANNEL) {
            self.guild_channels.insert(id, HashSet::new());

            let mut channels = channels;
            let mut threads = threads;

            for channel in &mut channels {
                channel.guild_id = Some(id);
            }

            for channel in &mut threads {
                channel.guild_id = Some(id);
            }

            self.cache_channels(channels);
            self.cache_channels(threads);
        }

        if self.wants(ResourceType::MEMBER_CURRENT) {
            if let Some(current_user) = self.current_user() {
                let current_member = members
                    .into_iter()
                    .find(|member| member.user.id == current_user.id);

                if let Some(member) = current_member {
                    self.guild_members.insert(guild.id, HashSet::new());
                    self.cache_member(guild.id, member);
                }
            }
        }

        if self.wants(ResourceType::ROLE) {
            self.guild_roles.insert(id, HashSet::new());
            self.cache_roles(id, roles);
        }

        if self.wants(ResourceType::VOICE_STATE) {
            self.voice_state_guilds.insert(id, HashSet::new());
            self.cache_voice_states(voice_states);
        }

        if self.wants(ResourceType::GUILD) {
            let guild = CachedGuild {
                id,
                member_count,
                name,
                owner_id,
                permissions,
                unavailable,
            };

            self.unavailable_guilds.remove(&guild.id());
            self.guilds.insert(guild.id(), guild);
        }
    }

    pub(crate) fn delete_guild(&self, id: Id<GuildMarker>, unavailable: bool) {
        fn remove_ids<T: Eq + Hash, U>(
            guild_map: &DashMap<Id<GuildMarker>, HashSet<T>>,
            container: &DashMap<T, U>,
            guild_id: Id<GuildMarker>,
        ) {
            if let Some((_, ids)) = guild_map.remove(&guild_id) {
                for id in ids {
                    container.remove(&id);
                }
            }
        }

        if self.wants(ResourceType::GUILD) {
            if unavailable {
                if let Some(mut guild) = self.guilds.get_mut(&id) {
                    guild.unavailable = true;
                }
            } else {
                self.guilds.remove(&id);
            }
        }

        if self.wants(ResourceType::CHANNEL) {
            remove_ids(&self.guild_channels, &self.channels, id);
        }

        if self.wants(ResourceType::EMOJI) {
            remove_ids(&self.guild_emojis, &self.emojis, id);
        }

        if self.wants(ResourceType::ROLE) {
            remove_ids(&self.guild_roles, &self.roles, id);
        }

        if self.wants(ResourceType::STICKER) {
            remove_ids(&self.guild_stickers, &self.stickers, id);
        }

        if self.wants(ResourceType::VOICE_STATE) {
            // Clear out a guilds voice states when a guild leaves
            self.voice_state_guilds.remove(&id);
        }

        if self.wants(ResourceType::MEMBER) || self.wants(ResourceType::MEMBER_CURRENT) {
            if let Some((_, ids)) = self.guild_members.remove(&id) {
                for user_id in ids {
                    self.members.remove(&(id, user_id));
                }
            }
        }

        if self.wants(ResourceType::PRESENCE) {
            if let Some((_, ids)) = self.guild_presences.remove(&id) {
                for user_id in ids {
                    self.presences.remove(&(id, user_id));
                }
            }
        }
    }
}

impl UpdateCache for GuildCreate {
    fn update(self, cache: &InMemoryCache) {
        cache.cache_guild(self.0);
    }
}

impl UpdateCache for GuildDelete {
    fn update(self, cache: &InMemoryCache) {
        cache.delete_guild(self.id, false);
    }
}

impl UpdateCache for GuildUpdate {
    fn update(self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::GUILD) {
            return;
        }

        if let Some(mut guild) = cache.guilds.get_mut(&self.0.id) {
            guild.name = self.0.name;
            guild.owner_id = self.0.owner_id;
            guild.permissions = self.0.permissions;
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, InMemoryCache};
    use twilight_model::{
        channel::{Channel, ChannelType},
        gateway::payload::incoming::{
            GuildCreate, GuildUpdate, MemberAdd, MemberRemove, UnavailableGuild,
        },
        guild::{Guild, PartialGuild, Permissions},
        id::Id,
        util::datetime::TimestampParseError,
    };

    #[allow(clippy::too_many_lines)]
    #[test]
    fn guild_create_channels_have_guild_ids() -> Result<(), TimestampParseError> {
        let channels = Vec::from([Channel {
            guild_id: None,
            id: Id::new(111),
            kind: ChannelType::GuildText,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(1),
            rtc_region: None,
            user_limit: None,
        }]);

        let threads = Vec::from([Channel {
            guild_id: None,
            id: Id::new(222),
            kind: ChannelType::PublicThread,
            parent_id: None,
            permission_overwrites: None,
            position: None,
            rtc_region: None,
            user_limit: None,
        }]);

        let guild = Guild {
            channels,
            id: Id::new(123),
            member_count: Some(25),
            members: Vec::new(),
            name: "this is a guild".to_owned(),
            owner_id: Id::new(456),
            permissions: Some(Permissions::SEND_MESSAGES),
            roles: Vec::new(),
            threads,
            unavailable: false,
            voice_states: Vec::new(),
        };

        let cache = InMemoryCache::new();
        cache.cache_guild(guild);

        let channel = cache.channel(Id::new(111)).unwrap();

        let thread = cache.channel(Id::new(222)).unwrap();

        // The channel was given to the cache without a guild ID, but because
        // it's part of a guild create, the cache can automatically attach the
        // guild ID to it. So now, the channel's guild ID is present with the
        // correct value.
        assert_eq!(Some(Id::new(123)), channel.guild_id);
        assert_eq!(Some(Id::new(123)), thread.guild_id);

        Ok(())
    }

    #[test]
    fn guild_update() {
        let cache = InMemoryCache::new();
        let guild = test::guild(Id::new(1), None);

        cache.update(GuildCreate(guild.clone()));

        let mutation = PartialGuild {
            id: guild.id,
            member_count: guild.member_count,
            name: "test2222".to_owned(),
            owner_id: Id::new(2),
            permissions: guild.permissions,
            roles: guild.roles,
        };

        cache.update(GuildUpdate(mutation.clone()));

        assert_eq!(cache.guild(guild.id).unwrap().name, mutation.name);
        assert_eq!(cache.guild(guild.id).unwrap().owner_id, mutation.owner_id);
        assert_eq!(cache.guild(guild.id).unwrap().id, mutation.id);
    }

    #[test]
    fn guild_member_count() {
        let user_id = Id::new(2);
        let guild_id = Id::new(1);
        let cache = InMemoryCache::new();
        let user = test::user(user_id);
        let member = test::member(user_id);
        let guild = test::guild(guild_id, Some(1));

        cache.update(GuildCreate(guild));
        cache.update(MemberAdd { guild_id, member });

        assert_eq!(cache.guild(guild_id).unwrap().member_count, Some(2));

        cache.update(MemberRemove { guild_id, user });

        assert_eq!(cache.guild(guild_id).unwrap().member_count, Some(1));
    }

    #[test]
    fn guild_members_size_after_unavailable() {
        let user_id = Id::new(2);
        let guild_id = Id::new(1);
        let cache = InMemoryCache::new();
        let member = test::member(user_id);
        let mut guild = test::guild(guild_id, Some(1));
        guild.members.push(member);

        cache.update(GuildCreate(guild.clone()));

        assert_eq!(
            1,
            cache
                .guild_members(guild_id)
                .map(|members| members.len())
                .unwrap_or_default()
        );

        cache.update(UnavailableGuild { id: guild_id });

        assert_eq!(
            0,
            cache
                .guild_members(guild_id)
                .map(|members| members.len())
                .unwrap_or_default()
        );
        assert!(cache.guild(guild_id).unwrap().unavailable);

        cache.update(GuildCreate(guild));

        assert_eq!(
            1,
            cache
                .guild_members(guild_id)
                .map(|members| members.len())
                .unwrap_or_default()
        );
        assert!(!cache.guild(guild_id).unwrap().unavailable);
    }
}
