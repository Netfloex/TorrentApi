mod auth_middleware;
mod error;
use auth_middleware::AuthMiddleware;
use error::Error;
use error::ErrorKind;
use serde::Serialize;
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
}
