use derive_getters::Getters;
use serde::{Deserialize, Deserializer};

use super::image::Image;

fn deserialize_headshot_url<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Vec<Image> = Deserialize::deserialize(deserializer)?;

    let headshot_url = raw
        .into_iter()
        .find(|image| image.cover_type() == "Headshot")
        .map(|image| image.url());

    Ok(headshot_url)
}

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct CastItem {
    name: String,
    order: i32,
    character: String,
    tmdb_id: i32,
    credit_id: String,
    #[serde(deserialize_with = "deserialize_headshot_url")]
    #[serde(rename = "Images")]
    headshot_url: Option<String>,
}

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct CrewItem {
    name: String,
    job: String,
    department: String,
    tmdb_id: i32,
    credit_id: String,
    #[serde(deserialize_with = "deserialize_headshot_url")]
    #[serde(rename = "Images")]
    headshot_url: Option<String>,
}

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct Credits {
    cast: Vec<CastItem>,
    crew: Vec<CrewItem>,
}
