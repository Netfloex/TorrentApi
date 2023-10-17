mod auth_middleware;
mod error;
use auth_middleware::AuthMiddleware;
use derive_getters::Getters;
use derive_setters::Setters;
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
    auto_tmm: bool,
    availability: f64,
    category: String,
    completed: i64,
    completion_on: i64,
    content_path: String,
    dl_limit: i64,
    dlspeed: i64,
    downloaded: i64,
    downloaded_session: i64,
    eta: i64,
    f_l_piece_prio: bool,
    force_start: bool,
    hash: String,
    last_activity: i64,
    magnet_uri: String,
    max_ratio: f64,
    max_seeding_time: i64,
    name: String,
    num_complete: i64,
    num_incomplete: i64,
    num_leechs: i64,
    num_seeds: i64,
    priority: i64,
    progress: f64,
    ratio: f64,
    ratio_limit: f64,
    save_path: String,
    seeding_time_limit: i64,
    seen_complete: i64,
    seq_dl: bool,
    size: i64,
    state: TorrentState,
    super_seeding: bool,
    tags: String,
    time_active: i64,
    total_size: i64,
    tracker: String,
    up_limit: i64,
    uploaded: i64,
    uploaded_session: i64,
    upspeed: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TorrentState {
    /// Some error occurred, applies to paused torrents
    Error,
    /// Torrent data files is missing
    MissingFiles,
    /// Torrent is being seeded and data is being transferred
    Uploading,
    /// Torrent is paused and has finished downloading
    PausedUP,
    /// Queuing is enabled and torrent is queued for upload
    QueuedUP,
    /// Torrent is being seeded, but no connection were made
    StalledUP,
    /// Torrent has finished downloading and is being checked
    CheckingUP,
    /// Torrent is forced to uploading and ignore queue limit
    ForcedUP,
    /// Torrent is allocating disk space for download
    Allocating,
    /// Torrent is being downloaded and data is being transferred
    Downloading,
    /// Torrent has just started downloading and is fetching metadata
    MetaDL,
    /// Torrent is paused and has NOT finished downloading
    PausedDL,
    /// Queuing is enabled and torrent is queued for download
    QueuedDL,
    /// Torrent is being downloaded, but no connection were made
    StalledDL,
    /// Same as checkingUP, but torrent has NOT finished downloading
    CheckingDL,
    /// Torrent is forced to downloading to ignore queue limit
    ForcedDL,
    /// Checking resume data on qBt startup
    CheckingResumeData,
    /// Torrent is moving to another location
    Moving,
    /// Unknown status
    Unknown,
}

fn serialize_hashes<S>(hashes: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match hashes {
        None => serializer.serialize_none(),
        Some(hashes) => hashes.join("|").serialize(serializer),
    }
}

#[derive(Serialize, Deserialize, Debug, Setters)]
#[setters(strip_option = true)]
pub struct GetTorrentsParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_hashes"
    )]
    hashes: Option<Vec<String>>,
}

impl GetTorrentsParameters {
    pub fn new() -> Self {
        Self {
            filter: None,
            category: None,
            tag: None,
            sort: None,
            reverse: None,
            limit: None,
            offset: None,
            hashes: None,
        }
    }
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

        if !resp.status().is_success() {
            return Err(Error::new(
                ErrorKind::RequestError,
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

        if !resp.status().is_success() {
            return Err(Error::new(
                ErrorKind::RequestError,
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

    pub async fn torrents(&self, options: GetTorrentsParameters) -> Result<Vec<Torrent>, Error> {
        let mut resp = self
            .http
            .get("/api/v2/torrents/info")
            .query(&options)
            .unwrap()
            .await?;

        if resp.status().is_success() {
            Ok(resp.body_json().await?)
        } else {
            let body = resp.body_string().await?;
            if body.ends_with("parameter is invalid") {
                Err(Error::new(
                    ErrorKind::BadParameters(body.replace(" parameter is invalid", "")),
                    "message",
                ))
            } else {
                Err(Error::new(ErrorKind::RequestError, body))
            }
        }
    }
}
