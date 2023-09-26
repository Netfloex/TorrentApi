mod auth_middleware;
mod error;
use auth_middleware::AuthMiddleware;
use error::Error;
use error::ErrorKind;
use serde::Deserialize;
use serde::Serialize;
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

    pub async fn add_torrent(&self, url: &str) -> Result<(), Error> {
        let form = AddTorrentOptions {
            urls: url.to_owned(),
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
}
