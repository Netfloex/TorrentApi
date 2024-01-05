use surf::Body;

use crate::{error::ErrorKind, AddCategoryOptions, Error, QbittorrentClient};

impl QbittorrentClient {
    pub async fn ensure_category(&self, name: &str, save_path: &str) -> Result<(), Error> {
        let categories = self.categories().await?;

        let category = categories.iter().find(|c| c.name() == name);
        match category {
            None => {
                self.add_category(name, save_path).await?;
            }
            Some(c) => {
                if c.save_path() != save_path {
                    self.edit_category(name, save_path).await?;
                }
            }
        }
        Ok(())
    }
}
