use std::path::PathBuf;

use tokio::fs;

use crate::{http_error::HttpErrorKind, r#static::media_file_extensions::MEDIA_FILE_EXTENSIONS};

pub async fn import_movie(local_path: PathBuf, dest_folder: PathBuf) -> Result<(), HttpErrorKind> {
    if !local_path.try_exists()? {
        println!("File does not exist");
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
        Ok(())
    }
}
