use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "CoverType")]
    cover_type: String,
    #[serde(rename = "Url")]
    url: String,
}

impl Image {
    pub fn url(self) -> String {
        self.url
    }

    pub fn cover_type(&self) -> &String {
        &self.cover_type
    }
}
