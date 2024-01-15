use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct Image {
    #[serde(rename = "CoverType")]
    cover_type: String,
    #[serde(rename = "Url")]
    url: String,
}

fn deserialize_poster_url<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Vec<Image> = Deserialize::deserialize(deserializer)?;

    let poster_url = raw
        .iter()
        .find(|image| image.cover_type == "Poster")
        .map(|image| image.url.clone());

    Ok(poster_url)
}

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]

pub struct MovieRating {
    pub value: f32,
    pub count: u32,
}

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct MovieRatings {
    tmdb: Option<MovieRating>,
    imdb: Option<MovieRating>,
    metacritic: Option<MovieRating>,
    rotten_tomatoes: Option<MovieRating>,
}

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct MovieInfo {
    imdb_id: Option<String>,
    overview: String,
    title: String,
    original_title: String,
    runtime: u16,
    year: u16,
    tmdb_id: u32,
    movie_ratings: MovieRatings,
    genres: Vec<String>,
    #[serde(deserialize_with = "deserialize_poster_url")]
    #[serde(rename = "Images")]
    poster_url: Option<String>,
    physical_release: Option<DateTime<Utc>>,
    digital_release: Option<DateTime<Utc>>,
    in_cinema: Option<DateTime<Utc>>,
}

impl MovieInfo {
    pub fn format(&self) -> String {
        format!("{} ({})", self.title, self.year)
    }
}
