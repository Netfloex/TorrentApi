use crate::{
    models::{http_error::HttpErrorKind, movie_files::MovieFiles, serde_regex::SerdeRegex},
    utils::parse_subtitle_language::parse_subtitle_language,
};
use log::{debug, info};
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};
use tokio::fs::{self};

pub async fn import_movie(
    local_path: &PathBuf,
    dest_folder: &PathBuf,
    max_depth: u8,
    subtitle_language_map: &HashMap<String, SerdeRegex>,
) -> Result<(), HttpErrorKind> {
    if !local_path.try_exists()? {
        return Err(HttpErrorKind::TorrentNotFound(format!(
            "Torrent not found on disk at {}",
            local_path.display()
        )));
    }

    let movie_files = MovieFiles::get_movie_files(local_path, max_depth)?;

    fs::create_dir_all(&dest_folder).await?;

    let movie_dest_file = dest_folder.join(
        movie_files
            .movie()
            .file_name()
            .unwrap_or(OsStr::new("Unknown Movie")),
    );

    info!("Copying to {movie_dest_file:?}");
    fs::copy(movie_files.movie(), &movie_dest_file).await?;
    info!("Movie copied to: {movie_dest_file:?}");

    // Copy subtitles
    for subtitle in movie_files.subtitles() {
        let subtitle_stem = match subtitle.file_stem().and_then(|name| name.to_str()) {
            Some(name) => name,
            None => continue,
        };

        let subtitle_ext = match subtitle.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext,
            None => continue,
        };

        let target_base = match movie_dest_file.file_stem().and_then(|d| d.to_str()) {
            Some(stem) => stem,
            None => continue,
        };

        let new_subtitle_path = parse_subtitle_language(
            subtitle_stem,
            subtitle_ext,
            target_base,
            subtitle_language_map,
        );

        if let Some(new_subtitle_path) = new_subtitle_path {
            info!("Importing subtitle {subtitle_stem} to as {new_subtitle_path}");

            let dest_subtitle = dest_folder.join(new_subtitle_path);
            debug!("Copying subtitle to {dest_subtitle:?}");
            fs::copy(subtitle, &dest_subtitle).await?;
            debug!("Subtitle copied to: {dest_subtitle:?}");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use tokio::{fs::File, io::AsyncWriteExt};

    #[tokio::test]
    async fn test_import_movie() {
        let tmp_dir = TempDir::new("test_import_movie").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let movie_path = tmp_dir_path.join("movie.mp4");
        File::create(&movie_path).await.unwrap();

        let dest_folder = tmp_dir_path.join("dest");
        import_movie(&movie_path, &dest_folder, 1, &HashMap::new())
            .await
            .unwrap();

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
        import_movie(&tmp_dir_path.to_owned(), &dest_folder, 1, &HashMap::new())
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
        import_movie(&tmp_dir_path.to_owned(), &dest_folder, 1, &HashMap::new())
            .await
            .unwrap();

        assert!(dest_folder.exists());
        assert!(dest_folder.join("larger_movie.mp4").exists());
        assert!(!dest_folder.join("empty_movie.mp4").exists());
    }
}
