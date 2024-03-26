use super::super::get_context;
use crate::models::http_error::HttpErrorKind;
use async_graphql::{Context, Object};
use movie_info::MovieInfo;

#[derive(Default)]
pub struct PopularMoviesQuery;

#[Object]
impl PopularMoviesQuery {
    async fn popular_movies<'ctx>(
        &self,
        context: &Context<'ctx>,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = get_context(context);

        let movie_info = ctx
            .movie_info_client()
            .popular(ctx.config().filters())
            .await?;

        Ok(movie_info)
    }
}
