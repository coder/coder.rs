imports!();

new_builder!(Env, OrgEnvs, GlobalEnvs, MemberEnvs);

use crate::builders::orgs::get::MemberBuilder;
use crate::builders::orgs::get::OrgBuilder;

exec!(
    Env -> crate::models::Environment,
    OrgEnvs -> Vec<crate::models::Environment>,
    MemberEnvs -> Vec<crate::models::Environment>,
);

from!(
    @Org
        -> OrgEnvs,
    @Member
        -> MemberEnvs,
    @GlobalEnvs
        -> Env,
);

impl_client!(
    /// Begins an environments query.
    -> envs ["environments"] -> GlobalEnvs,
);

impl_builder!(
    @Org
        /// Queries all environments belonging to the organization. Must be an organization
        /// manager.
        -> envs ["environments"] -> OrgEnvs,

    @Member
        /// Queries all environments belonging to the an organization member.
        -> envs ["environments"] -> MemberEnvs,

    @GlobalEnvs
        /// Queries an environment by its id. Must be a site admin or a manager of the organization
        /// the environment belongs to.
        => get [] -> Env = id,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    #[tokio::test]
    async fn test_env() {
        let c = client();

        let res = c
            .envs()
            .get(ENV_ID)
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
                .orgs()
                .get(ORG_ID)
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
                .orgs()
                .get(ORG_ID)
                .members()
                .get(MEMBER_ID)
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
