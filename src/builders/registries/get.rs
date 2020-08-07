imports!();

new_builder!(
    /// /api/registries/:id
    GlobalRegistry,
    /// /api/registries
    GlobalRegistries,
    /// /api/orgs/:id/registries
    OrgRegistries
);

use crate::builders::orgs::get::OrgBuilder;

exec!(
    GlobalRegistry -> crate::models::Registry,
    GlobalRegistries -> Vec<crate::models::Registry>,

    OrgRegistries -> Vec<crate::models::Registry>,
);

from!(
    @Org
        -> OrgRegistries,
    @GlobalRegistries
        -> GlobalRegistry,
);

impl_client!(
    /// Begins a global registry query.
    -> registries ["registries"] -> GlobalRegistries,
);

impl_builder!(
    @Org
        /// Queries all registries in an organization.
        -> registries ["registries"] -> OrgRegistries,
    @GlobalRegistries
        /// Queries a specific registry in an organization by its id.
        => get [] -> GlobalRegistry = id,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    #[tokio::test]
    async fn test_registry() {
        let c = client();

        let res = c
            .registries()
            .get(REG_ID)
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // id shouldn't be empty
        assert_ne!(res.id, "");
    }

    mod org {
        use super::*;

        #[tokio::test]
        async fn test_org_registries() {
            let c = client();

            let res = c
                .orgs()
                .get(ORG_ID)
                .registries()
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // we should get at least 1
            assert_ne!(res.len(), 0);

            // they should all have non-empty ids
            let ok = res.iter().fold(false, |ok, reg| ok || reg.id != "");
            assert_eq!(ok, true);
        }
    }
}
