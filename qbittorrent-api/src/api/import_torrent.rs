use crate::{
    models::partial_torrent::PartialTorrent, utils::hash_from_magnet::hash_from_magnet,
    AddTorrentOptions, Error, ErrorKind, QbittorrentClient,
};

impl QbittorrentClient {
    pub async fn import_torrent(&mut self, url: String) -> Result<PartialTorrent, Error> {
        let hash = hash_from_magnet(&url)
            .ok_or(Error::new(ErrorKind::InvalidMagnet, "Invalid Magnet Link"))?;

        let options = AddTorrentOptions::default();

        self.add_torrent(url, options).await?;

        let a = self.wait_torrent_completion(&hash).await?;
        println!("a: {:?}", a);

        println!("import_torrent: {}", hash);

        Ok(a)
    }
}
