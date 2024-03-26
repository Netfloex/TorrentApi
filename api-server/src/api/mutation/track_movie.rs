use crate::{
    models::{context::ContextPointer, http_error::HttpErrorKind},
    utils::track_movie::track_movie,
};
use async_graphql::{Context, Object};
use movie_info::TmdbId;

#[derive(Default)]
pub struct TrackMovieMutation;

#[Object]
impl TrackMovieMutation {
    async fn track_movie<'ctx>(
        &self,
        context: &Context<'ctx>,
        url: String,
        tmdb: TmdbId,
    ) -> Result<String, HttpErrorKind> {
        track_movie(context.data::<ContextPointer>().unwrap(), url, tmdb).await?;

        Ok("Ok".into())
    }
}
