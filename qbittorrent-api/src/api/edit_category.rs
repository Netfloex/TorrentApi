use surf::Body;

use crate::{error::ErrorKind, AddCategoryOptions, Error, QbittorrentClient};

impl QbittorrentClient {
    pub async fn edit_category(&self, name: &str, save_path: &str) -> Result<(), Error> {
        if name.is_empty() {
            return Err(Error::new(
                ErrorKind::BadParameters("name".to_string()),
                "Name is empty",
            ));
        }

        let form = AddCategoryOptions::new(name.to_string(), save_path.to_string());
        let body = Body::from_form(&form).unwrap();

        let mut resp = self
            .http
            .post("/api/v2/torrents/editCategory")
            .body(body)
            .await?;

        if !resp.status().is_success() {
            return Err(Error::new(
                ErrorKind::RequestError,
                resp.body_string().await?,
            ));
        }

        Ok(())
    }
}
