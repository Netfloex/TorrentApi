use crate::api::{
    mutation::{
        add_torrents::AddTorrentsMutation, delete_torrents::DeleteTorrentsMutation,
        track_movie::TrackMovieMutation,
    },
    query::{
        active_torrents::ActiveTorrentsQuery, movie_info::MovieInfoQuery,
        popular_movies::PopularMoviesQuery, search_filters::SearchFiltersQuery,
        search_movies::SearchMoviesQuery, search_torrents::SearchTorrentsQuery,
        tmdb_bulk::TmdbBulkQuery, trending_movies::TrendingMoviesQuery,
    },
};
use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{
    response::content::{self},
    State,
};

pub type SchemaType = Schema<Query, Mutation, EmptySubscription>;

#[derive(Default, MergedObject)]
pub struct Query(
    ActiveTorrentsQuery,
    MovieInfoQuery,
    PopularMoviesQuery,
    SearchFiltersQuery,
    SearchMoviesQuery,
    SearchTorrentsQuery,
    TmdbBulkQuery,
    TrendingMoviesQuery,
);

#[derive(Default, MergedObject)]
pub struct Mutation(
    AddTorrentsMutation,
    DeleteTorrentsMutation,
    TrackMovieMutation,
);

#[rocket::get("/")]
pub fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<SchemaType>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<SchemaType>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}
