use crate::{
    models::partial_torrent::PartialTorrent,
    r#static::media_file_extensions::MEDIA_FILE_EXTENSIONS,
    utils::{hash_from_magnet::hash_from_magnet, path_converter::remote_to_local},
    AddTorrentOptions, Error, ErrorKind, QbittorrentClient,
};
use std::path::{Path, PathBuf};
use tokio::fs;

impl QbittorrentClient {
    pub async fn import_torrent(
        &mut self,
        url: String,
        dest_folder: &Path,
    ) -> Result<PartialTorrent, Error> {
        let hash = hash_from_magnet(&url)
            .ok_or(Error::new(ErrorKind::InvalidMagnet, "Invalid Magnet Link"))?;
        let options = AddTorrentOptions::default();

        self.add_torrent(url, options).await?;

        let a = self.wait_torrent_completion(&hash).await?;
        println!("a: {:?}", a);

        println!("import_torrent: {}", hash);

        let path = a.content_path().as_ref().expect("Path should be available");
        let local_path = remote_to_local(&path);

        let file = Path::new(&local_path);

        if !file.metadata()?.is_dir() {
            Err(Error::new(
                ErrorKind::TorrentIsFile,
                "Single file torrents are not yet supported.",
            ))
        } else {
            for entry in file.read_dir()? {
                let entry = entry?;
                if let Some(ext) = entry.path().extension().map(|s| s.to_str()).flatten() {
                    if MEDIA_FILE_EXTENSIONS.contains(&ext) {
                        println!("Importing Movie file: \n{}", entry.path().to_string_lossy());

                        if !entry.path().is_dir() {
                            println!("Found directory with movie extension: {:?}", entry.path());
                        }

                        fs::create_dir_all(&dest_folder).await?;

                        let dest_file = dest_folder.join(entry.file_name());

                        println!("{:?}", dest_file);
                        fs::copy(entry.path(), &dest_file).await?;
                        println!("Movie copied to: {:?}", dest_file)
                    }
                }
            }
            Ok(a)
        }
    }
}
