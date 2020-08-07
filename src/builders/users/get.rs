imports!();

new_builder!(
    /// /api/users/:id
    User,
    /// /api/users
    Users
);

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

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    #[tokio::test]
    async fn test_user() {
        let c = client();

        let res = c
            .users()
            .get(USER_ID)
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // id shouldn't be empty
        assert_ne!(res.id, "");
    }

    #[tokio::test]
    async fn test_users() {
        let c = client();

        let res = c
            .users()
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // we should get at least 1
        assert_ne!(res.len(), 0);

        // they should all have non-empty ids
        let ok = res.iter().fold(true, |ok, usr| ok && usr.id != "");
        assert_eq!(ok, true);
    }
}
