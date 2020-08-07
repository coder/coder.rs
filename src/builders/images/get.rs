imports!();

new_builder!(Image, OrgImages, GlobalImages);

use crate::builders::orgs::get::OrgBuilder;

exec!(
    Image  -> crate::models::Image,
    OrgImages -> Vec<crate::models::Image>,
);

from!(
    @Org
        -> OrgImages,
    @GlobalImages
        -> Image,
);

impl_client!(
    /// Begins a global image query.
    -> images ["images"] -> GlobalImages,
);

impl_builder!(
    @Org
        /// Queries all images in an organization.
        -> images ["images"] -> OrgImages,
    @GlobalImages
        /// Queries an image by its id.
        => get [] -> Image = id,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    #[tokio::test]
    async fn test_image() {
        let c = client();

        let res = c
            .images()
            .get(IMAGE_ID)
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
                .orgs()
                .get(ORG_ID)
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
