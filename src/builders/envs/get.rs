imports!();

new_builder!(Env, Envs);

use crate::builders::orgs::get::MemberBuilder;
use crate::builders::orgs::get::OrgBuilder;
use crate::client::GetQueryBuilder;

exec!(
    Env -> crate::models::Environment,
    Envs -> Vec<crate::models::Environment>,
);

from!(
    @GetQuery
        -> Env,
    @Org
        -> Envs,
    @Member
        -> Envs,
);

impl_builder!(
    @GetQuery
        /// Queries an environment by its id. Must be a site admin or a manager of the organization
        /// the environment belongs to.
        => env ["environments"] -> Env = id,

    @Org
        /// Queries all environments belonging to the organization. Must be an organization
        /// manager.
        -> envs ["environments"] -> Envs,

    @Member
        /// Queries all environments belonging to the an organization member.
        -> envs ["environments"] -> Envs,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    #[tokio::test]
    async fn test_env() {
        let c = client();

        let res = c
            .get()
            .env(ENV_ID)
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // id should at least not be empty
        assert_ne!(res.id, "");
    }

    mod org {
        use super::*;

        #[tokio::test]
        async fn test_org_envs() {
            let c = client();

            let res = c
                .get()
                .org(ORG_ID)
                .envs()
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // we should get at least 1
            assert_ne!(res.len(), 0);

            // they should all have non-empty ids
            let ok = res.iter().fold(false, |ok, env| ok || env.id != "");
            assert_eq!(ok, true);
        }
    }

    mod member {
        use super::*;

        #[tokio::test]
        async fn test_org_member_envs() {
            let c = client();

            let res = c
                .get()
                .org(ORG_ID)
                .member(MEMBER_ID)
                .envs()
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // we should get at least 1 member
            assert_ne!(res.len(), 0);

            // they should all have non-empty ids
            let ok = res.iter().fold(false, |ok, env| ok || env.id != "");
            assert_eq!(ok, true);
        }
    }
}
