use derive_getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct Recommendation {
    tmdb_id: i32,
    title: String,
}