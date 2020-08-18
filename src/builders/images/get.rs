imports!();

new_builder!(
    /// /api/images/:id
    GlobalImage,
    /// /api/images
    GlobalImages,
    /// /api/orgs/:id/images
    OrgImages,
);

use crate::builders::orgs::get::OrgBuilder;

exec!(
    GlobalImage  -> crate::models::Image,
    OrgImages -> Vec<crate::models::Image>,
);

from!(
    @Org
        -> OrgImages,
    @GlobalImages
        -> GlobalImage,
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
        /// Queries an image in by its id.
        => get [] -> GlobalImage = id,
        ?> with_envs ["envs"] -> v: bool,
    @GlobalImage
        ?> with_envs ["envs"] -> v: bool,
    @OrgImages
        ?> with_envs ["envs"] -> v: bool,
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
        assert!(!res.id.is_empty(), "id should be a non-empty string");
    }

    #[tokio::test]
    async fn test_image_with_environments() {
        let c = client();

        let res = c
            .images()
            .get(IMAGE_ID)
            .with_envs(true)
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // id should at least not be empty
        assert!(!res.id.is_empty(), "id should be a non-empty string");
        assert!(res.environments.is_some(), "envs should be returned");

        let res = c
            .images()
            .get(IMAGE_ID)
            .with_envs(false)
            .execute()
            .await
            .expect("send request")
            .response
            .expect("api error returned");

        // id should at least not be empty
        assert!(!res.id.is_empty(), "id should be a non-empty string");
        assert!(res.environments.is_none(), "envs should not be returned");
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
            let ok = res.iter().fold(false, |ok, img| ok || img.id != "".into());
            assert_eq!(ok, true);
        }
    }
}
