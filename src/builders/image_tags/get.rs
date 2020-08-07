imports!();

new_builder!(
    /// /api/images/:id/tags/:tag
    ImageTag,
    /// /api/images/:id/tags
    ImageTags,
);

use crate::builders::images::get::GlobalImageBuilder;

exec!(
    ImageTag  -> crate::models::ImageTag,
    ImageTags -> Vec<crate::models::ImageTag>,
);

from!(
    @GlobalImage
        -> ImageTags,
    @ImageTags
        -> ImageTag,
);

impl_builder!(
    @GlobalImage
        /// Queries all image tags belonging to an image.
        -> tags ["tags"] -> ImageTags,
    @ImageTags
        /// Queries an image tag for an image by its tag.
        => get [] -> ImageTag = tag,
);

#[cfg(test)]
mod test {
    use crate::client::test::{client, ids::*};
    use crate::client::Executor;

    mod image {
        use super::*;

        #[tokio::test]
        async fn test_image_tag() {
            let c = client();

            let res = c
                .images()
                .get(IMAGE_ID)
                .tags()
                .get(IMAGE_TAG_ID)
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // id should at least not be empty
            assert_ne!(res.tag, "");
        }

        #[tokio::test]
        async fn test_image_tags() {
            let c = client();

            let res = c
                .images()
                .get(IMAGE_ID)
                .tags()
                .execute()
                .await
                .expect("send request")
                .response
                .expect("api error returned");

            // we should get at least 1
            assert_ne!(res.len(), 0);

            // they should all have non-empty ids
            let ok = res.iter().fold(false, |ok, img| ok || img.tag != "");
            assert_eq!(ok, true);
        }
    }
}
