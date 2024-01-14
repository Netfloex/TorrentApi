use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MovieInfo {
    title: String,
    year: u16,
}

impl MovieInfo {
    pub async fn from_tmdb(tmdb: u32) -> Self {
        print!("tmdb: {}", tmdb);
        let movie: MovieInfo = surf::get(format!("https://api.radarr.video/v1/movie/{}", tmdb))
            .recv_json()
            .await
            .unwrap();
        movie
    }

    pub fn format(&self) -> String {
        format!("{} ({})", self.title, self.year)
    }
}
