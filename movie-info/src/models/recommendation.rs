use getset::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[getset(get = "pub with_prefix")]
pub struct Recommendation {
    tmdb_id: i32,
    title: String,
}
