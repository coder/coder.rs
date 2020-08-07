imports!();

new_builder!(Service, Services);

use crate::builders::orgs::get::OrgBuilder;

exec!(
    Service -> crate::models::Service,
    Services -> Vec<crate::models::Service>,
);

from!(
    @Org
        -> Service,
        -> Services,
);

impl_macro!(
    @Org
        /// Queries all services in an organization.
        -> services ["services"] -> Services,
        /// Queries a service in an organization by its id.
        => service ["services"] -> Service = id,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    mod org {
        use super::*;

        #[tokio::test]
        async fn test_org_services() {
            let c = client();

            let res = c
                .get()
                .org(ORG_ID)
                .services()
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // we should get at least 1 member
            assert_ne!(res.len(), 0);

            // they should all have non-empty ids
            let ok = res.iter().fold(false, |ok, svc| ok || svc.id != "");
            assert_eq!(ok, true);
        }

        #[tokio::test]
        async fn test_org_service() {
            let c = client();

            let res = c
                .get()
                .org(ORG_ID)
                .service(SERVICE_ID)
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // id should not be empty
            assert_ne!(res.id, "");
        }
    }
}
