use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use log::debug;

use crate::{
    models::http_error::HttpErrorKind,
    r#static::{
        media_file_extensions::MEDIA_FILE_EXTENSIONS,
        subtitle_file_extensions::SUBTITLE_FILE_EXTENSIONS,
    },
};

pub struct MovieFiles {
    movie: PathBuf,
    subtitles: Vec<PathBuf>,
}

impl MovieFiles {
    fn get_folder_files(
        local_path: &PathBuf,
        max_depth: u8,
    ) -> Result<Vec<DirEntry>, HttpErrorKind> {
        let files = fs::read_dir(local_path)?;
        let mut result = vec![];

        for file in files {
            let file = file?;
            let metadata = file.metadata()?;

            if metadata.is_dir() && max_depth > 0 {
                let folder_files = Self::get_folder_files(&file.path(), max_depth - 1)?;
                result.extend(folder_files);
            } else if metadata.is_file() {
                result.push(file);
            }
        }

        Ok(result)
    }

    pub fn get_movie_files(
        local_path: &PathBuf,
        max_depth: u8,
    ) -> Result<MovieFiles, HttpErrorKind> {
        let mut subtitles = vec![];

        if local_path.metadata()?.is_file() {
            return Ok(MovieFiles {
                movie: local_path.clone(),
                subtitles,
            });
        };

        let files = Self::get_folder_files(local_path, max_depth)?;
        let mut movie_file: Option<DirEntry> = None;
        let mut highest_size = 0;

        for file in files {
            if let Some(ext) = file.path().extension().and_then(|s| s.to_str()) {
                let metadata = file.metadata()?;
                let path = file.path();

                if MEDIA_FILE_EXTENSIONS.contains(&ext) {
                    debug!("Found file: {:?}, {}b", path, metadata.len());
                    if metadata.len() >= highest_size {
                        highest_size = metadata.len();
                        movie_file = Some(file);
                    }
                } else if SUBTITLE_FILE_EXTENSIONS.contains(&ext) {
                    debug!("Found subtitle: {:?}", path);

                    subtitles.push(path);
                }
            }
        }

        match movie_file {
            Some(movie_file) => Ok(MovieFiles {
                movie: movie_file.path(),
                subtitles,
            }),
            None => Err(HttpErrorKind::MovieFileNotFound(
                "No movie file found in torrent.".into(),
            )),
        }
    }

    pub fn movie(&self) -> &PathBuf {
        &self.movie
    }

    pub fn subtitles(&self) -> &Vec<PathBuf> {
        &self.subtitles
    }
}
