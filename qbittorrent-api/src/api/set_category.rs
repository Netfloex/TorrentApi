use surf::Body;

use crate::{
    models::set_category_options::SetCategoryOptions, Error, ErrorKind, QbittorrentClient,
};

impl QbittorrentClient {
    pub async fn set_category(&self, hash: String, category: String) -> Result<(), Error> {
        let body = Body::from_form(&SetCategoryOptions::new(hash, category.to_owned())).unwrap();
        let resp = self
            .http
            .post("/api/v2/torrents/setCategory")
            .body(body)
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::CategoryDoesNotExist,
                format!("Category: \"{}\" does not exist", category),
            ))
        }
    }
}
