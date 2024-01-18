use std::path::PathBuf;

use tokio::fs;

use crate::{http_error::HttpErrorKind, r#static::media_file_extensions::MEDIA_FILE_EXTENSIONS};

pub async fn import_movie(local_path: PathBuf, dest_folder: PathBuf) -> Result<(), HttpErrorKind> {
    if !local_path.try_exists()? {
        return Err(HttpErrorKind::TorrentNotFound(format!(
            "Torrent not found on disk at {}",
            local_path.display()
        )));
    }

    if !local_path.metadata()?.is_dir() {
        Err(HttpErrorKind::TorrentIsFile(
            "Single file torrents are not yet supported.".into(),
        ))
    } else {
        for entry in local_path.read_dir()? {
            let entry = entry?;
            if let Some(ext) = entry.path().extension().map(|s| s.to_str()).flatten() {
                if MEDIA_FILE_EXTENSIONS.contains(&ext) {
                    info!("Importing Movie file: \n{}", entry.path().to_string_lossy());

                    if entry.path().is_dir() {
                        warn!("Found directory with movie extension: {:?}", entry.path());
                    } else {
                        fs::create_dir_all(&dest_folder).await?;

                        let dest_file = dest_folder.join(entry.file_name());

                        info!("Copying to {:?}", dest_file);
                        fs::copy(entry.path(), &dest_file).await?;
                        info!("Movie copied to: {:?}", dest_file);
                    }
                }
            }
        }
        Ok(())
    }
}
