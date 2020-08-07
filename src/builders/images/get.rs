imports!();

new_builder!(Image, Images);

use crate::builders::orgs::get::OrgBuilder;
use crate::client::GetQueryBuilder;

exec!(
    Image  -> crate::models::Image,
    Images -> Vec<crate::models::Image>,
);

from!(
    @GetQuery
        -> Image,
    @Org
        -> Images,
);

impl_macro!(
    @GetQuery
        /// Queries an image by its id.
        => image ["images"] -> Image = id,
    @Org
        /// Queries all images in an organization.
        -> images ["images"] -> Images,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    #[tokio::test]
    async fn test_image() {
        let c = client();

        let res = c
            .get()
            .image(IMAGE_ID)
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
        async fn test_org_images() {
            let c = client();

            let res = c
                .get()
                .org(ORG_ID)
                .images()
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // we should get at least 1
            assert_ne!(res.len(), 0);

            // they should all have non-empty ids
            let ok = res.iter().fold(false, |ok, img| ok || img.id != "");
            assert_eq!(ok, true);
        }
    }
}
