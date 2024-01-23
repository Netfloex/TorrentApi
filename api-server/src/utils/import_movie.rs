use crate::{http_error::HttpErrorKind, r#static::media_file_extensions::MEDIA_FILE_EXTENSIONS};
use std::{ffi::OsStr, path::PathBuf};
use tokio::fs::{self, DirEntry};

pub async fn import_movie(local_path: PathBuf, dest_folder: PathBuf) -> Result<(), HttpErrorKind> {
    if !local_path.try_exists()? {
        return Err(HttpErrorKind::TorrentNotFound(format!(
            "Torrent not found on disk at {}",
            local_path.display()
        )));
    }

    let movie = if !local_path.metadata()?.is_dir() {
        local_path
    } else {
        let mut files = fs::read_dir(local_path).await?;
        let mut movie_file: Option<DirEntry> = None;
        let mut max_size = 0;

        while let Some(file) = files.next_entry().await? {
            if let Some(ext) = file.path().extension().map(|s| s.to_str()).flatten() {
                if MEDIA_FILE_EXTENSIONS.contains(&ext) && !file.path().is_dir() {
                    let metadata = file.metadata().await?;
                    debug!("Found file: {:?}, {}b", file.path(), metadata.len());
                    if metadata.len() > max_size {
                        max_size = metadata.len();
                        movie_file = Some(file);
                    }
                }
            }
        }

        if let Some(movie_file) = movie_file {
            movie_file.path()
        } else {
            return Err(HttpErrorKind::MovieFileNotFound(
                "No movie file found in torrent.".into(),
            ));
        }
    };
    fs::create_dir_all(&dest_folder).await?;

    let dest_file = dest_folder.join(movie.file_name().unwrap_or(OsStr::new("Unknown Movie")));

    info!("Copying to {:?}", dest_file);
    fs::copy(movie, &dest_file).await?;
    info!("Movie copied to: {:?}", dest_file);

    Ok(())
}
