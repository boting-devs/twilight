use super::{
    Permissions, Role
};
use crate::{
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialGuild {
    pub id: Id<GuildMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    pub name: String,
    pub owner_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    pub roles: Vec<Role>,
}

#[cfg(test)]
mod tests {
    use crate::{
        test::image_hash,
    };

    use super::{
        PartialGuild,
        Permissions,
    };
    use crate::id::Id;
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn partial_guild() {
        let value = PartialGuild {
            id: Id::new(1),
            member_count: Some(12_000),
            name: "the name".to_owned(),
            owner_id: Id::new(5),
            permissions: Some(Permissions::SEND_MESSAGES),
            roles: Vec::new(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PartialGuild",
                    len: 35,
                },
                Token::Str("afk_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("afk_timeout"),
                Token::NewtypeStruct { name: "AfkTimeout" },
                Token::U16(900),
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("banner"),
                Token::Some,
                Token::Str(image_hash::BANNER_INPUT),
                Token::Str("default_message_notifications"),
                Token::U8(1),
                Token::Str("description"),
                Token::Some,
                Token::Str("a description"),
                Token::Str("discovery_splash"),
                Token::Some,
                Token::Str(image_hash::SPLASH_INPUT),
                Token::Str("emojis"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("explicit_content_filter"),
                Token::U8(1),
                Token::Str("features"),
                Token::Seq { len: Some(1) },
                Token::Str("ANIMATED_BANNER"),
                Token::SeqEnd,
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("max_members"),
                Token::Some,
                Token::U64(25_000),
                Token::Str("max_presences"),
                Token::Some,
                Token::U64(10_000),
                Token::Str("member_count"),
                Token::Some,
                Token::U64(12_000),
                Token::Str("mfa_level"),
                Token::U8(1),
                Token::Str("name"),
                Token::Str("the name"),
                Token::Str("nsfw_level"),
                Token::U8(0),
                Token::Str("owner_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("owner"),
                Token::Some,
                Token::Bool(false),
                Token::Str("permissions"),
                Token::Some,
                Token::Str("2048"),
                Token::Str("preferred_locale"),
                Token::Str("en-us"),
                Token::Str("premium_progress_bar_enabled"),
                Token::Bool(true),
                Token::Str("premium_subscription_count"),
                Token::Some,
                Token::U64(3),
                Token::Str("premium_tier"),
                Token::U8(1),
                Token::Str("public_updates_channel_id"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("rules_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("splash"),
                Token::Some,
                Token::Str(image_hash::SPLASH_INPUT),
                Token::Str("system_channel_flags"),
                Token::U64(2),
                Token::Str("system_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("7"),
                Token::Str("verification_level"),
                Token::U8(2),
                Token::Str("vanity_url_code"),
                Token::Some,
                Token::Str("twilight"),
                Token::Str("widget_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("8"),
                Token::Str("widget_enabled"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
