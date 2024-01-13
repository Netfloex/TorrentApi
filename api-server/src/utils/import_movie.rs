use crate::{
    http_error::HttpErrorKind, r#static::media_file_extensions::MEDIA_FILE_EXTENSIONS,
    utils::hash_from_magnet::hash_from_magnet,
};
use qbittorrent_api::{AddTorrentOptions, PartialTorrent, QbittorrentClient};
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn import_movie(
    client: &mut QbittorrentClient,
    url: String,
    dest_folder: &PathBuf,
) -> Result<PartialTorrent, HttpErrorKind> {
    let hash =
        hash_from_magnet(&url).ok_or(HttpErrorKind::InvalidMagnet("Invalid Magnet Link".into()))?;
    let options = AddTorrentOptions::default();

    client.add_torrent(url, options).await?;

    let a = client.wait_torrent_completion(&hash).await?;
    println!("a: {:?}", a);

    println!("import_torrent: {}", hash);

    let path = a.content_path().as_ref().expect("Path should be available");
    // TODO: Convert between remote and local path
    let local_path = &path;
    let torrent = Path::new(&local_path);

    if !torrent.try_exists()? {
        Err(HttpErrorKind::TorrentNotFound(format!(
            "Torrent not found on disk at {}",
            torrent.display()
        )))?
    }

    if !torrent.metadata()?.is_dir() {
        Err(HttpErrorKind::TorrentIsFile(
            "Single file torrents are not yet supported.".into(),
        ))
    } else {
        for entry in torrent.read_dir()? {
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
