use crate::{
    id::{marker::RoleMarker, Id},
    user::User,
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialMember {
    pub communication_disabled_until: Option<Timestamp>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::PartialMember;
    use crate::{
        guild::MemberFlags,
        id::Id,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn partial_member() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = PartialMember {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            flags,
            joined_at,
            mute: true,
            nick: Some("a nickname".to_owned()),
            permissions: None,
            premium_since: None,
            roles: vec![Id::new(1)],
            user: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PartialMember",
                    len: 8,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Str("2015-04-26T06:26:56.936000+00:00"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("a nickname"),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
