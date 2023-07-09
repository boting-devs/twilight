use serde::Serialize;
use twilight_model::{
    application::interaction::application_command::InteractionMember,
    guild::{Member, PartialMember},
    id::{
        marker::{RoleMarker, UserMarker},
        Id,
    },
    util::Timestamp,
};

/// Computed fields required to complete a full cached member via
// /// [`CachedMember::from_interaction_member`] that are not otherwise present.
// pub(crate) struct ComputedInteractionMemberFields {
//     pub avatar: Option<ImageHash>,
//     pub deaf: Option<bool>,
//     pub mute: Option<bool>,
// }

/// Represents a cached [`Member`].
///
/// [`Member`]: twilight_model::guild::Member
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMember {
    pub(crate) communication_disabled_until: Option<Timestamp>,
    pub(crate) roles: Vec<Id<RoleMarker>>,
    pub(crate) user_id: Id<UserMarker>,
}

impl CachedMember {
    /// When the user can resume communication in a guild again.
    ///
    /// Checking if this value is [`Some`] is not enough to know if a used is currently
    /// timed out as Discord doesn't send any events when the timeout expires, and
    /// therefore the cache is not updated accordingly. You should ensure that the
    /// provided [`Timestamp`] is not in the past. See [discord-api-docs#4269].
    ///
    /// [discord-api-docs#4269]: https://github.com/discord/discord-api-docs/issues/4269
    pub const fn communication_disabled_until(&self) -> Option<Timestamp> {
        self.communication_disabled_until
    }

    /// List of role IDs this member has.
    pub fn roles(&self) -> &[Id<RoleMarker>] {
        &self.roles
    }

    /// ID of the user relating to the member.
    pub const fn user_id(&self) -> Id<UserMarker> {
        self.user_id
    }

    /// Construct a cached member from its [`twilight_model`] form.
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn from_model(member: Member) -> Self {
        let Member {
            communication_disabled_until,
            roles,
            user,
        } = member;

        Self {
            communication_disabled_until,
            roles,
            user_id: user.id,
        }
    }

    // clippy: the member field's destructor needs to drop
    // clippy: the contents of `fields` is consumed
    #[allow(clippy::missing_const_for_fn, clippy::needless_pass_by_value)]
    pub(crate) fn from_interaction_member(
        user_id: Id<UserMarker>,
        member: InteractionMember,
    ) -> Self {
        let InteractionMember {
            communication_disabled_until,
            roles,
        } = member;

        Self {
            communication_disabled_until,
            roles,
            user_id,
        }
    }

    pub(crate) fn from_partial_member(user_id: Id<UserMarker>, member: PartialMember) -> Self {
        let PartialMember {
            communication_disabled_until,
            roles,
            user,
        } = member;

        Self {
            communication_disabled_until,
            roles,
            user_id: user.map_or(user_id, |user| user.id),
        }
    }
}

impl PartialEq<Member> for CachedMember {
    fn eq(&self, other: &Member) -> bool {
        self.communication_disabled_until == other.communication_disabled_until
            && self.roles == other.roles
            && self.user_id == other.user.id
    }
}

impl PartialEq<PartialMember> for CachedMember {
    fn eq(&self, other: &PartialMember) -> bool {
        self.communication_disabled_until == other.communication_disabled_until
            && self.roles == other.roles
    }
}

impl PartialEq<InteractionMember> for CachedMember {
    fn eq(&self, other: &InteractionMember) -> bool {
        self.roles == other.roles
    }
}

#[cfg(test)]
mod tests {
    use super::CachedMember;
    use static_assertions::assert_fields;
    use twilight_model::{
        guild::{Member, MemberFlags, PartialMember},
        id::Id,
        user::User,
        util::Timestamp,
    };

    assert_fields!(
        CachedMember: deaf,
        joined_at,
        mute,
        nick,
        pending,
        premium_since,
        roles,
        user_id
    );

    fn cached_member() -> CachedMember {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;
        CachedMember {
            avatar: None,
            communication_disabled_until: None,
            deaf: Some(false),
            flags,
            joined_at,
            mute: Some(true),
            nick: Some("member nick".to_owned()),
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user_id: user().id,
        }
    }

    fn user() -> User {
        User {
            accent_color: None,
            avatar: None,
            banner: None,
            bot: false,
            discriminator: 1,
            email: None,
            flags: None,
            id: Id::new(1),
            locale: None,
            mfa_enabled: None,
            name: "bar".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        }
    }

    #[test]
    fn eq_member() {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let member = Member {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            flags,
            joined_at,
            mute: true,
            nick: Some("member nick".to_owned()),
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user: user(),
        };

        assert_eq!(cached_member(), member);
    }

    #[test]
    fn eq_partial_member() {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let member = PartialMember {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            flags,
            joined_at,
            mute: true,
            nick: Some("member nick".to_owned()),
            permissions: None,
            premium_since: None,
            roles: Vec::new(),
            user: None,
        };

        assert_eq!(cached_member(), member);
    }
}
