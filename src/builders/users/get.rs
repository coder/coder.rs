imports!();

new_builder!(User, Users);

exec!(
    User  -> crate::models::User,
    Users -> Vec<crate::models::User>,
);

from!(
    @Users
        -> User,
);

impl_client!(
    /// Begins a user query.
    -> users ["users"] -> Users,
);

impl_builder!(
    @Users
        /// Queries the current user.
        -> me ["me"] -> User,
        /// Queries a user by their id.
        => get [] -> User = id,
);
