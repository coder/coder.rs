imports!();

new_builder!(Org, Orgs, Member, Members, Namespaces);

use crate::client::GetQueryBuilder;

exec!(
    Org -> crate::models::Organization,
    Orgs -> Vec<crate::models::Organization>,

    Member -> crate::models::OrgMember,
    Members -> Vec<crate::models::OrgMember>,

    Namespaces -> Vec<String>,
);

from!(
    @GetQuery
        -> Org,
        -> Orgs,

    @Org
        -> Member,
        -> Members,

    @Orgs
        -> Namespaces,
);

impl_builder!(
    @GetQuery
        /// Queries all orgs the user belongs to, or all if the user is a site admin.
        -> orgs ["orgs"] -> Orgs,
        /// Queries an org by its id.
        => org ["orgs"] -> Org = id,

    @Org
        /// Queries all members in an organization.
        -> members ["members"] -> Members,
        /// Queries a organization member by their id.
        => member ["members"] -> Member = id,

    @Orgs
        /// Queries the available namespaces in an organization.
        -> namespaces ["namespaces"] -> Namespaces,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    #[tokio::test]
    async fn test_orgs() {
        let c = client();

        let res = c
            .get()
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
            .get()
            .org(ORG_ID)
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
            .get()
            .orgs()
            .namespaces()
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // we should get at least 1 namespace
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
                .get()
                .org(ORG_ID)
                .members()
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // we should get at least 1 member
            assert_ne!(res.len(), 0);

            // they should all have non-empty ids
            let ok = res.iter().fold(false, |ok, mem| ok || mem.user.id != "");
            assert_eq!(ok, true);
        }

        #[tokio::test]
        async fn test_org_member() {
            let c = client();

            let res = c
                .get()
                .org(ORG_ID)
                .member(MEMBER_ID)
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // id should be a non-empty string
            assert_ne!(res.user.id, "");
        }
    }
}
