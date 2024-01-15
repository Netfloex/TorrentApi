use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MovieInfo {
    title: String,
    year: u16,
}

impl MovieInfo {
    pub fn format(&self) -> String {
        format!("{} ({})", self.title, self.year)
    }
}
