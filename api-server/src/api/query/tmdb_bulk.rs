use super::super::get_context;
use crate::models::http_error::HttpErrorKind;
use async_graphql::{Context, Object};
use movie_info::{MovieInfo, TmdbId};
use std::collections::HashSet;

#[derive(Default)]
pub struct TmdbBulkQuery;

#[Object]
impl TmdbBulkQuery {
    async fn tmdb_bulk<'ctx>(
        &self,
        context: &Context<'ctx>,
        tmdb_ids: HashSet<TmdbId>,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let movie_info = get_context(context)
            .movie_info_client()
            .bulk(&tmdb_ids)
            .await?;

        Ok(movie_info)
    }
}
