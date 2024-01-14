use derive_getters::Getters;
use derive_setters::Setters;
use url::Url;

#[derive(Getters, Debug, Setters)]
#[setters(prefix = "set_")]
pub struct Magnet {
    display_name: String,
    trackers: Vec<String>,
    info_hash: String,
}

impl Magnet {
    pub fn from_url(url: &str) -> Result<Self, String> {
        let url = url.to_lowercase();
        if !url.starts_with("magnet:") {
            Err("Not a magnet link".into())
        } else {
            let url = Url::parse(&url).map_err(|e| e.to_string())?;
            let mut qp = url.query_pairs();
            Ok(Self {
                display_name: qp
                    .find(|(k, _)| k == "dn")
                    .map(|(_, v)| v.into_owned())
                    .unwrap_or_else(|| "Unknown".into()),
                trackers: qp
                    .filter(|(k, _)| k == "tr")
                    .map(|(_, v)| v.into_owned())
                    .collect(),
                info_hash: qp
                    .find(|(k, _)| k == "xt")
                    .map(|(_, v)| v.into_owned())
                    .unwrap_or_else(|| "Unknown".into()),
            })
        }
    }

    pub fn url(&self) -> String {
        format!(
            "magnet:?xt=urn:btih:{}&dn={}&tr={}",
            self.info_hash,
            self.display_name,
            self.trackers.join("&tr=")
        )
    }
}
