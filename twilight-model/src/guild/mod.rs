//! Group of [`Channel`]s and [`User`]s with additional moderation options.
//!
//! [`Guild`]s allow, for example, assigning [`Role`]s to [`Member`]s to limit
//! their [`Permissions`] globally, or per [`Channel`].
//!
//! [`User`]: super::user::User

pub mod audit_log;
pub mod auto_moderation;
pub mod invite;
pub mod scheduled_event;
pub mod template;
pub mod widget;

mod afk_timeout;
mod ban;
mod default_message_notification_level;
mod emoji;
mod explicit_content_filter;
mod feature;
mod info;
mod integration;
mod integration_account;
mod integration_application;
mod integration_expire_behavior;
mod integration_type;
mod member;
mod member_flags;
mod mfa_level;
mod nsfw_level;
mod partial_guild;
mod partial_member;
mod permissions;
mod premium_tier;
mod preview;
mod prune;
mod role;
mod role_tags;
mod system_channel_flags;
mod unavailable_guild;
mod vanity_url;
mod verification_level;

pub use self::nsfw_level::NSFWLevel;
pub use self::permissions::Permissions;
pub use self::{
    afk_timeout::AfkTimeout, ban::Ban,
    default_message_notification_level::DefaultMessageNotificationLevel, emoji::Emoji,
    explicit_content_filter::ExplicitContentFilter, feature::GuildFeature, info::GuildInfo,
    integration::GuildIntegration, integration_account::IntegrationAccount,
    integration_application::IntegrationApplication,
    integration_expire_behavior::IntegrationExpireBehavior, integration_type::GuildIntegrationType,
    member::Member, member_flags::MemberFlags, mfa_level::MfaLevel, partial_guild::PartialGuild,
    partial_member::PartialMember, premium_tier::PremiumTier, preview::GuildPreview,
    prune::GuildPrune, role::Role, role_tags::RoleTags, system_channel_flags::SystemChannelFlags,
    unavailable_guild::UnavailableGuild, vanity_url::VanityUrl,
    verification_level::VerificationLevel, widget::GuildWidget,
};

use crate::{
    channel::Channel,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    voice::VoiceState,
};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Guild {
    #[serde(default)]
    pub channels: Vec<Channel>,
    pub id: Id<GuildMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(default)]
    pub members: Vec<Member>,
    pub name: String,
    pub owner_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    pub roles: Vec<Role>,
    #[serde(default)]
    pub threads: Vec<Channel>,
    #[serde(default)]
    pub unavailable: bool,
    #[serde(default)]
    pub voice_states: Vec<VoiceState>,
}

impl<'de> Deserialize<'de> for Guild {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Channels,
            Id,
            MemberCount,
            Members,
            Name,
            OwnerId,
            Permissions,
            Roles,
            Threads,
            Unavailable,
            VoiceStates,
        }

        struct GuildVisitor;

        impl<'de> Visitor<'de> for GuildVisitor {
            type Value = Guild;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("struct Guild")
            }

            #[allow(clippy::too_many_lines)]
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let mut channels = None::<Vec<Channel>>;
                let mut id = None;
                let mut member_count = None::<Option<_>>;
                let mut members = None;
                let mut name = None;
                let mut owner_id = None;
                let mut permissions = None::<Option<_>>;
                let mut roles = None;
                let mut threads = None::<Vec<Channel>>;
                let mut unavailable = None;
                let mut voice_states = None::<Vec<VoiceState>>;

                let span = tracing::trace_span!("deserializing guild");
                let _span_enter = span.enter();

                loop {
                    let span_child = tracing::trace_span!("iterating over element");
                    let _span_child_enter = span_child.enter();

                    let key = match map.next_key() {
                        Ok(Some(key)) => {
                            tracing::trace!(?key, "found key");

                            key
                        }
                        Ok(None) => break,
                        Err(why) => {
                            // Encountered when we run into an unknown key.
                            map.next_value::<IgnoredAny>()?;

                            tracing::trace!("ran into an unknown key: {why:?}");

                            continue;
                        }
                    };

                    match key {
                        Field::Channels => {
                            if channels.is_some() {
                                return Err(DeError::duplicate_field("channels"));
                            }

                            channels = Some(map.next_value()?);
                        }
                        Field::Id => {
                            if id.is_some() {
                                return Err(DeError::duplicate_field("id"));
                            }

                            id = Some(map.next_value()?);
                        }
                        Field::MemberCount => {
                            if member_count.is_some() {
                                return Err(DeError::duplicate_field("member_count"));
                            }

                            member_count = Some(map.next_value()?);
                        }
                        Field::Members => {
                            if members.is_some() {
                                return Err(DeError::duplicate_field("members"));
                            }

                            members = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(DeError::duplicate_field("name"));
                            }

                            name = Some(map.next_value()?);
                        }
                        Field::OwnerId => {
                            if owner_id.is_some() {
                                return Err(DeError::duplicate_field("owner_id"));
                            }

                            owner_id = Some(map.next_value()?);
                        }
                        Field::Permissions => {
                            if permissions.is_some() {
                                return Err(DeError::duplicate_field("permissions"));
                            }

                            permissions = Some(map.next_value()?);
                        }
                        Field::Roles => {
                            if roles.is_some() {
                                return Err(DeError::duplicate_field("roles"));
                            }

                            roles = Some(map.next_value()?);
                        }
                        Field::Threads => {
                            if threads.is_some() {
                                return Err(DeError::duplicate_field("threads"));
                            }

                            threads = Some(map.next_value()?);
                        }
                        Field::Unavailable => {
                            if unavailable.is_some() {
                                return Err(DeError::duplicate_field("unavailable"));
                            }

                            unavailable = Some(map.next_value()?);
                        }
                        Field::VoiceStates => {
                            if voice_states.is_some() {
                                return Err(DeError::duplicate_field("voice_states"));
                            }

                            voice_states = Some(map.next_value()?);
                        }
                    }
                }

                let id = id.ok_or_else(|| DeError::missing_field("id"))?;
                let name = name.ok_or_else(|| DeError::missing_field("name"))?;
                let owner_id = owner_id.ok_or_else(|| DeError::missing_field("owner_id"))?;
                let roles = roles.ok_or_else(|| DeError::missing_field("roles"))?;

                let mut channels = channels.unwrap_or_default();
                let member_count = member_count.unwrap_or_default();
                let members = members.unwrap_or_default();
                let permissions = permissions.unwrap_or_default();
                let mut threads = threads.unwrap_or_default();
                let unavailable = unavailable.unwrap_or_default();
                let mut voice_states = voice_states.unwrap_or_default();

                tracing::trace!(
                    ?channels,
                    %id,
                    ?member_count,
                    ?members,
                    %name,
                    %owner_id,
                    ?permissions,
                );

                // Split in two due to generic impl only going up to 32.
                tracing::trace!(?roles, ?threads, ?unavailable, ?voice_states,);

                for channel in &mut channels {
                    channel.guild_id = Some(id);
                }

                for thread in &mut threads {
                    thread.guild_id = Some(id);
                }

                for voice_state in &mut voice_states {
                    voice_state.guild_id.replace(id);
                }

                Ok(Guild {
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
                })
            }
        }

        const FIELDS: &[&str] = &[
            "channels",
            "id",
            "member_count",
            "members",
            "name",
            "owner_id",
            "permissions",
            "roles",
            "threads",
            "unavailable",
            "voice_states",
        ];

        deserializer.deserialize_struct("Guild", FIELDS, GuildVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{Guild, Permissions};
    use crate::id::Id;
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn guild() {
        let value = Guild {
            channels: Vec::new(),
            id: Id::new(1),
            member_count: Some(12_000),
            members: Vec::new(),
            name: "the name".to_owned(),
            owner_id: Id::new(5),
            permissions: Some(Permissions::SEND_MESSAGES),
            roles: Vec::new(),
            threads: Vec::new(),
            unavailable: false,
            voice_states: Vec::new(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Guild",
                    len: 47,
                },
                Token::Str("channels"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("member_count"),
                Token::Some,
                Token::U64(12_000),
                Token::Str("members"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("name"),
                Token::Str("the name"),
                Token::Str("owner_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("permissions"),
                Token::Some,
                Token::Str("2048"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("threads"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("unavailable"),
                Token::Bool(false),
                Token::Str("voice_states"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
