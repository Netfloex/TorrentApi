use crate::{
    models::http_error::HttpErrorKind,
    r#static::{
        media_file_extensions::MEDIA_FILE_EXTENSIONS,
        subtitle_file_extensions::SUBTITLE_FILE_EXTENSIONS,
        subtitle_language_map::create_subtitle_language_map,
    },
    utils::parse_subtitle_language::parse_subtitle_language,
};
use log::{debug, info};
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

    let mut subtitles = vec![];

    let movie = if !local_path.metadata()?.is_dir() {
        local_path.to_owned()
    } else {
        let mut files = fs::read_dir(local_path).await?;
        let mut movie_file: Option<DirEntry> = None;
        let mut max_size = 0;

        while let Some(file) = files.next_entry().await? {
            if file.file_type().await?.is_dir() {
                continue;
            }

            if let Some(ext) = file.path().extension().and_then(|s| s.to_str()) {
                let metadata = file.metadata().await?;

                if MEDIA_FILE_EXTENSIONS.contains(&ext) {
                    debug!("Found file: {:?}, {}b", file.path(), metadata.len());
                    if metadata.len() >= max_size {
                        max_size = metadata.len();
                        movie_file = Some(file);
                    }
                } else if SUBTITLE_FILE_EXTENSIONS.contains(&ext) {
                    debug!("Found subtitle: {:?}", file.path());

                    subtitles.push(file);
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
    fs::hard_link(movie, &dest_file).await?;
    info!("Movie copied to: {:?}", dest_file);

    // Copy subtitles
    let subtitle_language_map = create_subtitle_language_map();
    for subtitle in subtitles {
        let subtitle_name = subtitle.file_name();
        let subtitle_name = subtitle_name.to_string_lossy();
        let path = subtitle.path();

        let subtitle_ext = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext,
            None => continue,
        };

        let dest_file = match dest_file.file_stem().and_then(|d| d.to_str()) {
            Some(stem) => stem,
            None => continue,
        };

        let new_subtitle_path = parse_subtitle_language(
            &subtitle_name,
            subtitle_ext,
            dest_file,
            &subtitle_language_map,
        );

        debug!("Importing subtitle {} to {:?}", subtitle_name, dest_file);

        if let Some(new_subtitle_path) = new_subtitle_path {
            let dest_subtitle = dest_folder.join(new_subtitle_path);
            info!("Copying subtitle to {:?}", dest_subtitle);
            fs::hard_link(subtitle.path(), &dest_subtitle).await?;
            info!("Subtitle copied to: {:?}", dest_subtitle);
        }
    }

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
