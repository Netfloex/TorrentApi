use super::super::get_context;
use crate::{add_torrent_options::ApiAddTorrentOptions, http_error::HttpErrorKind};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct AddTorrentsMutation;

#[Object]
impl AddTorrentsMutation {
    async fn add_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        urls: Vec<String>,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        get_context(context)
            .qbittorrent_client()
            .add_torrents(&urls, options.unwrap_or_default().into())
            .await?;

        Ok("Ok".into())
    }
}
