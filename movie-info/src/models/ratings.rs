use getset::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[getset(get = "pub with_prefix")]
pub struct MovieRating {
    pub value: f64,
    pub count: i32,
}
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[getset(get = "pub with_prefix")]
pub struct MovieRatings {
    tmdb: Option<MovieRating>,
    imdb: Option<MovieRating>,
    metacritic: Option<MovieRating>,
    rotten_tomatoes: Option<MovieRating>,
}
