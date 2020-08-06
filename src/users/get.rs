imports!();

new_builder!(
    /// Queries the current user.
    Me,
    /// Queries a user by their id.
    User,
    /// Queries all users.
    Users,
);

use crate::client::GetQueryBuilder;

exec!(
    Me    -> crate::models::User
    User  -> crate::models::User
    Users -> Vec<crate::models::User>
);

from!(
    @GetQuery
        => User
        => Users
);

impl_macro!(
    @GetQuery
        |-> me    ["users/me"] -> User
        |-> users ["users"]    -> Users
        |=> user  ["users"]    -> User = id
);
