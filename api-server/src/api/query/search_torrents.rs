use super::super::get_context;
use crate::{
    models::http_error::HttpErrorKind,
    search_handler::{search_handler, SearchHandlerParams, SearchHandlerResponse},
};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct SearchTorrentsQuery;

#[Object]
impl SearchTorrentsQuery {
    async fn search_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        params: SearchHandlerParams,
    ) -> Result<SearchHandlerResponse, HttpErrorKind> {
        let ctx = get_context(context);

        let torrents =
            search_handler(params, ctx.torrent_client(), ctx.movie_info_client()).await?;
        Ok(torrents)
    }
}
