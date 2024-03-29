use chrono::{DateTime, Utc};
use getset::Getters;
use serde::{Deserialize, Deserializer};

use super::{
    certification::Certification, collection::Collection, credits::Credits, image::Image,
    ratings::MovieRatings, recommendation::Recommendation, tmdb_id::TmdbId,
};

fn deserialize_poster_url<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Vec<Image> = Deserialize::deserialize(deserializer)?;

    let poster_url = raw
        .into_iter()
        .find(|image| image.cover_type() == "Poster")
        .map(|image| image.url());

    Ok(poster_url)
}

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[getset(get = "pub with_prefix")]
pub struct MovieInfo {
    imdb_id: Option<String>,
    overview: String,
    title: String,
    original_title: String,
    runtime: u16,
    year: u16,
    movie_ratings: MovieRatings,
    genres: Vec<String>,
    #[serde(deserialize_with = "deserialize_poster_url")]
    #[serde(rename = "Images")]
    poster_url: Option<String>,
    physical_release: Option<DateTime<Utc>>,
    digital_release: Option<DateTime<Utc>>,
    in_cinema: Option<DateTime<Utc>>,
    recommendations: Vec<Recommendation>,
    credits: Credits,
    studio: String,
    youtube_trailer_id: Option<String>,
    certifications: Vec<Certification>,
    collection: Option<Collection>,
    original_language: String,
    homepage: String,
    tmdb_id: TmdbId,
}

impl MovieInfo {
    pub fn format(&self) -> String {
        format!("{} ({})", self.title, self.year)
    }

    pub fn certifications_mut(&mut self) -> &mut Vec<Certification> {
        &mut self.certifications
    }
}
