use super::super::get_context;
use crate::models::http_error::HttpErrorKind;
use async_graphql::{Context, Object};
use movie_info::{MovieInfo, TmdbId};

#[derive(Default)]
pub struct MovieInfoQuery;

#[Object]
impl MovieInfoQuery {
    async fn movie_info<'ctx>(
        &self,
        context: &Context<'ctx>,
        tmdb: TmdbId,
    ) -> Result<Option<MovieInfo>, HttpErrorKind> {
        let ctx = get_context(context);

        let movie_info = ctx.movie_info_client().from_tmdb(tmdb).await?;

        Ok(movie_info)
    }
}
