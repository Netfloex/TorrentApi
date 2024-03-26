use super::super::get_context;
use crate::http_error::HttpErrorKind;
use async_graphql::{Context, Object};
use qbittorrent_api::AddTorrentOptions;

#[derive(Default)]
pub struct AddTorrentsMutation;

#[Object]
impl AddTorrentsMutation {
    async fn add_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        urls: Vec<String>,
        options: Option<AddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        get_context(context)
            .qbittorrent_client()
            .add_torrents(&urls, options.unwrap_or_default())
            .await?;

        Ok("Ok".into())
    }
}
