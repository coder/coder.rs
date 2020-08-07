imports!();

new_builder!(Me, User, Users);

use crate::client::GetQueryBuilder;

exec!(
    Me    -> crate::models::User,
    User  -> crate::models::User,
    Users -> Vec<crate::models::User>,
);

from!(
    @GetQuery
        -> User,
        -> Users,
);

impl_builder!(
    @GetQuery
        /// Queries the current user.
        -> me    ["users/me"] -> User,
        /// Queries all users.
        -> users ["users"]    -> Users,
        /// Queries a user by their id.
        => user  ["users"]    -> User = id,
);
