use std::env;
use twilight_cache_inmemory::{
    interfaces::GuildInterface,
    model::{
        CachedEmoji, CachedMember, CachedMessage, CachedPresence, CachedSticker, CachedVoiceState,
    },
    InMemoryCache,
};
use twilight_gateway::{Config, Event, EventTypeFlags, Intents, Shard, ShardId};
use twilight_http::Client;
use twilight_model::{
    channel::{Channel, StageInstance},
    gateway::payload::incoming::GuildUpdate,
    guild::{Guild, GuildIntegration, Role},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::{CurrentUser, User},
};

// A custom struct to store only the owner of a cached guild and the member
// count.
// This saves a lot of memory compared to storing the whole guild, which
// contains a lot of fields we will never use.
// In reality you would probably want to do this for all the cache structs.
struct MinimalCachedGuild {
    id: Id<GuildMarker>,
    owner_id: Id<UserMarker>,
    member_count: Option<u64>,
}

impl From<Guild> for MinimalCachedGuild {
    fn from(guild: Guild) -> Self {
        Self {
            id: guild.id,
            owner_id: guild.owner_id,
            member_count: guild.member_count,
        }
    }
}

impl GuildInterface for MinimalCachedGuild {
    fn id(&self) -> Id<GuildMarker> {
        self.id
    }

    fn owner_id(&self) -> Id<UserMarker> {
        self.owner_id
    }

    fn set_unavailable(&mut self, _unavailable: bool) {
        // We don't store this information, so this is a no-op
    }

    fn update_with_guild_update(&mut self, guild_update: &GuildUpdate) {
        self.id = guild_update.id;
        self.owner_id = guild_update.owner_id;
        self.member_count = guild_update.member_count;
    }

    fn increase_member_count(&mut self, amount: u64) {
        if let Some(count) = self.member_count.as_mut() {
            *count += amount;
        }
    }

    fn decrease_member_count(&mut self, amount: u64) {
        if let Some(count) = self.member_count.as_mut() {
            *count -= amount;
        }
    }
}

// Type alias for a cache that uses our minimal guild.
// We use the default types for all other entities.
type CustomInMemoryCache = InMemoryCache<
    Channel,
    CurrentUser,
    CachedEmoji,
    MinimalCachedGuild,
    GuildIntegration,
    CachedMember,
    CachedMessage,
    CachedPresence,
    Role,
    StageInstance,
    CachedSticker,
    User,
    CachedVoiceState,
>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let event_types = EventTypeFlags::MESSAGE_CREATE
        | EventTypeFlags::GUILD_CREATE
        | EventTypeFlags::GUILD_UPDATE
        | EventTypeFlags::GUILD_DELETE
        | EventTypeFlags::MEMBER_ADD
        | EventTypeFlags::MEMBER_REMOVE;

    let config = Config::builder(
        env::var("DISCORD_TOKEN")?,
        Intents::GUILDS
            | Intents::GUILD_MEMBERS
            | Intents::GUILD_MESSAGES
            | Intents::MESSAGE_CONTENT,
    )
    .event_types(event_types)
    .build();

    let mut shard = Shard::with_config(ShardId::ONE, config);

    let client = Client::new(env::var("DISCORD_TOKEN")?);

    let cache = CustomInMemoryCache::new();

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        cache.update(&event);

        if let Event::MessageCreate(msg) = event {
            if !msg.content.starts_with("!guild-info") {
                continue;
            }

            let Some(guild_id) = msg.guild_id else { continue };

            let Some(guild) = cache.guild(guild_id) else {
                continue;
            };

            let text = format!(
                "The owner of this server is <@{}>. The guild currently has {} members.",
                guild.owner_id,
                guild
                    .member_count
                    .map_or(String::from("N/A"), |c| c.to_string()),
            );

            client.create_message(msg.channel_id).content(&text).await?;
        }
    }

    Ok(())
}
