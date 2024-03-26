use super::super::get_context;
use crate::http_error::HttpErrorKind;
use async_graphql::{Context, Object};
use movie_info::MovieInfo;

#[derive(Default)]
pub struct TrendingMoviesQuery;

#[Object]
impl TrendingMoviesQuery {
    async fn trending_movies<'ctx>(
        &self,
        context: &Context<'ctx>,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = get_context(context);

        let movie_info = ctx
            .movie_info_client()
            .trending(ctx.config().filters())
            .await?;

        Ok(movie_info)
    }
}
