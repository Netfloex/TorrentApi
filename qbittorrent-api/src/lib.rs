mod auth_middleware;
mod error;
use auth_middleware::AuthMiddleware;
use derive_getters::Getters;
use error::Error;
use error::ErrorKind;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Debug;
use surf::Body;
use surf::Client;
use surf::{Config, Url};

pub struct QbittorrentClient {
    http: Client,
}

#[derive(Serialize)]
struct AddTorrentOptions {
    urls: String,
    category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    name: String,
    #[serde(rename = "savePath")]
    save_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Categories {
    #[serde(flatten)]
    categories: HashMap<String, Category>,
}

#[derive(Serialize)]
struct AddCategoryOptions {
    category: String,
    #[serde(rename = "savePath")]
    save_path: String,
}
#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct Torrent {
    added_on: i64,
    amount_left: i64,
    category: String,
    hash: String,
    name: String,
}

impl QbittorrentClient {
    pub fn new<S: Into<String>, P: Into<String>, U: TryInto<Url>>(
        username: S,
        password: P,
        url: U,
    ) -> Self
    where
        U::Error: Debug,
    {
        let url: Url = url.try_into().expect("Invalid url");

        let config = Config::new().set_base_url(url.clone());
        let client: Client = config.try_into().unwrap();

        Self {
            http: client.with(AuthMiddleware::new(username.into(), password.into(), url)),
        }
    }

    pub async fn version(&self) -> Result<String, Error> {
        let version = self.http.get("/api/v2/app/version").recv_string().await?;

        Ok(version)
    }

    pub async fn add_torrents(
        &self,
        urls: &Vec<&str>,
        category: Option<String>,
    ) -> Result<(), Error> {
        let form = AddTorrentOptions {
            urls: urls.join("\n"),
            category,
        };
        let body = Body::from_form(&form).unwrap();
        let resp = self
            .http
            .post("/api/v2/torrents/add")
            .body(body)
            .recv_string()
            .await?;

        if resp == "Ok." {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::TorrentAddError,
                "Could not add torrent",
            ))
        }
    }

    pub async fn add_torrent(&self, url: &str, category: Option<String>) -> Result<(), Error> {
        let urls = vec![url];
        self.add_torrents(&urls, category).await
    }

    pub async fn categories(&self) -> Result<Vec<Category>, Error> {
        let resp: Categories = self
            .http
            .get("/api/v2/torrents/categories")
            .recv_json()
            .await?;

        Ok(resp.categories.into_values().collect())
    }

    pub async fn add_category(&self, name: &str, save_path: &str) -> Result<(), Error> {
        if name.is_empty() {
            return Err(Error::new(
                ErrorKind::BadParameters("name".to_string()),
                "Name is empty",
            ));
        }

        let form = AddCategoryOptions {
            category: name.to_string(),
            save_path: save_path.to_string(),
        };
        let body = Body::from_form(&form).unwrap();

        let mut resp = self
            .http
            .post("/api/v2/torrents/createCategory")
            .body(body)
            .await?;

        if resp.status() != 200 {
            return Err(Error::new(
                ErrorKind::CategoryAddError,
                resp.body_string().await?,
            ));
        }

        Ok(())
    }

    pub async fn edit_category(&self, name: &str, save_path: &str) -> Result<(), Error> {
        if name.is_empty() {
            return Err(Error::new(
                ErrorKind::BadParameters("name".to_string()),
                "Name is empty",
            ));
        }

        let form = AddCategoryOptions {
            category: name.to_string(),
            save_path: save_path.to_string(),
        };
        let body = Body::from_form(&form).unwrap();

        let mut resp = self
            .http
            .post("/api/v2/torrents/editCategory")
            .body(body)
            .await?;

        if resp.status() != 200 {
            return Err(Error::new(
                ErrorKind::CategoryAddError,
                resp.body_string().await?,
            ));
        }

        Ok(())
    }

    pub async fn ensure_category(&self, name: &str, save_path: &str) -> Result<(), Error> {
        let categories = self.categories().await?;

        let category = categories.iter().find(|c| c.name == name);
        match category {
            None => {
                self.add_category(name, save_path).await?;
            }
            Some(c) => {
                if c.save_path != save_path {
                    self.edit_category(name, save_path).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn torrents(&self, category: Option<String>) -> Result<Vec<Torrent>, Error> {
        let query = json!({"category": category});
        let query = query.as_object().unwrap();
        println!("{:?}", query);
        let resp: Vec<Torrent> = self
            .http
            .get("/api/v2/torrents/info")
            .query(query)
            .unwrap()
            .recv_json()
            .await?;

        Ok(resp)
    }
}
