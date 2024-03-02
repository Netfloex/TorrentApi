use crate::{http_error::HttpErrorKind, r#static::media_file_extensions::MEDIA_FILE_EXTENSIONS};
use std::{ffi::OsStr, path::PathBuf};
use tokio::fs::{self, DirEntry};

pub async fn import_movie(
    local_path: &PathBuf,
    dest_folder: &PathBuf,
) -> Result<(), HttpErrorKind> {
    if !local_path.try_exists()? {
        return Err(HttpErrorKind::TorrentNotFound(format!(
            "Torrent not found on disk at {}",
            local_path.display()
        )));
    }

    let movie = if !local_path.metadata()?.is_dir() {
        local_path.to_owned()
    } else {
        let mut files = fs::read_dir(local_path).await?;
        let mut movie_file: Option<DirEntry> = None;
        let mut max_size = 0;

        while let Some(file) = files.next_entry().await? {
            if let Some(ext) = file.path().extension().and_then(|s| s.to_str()) {
                if MEDIA_FILE_EXTENSIONS.contains(&ext) && !file.path().is_dir() {
                    let metadata = file.metadata().await?;
                    debug!("Found file: {:?}, {}b", file.path(), metadata.len());
                    if metadata.len() >= max_size {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{fs::File, io::AsyncWriteExt};

    use tempdir::TempDir;

    #[tokio::test]
    async fn test_import_movie() {
        let tmp_dir = TempDir::new("test_import_movie").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let movie_path = tmp_dir_path.join("movie.mp4");
        File::create(&movie_path).await.unwrap();

        let dest_folder = tmp_dir_path.join("dest");
        import_movie(&movie_path, &dest_folder).await.unwrap();

        assert!(dest_folder.exists());
        assert!(dest_folder.join("movie.mp4").exists());
    }

    #[tokio::test]
    async fn test_import_movie_from_folder() {
        let tmp_dir = TempDir::new("test_import_movie_from_folder").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let movie_path = tmp_dir_path.join("movie.mp4");
        File::create(&movie_path).await.unwrap();

        let dest_folder = tmp_dir_path.join("dest");
        import_movie(&tmp_dir_path.to_owned(), &dest_folder)
            .await
            .unwrap();

        assert!(dest_folder.exists());
        assert!(dest_folder.join("movie.mp4").exists());
    }

    #[tokio::test]
    async fn test_multiple_files() {
        let tmp_dir = TempDir::new("test_multiple_files").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let movie_path = tmp_dir_path.join("empty_movie.mp4");
        File::create(&movie_path).await.unwrap();
        let movie_path2 = tmp_dir_path.join("larger_movie.mp4");
        let mut larger_movie = File::create(&movie_path2).await.unwrap();

        larger_movie.write_all(&[0; 100]).await.unwrap();

        let dest_folder = tmp_dir_path.join("dest");
        import_movie(&tmp_dir_path.to_owned(), &dest_folder)
            .await
            .unwrap();

        assert!(dest_folder.exists());
        assert!(dest_folder.join("larger_movie.mp4").exists());
        assert!(!dest_folder.join("empty_movie.mp4").exists());
    }
}
