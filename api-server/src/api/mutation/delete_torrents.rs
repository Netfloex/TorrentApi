use super::super::get_context;
use crate::http_error::HttpErrorKind;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct DeleteTorrentsMutation;

#[Object]
impl DeleteTorrentsMutation {
    async fn delete_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        hashes: Vec<String>,
        delete_files: bool,
    ) -> Result<String, HttpErrorKind> {
        get_context(context)
            .qbittorrent_client()
            .delete_torrents(hashes, delete_files)
            .await?;

        Ok("Ok".into())
    }
}
