use crate::{
    guild::Permissions,
    id::{marker::RoleMarker, Id},
};
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Role {
    pub id: Id<RoleMarker>,
    pub permissions: Permissions,
    pub position: i64,
}

impl Ord for Role {
    /// Compare two roles to each other using their position and ID.
    ///
    /// Roles are primarily ordered by their position in descending order. For example,
    /// a role with a position of 17 is considered a higher role than one with a
    /// position of 12.
    ///
    /// Discord does not guarantee that role positions are positive, unique, or contiguous. When
    /// two or more roles have the same position then the order is based on the roles' IDs in
    /// ascending order. For example, given two roles with positions of 10 then a role
    /// with an ID of 1 would be considered a higher role than one with an ID of 20.
    ///
    /// ### Examples
    ///
    /// Compare the position of two roles:
    ///
    /// ```
    /// # use twilight_model::{guild::{Permissions, Role}, id::Id};
    /// # use std::cmp::Ordering;
    /// let role_a = Role {
    ///     id: Id::new(123),
    ///     position: 12,
    /// #   color: 0,
    /// #   hoist: true,
    /// #   icon: None,
    /// #   managed: false,
    /// #   mentionable: true,
    /// #   name: "test".to_owned(),
    /// #   permissions: Permissions::ADMINISTRATOR,
    /// #   tags: None,
    /// #   unicode_emoji: None,
    ///     // ...
    /// };
    /// let role_b = Role {
    ///     id: Id::new(456),
    ///     position: 13,
    /// #   color: 0,
    /// #   hoist: true,
    /// #   icon: None,
    /// #   managed: false,
    /// #   mentionable: true,
    /// #   name: "test".to_owned(),
    /// #   permissions: Permissions::ADMINISTRATOR,
    /// #   tags: None,
    /// #   unicode_emoji: None,
    ///     // ...
    /// };
    /// assert_eq!(Ordering::Less, role_a.cmp(&role_b));
    /// assert_eq!(Ordering::Greater, role_b.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_a.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_b.cmp(&role_b));
    /// ```
    ///
    /// Compare the position of two roles with the same position:
    ///
    /// ```
    /// # use twilight_model::{guild::{Permissions, Role}, id::Id};
    /// # use std::cmp::Ordering;
    /// let role_a = Role {
    ///     id: Id::new(123),
    ///     position: 12,
    /// #   color: 0,
    /// #   hoist: true,
    /// #   icon: None,
    /// #   managed: false,
    /// #   mentionable: true,
    /// #   name: "test".to_owned(),
    /// #   permissions: Permissions::ADMINISTRATOR,
    /// #   tags: None,
    /// #   unicode_emoji: None,
    /// };
    /// let role_b = Role {
    ///     id: Id::new(456),
    ///     position: 12,
    /// #   color: 0,
    /// #   hoist: true,
    /// #   icon: None,
    /// #   managed: false,
    /// #   mentionable: true,
    /// #   name: "test".to_owned(),
    /// #   permissions: Permissions::ADMINISTRATOR,
    /// #   tags: None,
    /// #   unicode_emoji: None,
    /// };
    /// assert_eq!(Ordering::Less, role_a.cmp(&role_b));
    /// assert_eq!(Ordering::Greater, role_b.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_a.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_b.cmp(&role_b));
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.position
            .cmp(&other.position)
            .then(self.id.get().cmp(&other.id.get()))
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::{Permissions, Role};
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(Role: id, permissions, position);

    assert_impl_all!(
        Role: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize
    );

    #[test]
    fn role() {
        let role = Role {
            id: Id::new(123),
            permissions: Permissions::ADMINISTRATOR,
            position: 12,
        };

        serde_test::assert_tokens(
            &role,
            &[
                Token::Struct {
                    name: "Role",
                    len: 8,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("permissions"),
                Token::Str("8"),
                Token::Str("position"),
                Token::I64(12),
                Token::StructEnd,
            ],
        );
    }
}
