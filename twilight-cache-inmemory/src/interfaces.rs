//! Traits for implementing a [`InMemoryCache`] with custom structs.
//!
//! By default, the cache uses widely compatible default types that contain almost all
//! fields that are present in the Discord API. Fields that are never used by the user
//! will result in excess memory usage that will especially matter to big bots with a
//! lot of cached data.
//!
//! The traits in this module allow creating custom cached representations of Discord
//! API models compatible with the [`InMemoryCache`]. They may be mixed with the default
//! types provided by twilight, which also implement these traits.
//!
//! Many traits require getters for certain types, which means they are used for caching
//! logic. However, users generally won't have to store all the fields. It is possible
//! to return `None` or empty arrays on most of the methods if the data that is accessed
//! is not stored in the custom implementation.
//!
//! [`InMemoryCache`]: crate::InMemoryCache

use twilight_model::{
    application::interaction::application_command::InteractionMember,
    channel::{
        message::{Reaction, Sticker},
        Channel, ChannelType, Message, StageInstance,
    },
    gateway::{
        payload::incoming::{GuildUpdate, MemberUpdate, MessageUpdate},
        presence::Presence,
    },
    guild::{Emoji, Guild, GuildIntegration, Member, PartialMember, Role},
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, StickerMarker, UserMarker},
        Id,
    },
    user::{CurrentUser, User},
    util::{ImageHash, Timestamp},
    voice::VoiceState,
};
#[cfg(feature = "permission-calculator")]
use twilight_model::{channel::permission_overwrite::PermissionOverwrite, guild::Permissions};

use crate::model::member::ComputedInteractionMemberFields;

/// Interface for a generic cached representation of a [`Member`].
pub trait MemberInterface:
    From<Member>
    + From<(
        Id<UserMarker>,
        InteractionMember,
        ComputedInteractionMemberFields,
    )> + From<(Id<UserMarker>, PartialMember)>
    + PartialEq<Member>
    + PartialEq<PartialMember>
    + PartialEq<InteractionMember>
{
    /// Roles of this member.
    fn roles(&self) -> &[Id<RoleMarker>];

    /// Timestamp until which this member's communication is disabled.
    #[cfg(feature = "permission-calculator")]
    fn communication_disabled_until(&self) -> Option<Timestamp>;

    /// Avatar of this member.
    fn avatar(&self) -> Option<ImageHash>;

    /// Whether this member is deafened.
    fn deaf(&self) -> Option<bool>;

    /// Whether this member is muted.
    fn mute(&self) -> Option<bool>;

    /// Update the cached data with a [`MemberUpdate`] event.
    fn update_with_member_update(&mut self, member_update: &MemberUpdate);
}

/// Interface for a generic cached representation of a [`Role`].
pub trait RoleInterface: From<Role> + PartialEq<Self> {
    /// Role's position in the guild roles.
    fn position(&self) -> i64;

    /// ID of the role.
    fn id(&self) -> Id<RoleMarker>;

    /// Permissions granted to members with the role.
    #[cfg(feature = "permission-calculator")]
    fn permissions(&self) -> Permissions;
}

impl RoleInterface for Role {
    fn position(&self) -> i64 {
        self.position
    }

    fn id(&self) -> Id<RoleMarker> {
        self.id
    }

    #[cfg(feature = "permission-calculator")]
    fn permissions(&self) -> Permissions {
        self.permissions
    }
}

/// Interface for a generic cached representation of a [`Channel`].
pub trait ChannelInterface: From<Channel> {
    /// ID of the guild this channel belongs to.
    fn guild_id(&self) -> Option<Id<GuildMarker>>;

    /// Type of the channel.
    fn kind(&self) -> ChannelType;

    /// ID of the parent channel if this is a thread.
    #[cfg(feature = "permission-calculator")]
    fn parent_id(&self) -> Option<Id<ChannelMarker>>;

    /// ID of the channel.
    fn id(&self) -> Id<ChannelMarker>;

    /// Permission overwrites for the channel.
    #[cfg(feature = "permission-calculator")]
    fn permission_overwrites(&self) -> Option<&[PermissionOverwrite]>;

    /// Set the last pin timestamp to a new timestamp.
    fn set_last_pin_timestamp(&mut self, timestamp: Option<Timestamp>);
}

impl ChannelInterface for Channel {
    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }

    fn kind(&self) -> ChannelType {
        self.kind
    }

    #[cfg(feature = "permission-calculator")]
    fn parent_id(&self) -> Option<Id<ChannelMarker>> {
        self.parent_id
    }

    fn id(&self) -> Id<ChannelMarker> {
        self.id
    }

    #[cfg(feature = "permission-calculator")]
    fn permission_overwrites(&self) -> Option<&[PermissionOverwrite]> {
        self.permission_overwrites.as_deref()
    }

    fn set_last_pin_timestamp(&mut self, timestamp: Option<Timestamp>) {
        self.last_pin_timestamp = timestamp;
    }
}

/// Interface for a generic cached representation of a [`Guild`].
pub trait GuildInterface: From<Guild> {
    /// ID of the guild.
    fn id(&self) -> Id<GuildMarker>;

    /// ID of the guild's owner.
    #[cfg(feature = "permission-calculator")]
    fn owner_id(&self) -> Id<UserMarker>;

    /// Set the guild's unavailable flag.
    fn set_unavailable(&mut self, unavailable: bool);

    /// Update the cached data with a [`GuildUpdate`] event. Fields containing other
    /// cached structures such as channels are cleared prior.
    fn update_with_guild_update(&mut self, guild_update: &GuildUpdate);

    /// Increase the guild member count.
    fn increase_member_count(&mut self, amount: u64);

    /// Decrease the guild member count.
    fn decrease_member_count(&mut self, amount: u64);
}

/// Interface for a generic cached representation of a [`VoiceState`].
pub trait VoiceStateInterface: From<(Id<ChannelMarker>, Id<GuildMarker>, VoiceState)> {
    /// ID of the channel this voice state belongs to.
    fn channel_id(&self) -> Id<ChannelMarker>;
}

/// Interface for a generic cached representation of a [`Message`].
pub trait MessageInterface: From<Message> {
    /// Update the cached data with a [`MessageUpdate`] event.
    fn update_with_message_update(&mut self, message_update: &MessageUpdate);

    /// Reactions added to this message.
    fn reactions(&self) -> &[Reaction];

    /// Mutable getter for reactions added to this message.
    fn reactions_mut(&mut self) -> &mut [Reaction];

    /// Retain all reactions to this message matching a predicate, removing non-matching ones.
    fn retain_reactions(&mut self, f: impl FnMut(&Reaction) -> bool);

    /// Clear all reactions to this message.
    fn clear_reactions(&mut self);

    /// Add a reaction to this message.
    fn add_reaction(&mut self, reaction: Reaction);

    /// Remove a reaction from this message.
    fn remove_reaction(&mut self, idx: usize);
}

/// Interface for a generic cached representation of a [`CurrentUser`].
pub trait CurrentUserInterface: From<CurrentUser> + Clone {
    /// ID of the user.
    fn id(&self) -> Id<UserMarker>;
}

impl CurrentUserInterface for CurrentUser {
    fn id(&self) -> Id<UserMarker> {
        self.id
    }
}

/// Interface for a generic cached representation of a [`Sticker`].
pub trait StickerInterface: From<Sticker> + PartialEq<Sticker> {
    /// ID of the sticker.
    fn id(&self) -> Id<StickerMarker>;
}

/// Interface for a generic cached representation of a [`Emoji`].
pub trait EmojiInterface: From<Emoji> + PartialEq<Emoji> {}

/// Interface for a generic cached representation of a [`GuildIntegration`].
pub trait GuildIntegrationInterface: From<GuildIntegration> + PartialEq<Self> {}

impl GuildIntegrationInterface for GuildIntegration {}

/// Interface for a generic cached representation of a [`Presence`].
pub trait PresenceInterface: From<Presence> {}

/// Interface for a generic cached representation of a [`StageInstance`].
pub trait StageInstanceInterface: From<StageInstance> + PartialEq<Self> {}

impl StageInstanceInterface for StageInstance {}

/// Interface for a generic cached representation of a [`UserInterface`].
pub trait UserInterface: From<User> + PartialEq<User> + Clone {}

impl UserInterface for User {}
