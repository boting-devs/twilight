use std::slice::Iter;

use serde::Serialize;
use twilight_model::{
    guild::{GuildFeature, Permissions},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

/// Represents a cached [`Guild`].
///
/// [`Guild`]: twilight_model::guild::Guild
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedGuild {
    pub(crate) id: Id<GuildMarker>,
    pub(crate) member_count: Option<u64>,
    pub(crate) name: String,
    pub(crate) owner_id: Id<UserMarker>,
    pub(crate) permissions: Option<Permissions>,
    pub(crate) unavailable: bool,
}

impl CachedGuild {
    /// ID of the guild.
    pub const fn id(&self) -> Id<GuildMarker> {
        self.id
    }

    /// Total number of members in the guild.
    pub const fn member_count(&self) -> Option<u64> {
        self.member_count
    }

    /// Name of the guild.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// ID of the guild's owner.
    pub const fn owner_id(&self) -> Id<UserMarker> {
        self.owner_id
    }

    /// Total permissions for the current user in the guild, excluding overwrites.
    pub const fn permissions(&self) -> Option<Permissions> {
        self.permissions
    }

    /// Whether the guild is unavailable due to an outage.
    pub const fn unavailable(&self) -> bool {
        self.unavailable
    }
}

pub struct Features<'a> {
    inner: Iter<'a, GuildFeature>,
}

impl<'a> Iterator for Features<'a> {
    type Item = &'a GuildFeature;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::{CachedGuild, Features};
    use serde::Serialize;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    // assert_fields!(
    //     CachedGuild: afk_channel_id,
    //     afk_timeout,
    //     application_id,
    //     banner,
    //     default_message_notifications,
    //     description,
    //     discovery_splash,
    //     explicit_content_filter,
    //     features,
    //     icon,
    //     id,
    //     joined_at,
    //     large,
    //     max_members,
    //     max_presences,
    //     max_video_channel_users,
    //     member_count,
    //     mfa_level,
    //     name,
    //     nsfw_level,
    //     owner_id,
    //     owner,
    //     permissions,
    //     preferred_locale,
    //     premium_progress_bar_enabled,
    //     premium_subscription_count,
    //     premium_tier,
    //     rules_channel_id,
    //     splash,
    //     system_channel_id,
    //     system_channel_flags,
    //     unavailable,
    //     vanity_url_code,
    //     verification_level,
    //     widget_channel_id,
    //     widget_enabled
    // );
    assert_impl_all!(
        CachedGuild: Clone,
        Debug,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_impl_all!(Features<'_>: Iterator, Send, Sync);
}
