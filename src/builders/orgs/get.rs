imports!();

new_builder!(Org, Orgs, Member, Members, OrgNamespaces, OrgRegistries);

exec!(
    Org -> crate::models::Organization,
    Orgs -> Vec<crate::models::Organization>,

    Member -> crate::models::OrgMember,
    Members -> Vec<crate::models::OrgMember>,

    OrgRegistries -> Vec<crate::models::Registry>,

    OrgNamespaces -> Vec<String>,
);

from!(
    @Orgs
        -> Org,
        -> OrgNamespaces,
    @Org
        -> Members,
        -> OrgRegistries,
    @Members
        -> Member,
);

impl_client!(
    /// Begins an organization query.
    -> orgs ["orgs"] -> Orgs,
);

impl_builder!(
    @Orgs
        /// Queries an organization by its id.
        => get [] -> Org = id,
        /// Queries the available namespaces for organizations.
        -> namespaces ["namespaces"] -> OrgNamespaces,
    @Org
        /// Queries all members in an organization.
        -> members ["members"] -> Members,
        /// Queries all registries in an organization.
        -> registries ["registries"] -> OrgRegistries,
    @Members
        /// Queries a specific member in an organization by their user id.
        => get [] -> Member = user_id,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    #[tokio::test]
    async fn test_orgs() {
        let c = client();

        let res = c
            .orgs()
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // we should have at least one
        assert_ne!(res.len(), 0);

        // they should all have non-empty ids
        let ok = res.iter().fold(false, |ok, org| ok || org.id != "");
        assert_eq!(ok, true);
    }

    #[tokio::test]
    async fn test_org() {
        let c = client();

        let res = c
            .orgs()
            .get(ORG_ID)
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // just make sure the id is correct
        assert_eq!(res.id, ORG_ID);
    }

    #[tokio::test]
    async fn test_org_namespaces() {
        let c = client();

        let res = c
            .orgs()
            .namespaces()
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // we should get at least 1
        assert_ne!(res.len(), 0);

        // they should all be a non-empty string
        let ok = res.iter().fold(false, |ok, n| ok || n != "");
        assert_eq!(ok, true);
    }

    mod members {
        use super::*;

        #[tokio::test]
        async fn test_org_members() {
            let c = client();

            let res = c
                .orgs()
                .get(ORG_ID)
                .members()
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // we should get at least 1
            assert_ne!(res.len(), 0);

            // they should all have non-empty ids
            let ok = res.iter().fold(false, |ok, mem| ok || mem.user.id != "");
            assert_eq!(ok, true);
        }

        #[tokio::test]
        async fn test_org_member() {
            let c = client();

            let res = c
                .orgs()
                .get(ORG_ID)
                .members()
                .get(MEMBER_ID)
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // id should be a non-empty string
            assert_ne!(res.user.id, "");
        }
    }

    mod registries {
        use super::*;

        #[tokio::test]
        async fn test_org_members() {
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
