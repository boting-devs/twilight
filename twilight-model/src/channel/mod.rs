pub mod forum;
pub mod message;
pub mod permission_overwrite;
pub mod stage_instance;
pub mod thread;
pub mod webhook;

mod attachment;
mod channel_mention;
mod channel_type;
mod flags;
mod followed_channel;
mod video_quality_mode;

pub use self::{
    attachment::Attachment,
    channel_mention::ChannelMention,
    channel_type::ChannelType,
    flags::ChannelFlags,
    followed_channel::FollowedChannel,
    message::Message,
    stage_instance::StageInstance,
    video_quality_mode::VideoQualityMode,
    webhook::{Webhook, WebhookType},
};

use crate::{
    channel::permission_overwrite::PermissionOverwrite,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Channel to send messages in, call with other users, organize groups, and
/// more.
///
/// The `Channel` type is one overarching type for all types of channels: there
/// is no distinction between audio channels, textual channels, guild channels,
/// groups, threads, and so on. The type of channel can be determined by
/// checking [`Channel::kind`], which can be used to determine what fields you
/// might expect to be present.
///
/// For Discord's documentation on channels, refer to [Discord Docs/Channel].
///
/// [Discord Docs/Channel]: https://discord.com/developers/docs/resources/channel
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Channel {
    /// ID of the guild the channel is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// ID of the channel.
    pub id: Id<ChannelMarker>,
    /// Type of the channel.
    ///
    /// This can be used to determine what fields *might* be available.
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// ID of the parent channel.
    ///
    /// For guild channels this is the ID of the parent category channel.
    ///
    /// For threads this is the ID of the channel the thread was created in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    /// Explicit permission overwrites for members and roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    /// Sorting position of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// ID of the voice region for the channel.
    ///
    /// Defaults to automatic for applicable channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,
    /// Number of users that may be in the channel.
    ///
    /// Zero refers to no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::{Channel, ChannelType};
    use crate::{
        channel::permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
        guild::Permissions,
        id::Id,
    };

    // The deserializer for GuildChannel should skip over fields names that
    // it couldn't deserialize.
    #[test]
    fn guild_channel_unknown_field_deserialization() {
        let input = serde_json::json!({
            "type": 0,
            "position": 0,
            "permission_overwrites": [],
            "parent_id": null,
            "id": "2",
            "guild_id": "1",
            "guild_hashes": {
                "version": 1,
                "roles": {
                    "hash": "aaaaaaaaaaa"
                },
                "metadata": {
                    "hash": "bbbbbbbbbbb"
                },
                "channels": {
                    "hash": "ccccccccccc"
                }
            },
            "unknown_field": "the deserializer should skip unknown field names",
        });

        let value = Channel {
            guild_id: Some(Id::new(1)),
            id: Id::new(2),
            kind: ChannelType::GuildText,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(0),
            rtc_region: None,
            user_limit: None,
        };

        assert_eq!(value, serde_json::from_value(input).unwrap());
    }

    #[test]
    fn guild_category_channel_deserialization() {
        let value = Channel {
            guild_id: Some(Id::new(2)),
            id: Id::new(1),
            kind: ChannelType::GuildCategory,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(3),
            rtc_region: None,
            user_limit: None,
        };
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": Some("2"),
                "name": "foo",
                "permission_overwrites": permission_overwrites,
                "position": 3,
                "type": 4,
            }))
            .unwrap()
        );
    }

    #[test]
    fn guild_announcement_channel_deserialization() {
        let value = Channel {
            guild_id: Some(Id::new(2)),
            id: Id::new(1),
            kind: ChannelType::GuildAnnouncement,
            parent_id: Some(Id::new(5)),
            permission_overwrites: Some(Vec::new()),
            position: Some(3),
            rtc_region: None,
            user_limit: None,
        };
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": "2",
                "parent_id": "5",
                "permission_overwrites": permission_overwrites,
                "position": 3,
                "type": ChannelType::GuildAnnouncement,
            }))
            .unwrap()
        );
    }

    #[test]
    fn guild_announcement_thread_deserialization() {
        let value = Channel {
            guild_id: Some(Id::new(1)),
            id: Id::new(6),
            kind: ChannelType::AnnouncementThread,
            parent_id: Some(Id::new(2)),
            permission_overwrites: None,
            position: None,
            rtc_region: None,
            user_limit: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::AnnouncementThread,
                "parent_id": "2",
            }))
            .unwrap()
        )
    }

    #[test]
    fn public_thread_deserialization() {
        let value = Channel {
            guild_id: Some(Id::new(1)),
            id: Id::new(6),
            kind: ChannelType::PublicThread,
            parent_id: Some(Id::new(2)),
            permission_overwrites: None,
            position: None,
            rtc_region: None,
            user_limit: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::PublicThread,
                "parent_id": "2",
            }))
            .unwrap()
        )
    }

    #[test]
    fn private_thread_deserialization() {
        let value = Channel {
            guild_id: Some(Id::new(1)),
            id: Id::new(6),
            kind: ChannelType::PrivateThread,
            parent_id: Some(Id::new(2)),
            permission_overwrites: Some(Vec::from([PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::empty(),
                id: Id::new(5),
                kind: PermissionOverwriteType::Member,
            }])),
            position: None,
            rtc_region: None,
            user_limit: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::PrivateThread,
                "parent_id": "2",
                "permission_overwrites": [
                    {
                        "allow": "0",
                        "deny": "0",
                        "type": 1,
                        "id": "5"
                    }
                ]
            }))
            .unwrap()
        )
    }
}
