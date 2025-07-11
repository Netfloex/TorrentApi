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

#[derive(Debug)]
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
                    debug!("Found subtitle: {path:?}");

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

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    use fs::{create_dir, File};
    use tempdir::TempDir;

    #[test]
    fn test_get_folder_files() {
        let temp_dir = TempDir::new("test_get_folder_files").unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();

        File::create(temp_dir_path.join("file1")).unwrap();
        File::create(temp_dir_path.join("file2")).unwrap();
        File::create(temp_dir_path.join("file3")).unwrap();

        let folder_files_0 = MovieFiles::get_folder_files(&temp_dir_path, 0).unwrap();
        let folder_files_1 = MovieFiles::get_folder_files(&temp_dir_path, 1).unwrap();
        assert_eq!(folder_files_0.len(), 3);
        assert_eq!(folder_files_1.len(), 3);

        let subs_dir = temp_dir_path.join("Subs");
        create_dir(&subs_dir).unwrap();

        File::create(subs_dir.join("sub1")).unwrap();

        let folder_files_0 = MovieFiles::get_folder_files(&temp_dir_path, 0).unwrap();
        let folder_files_1 = MovieFiles::get_folder_files(&temp_dir_path, 1).unwrap();
        assert_eq!(folder_files_0.len(), 3);
        assert_eq!(folder_files_1.len(), 4);
    }

    #[test]
    fn test_get_movie_files() {
        let temp_dir = TempDir::new("test_get_movie_files").unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();

        let movie_path = temp_dir_path.join("movie.mp4");
        File::create(&movie_path).unwrap();

        let movie_files = MovieFiles::get_movie_files(&temp_dir_path, 1).unwrap();
        assert_eq!(movie_files.movie(), &movie_path);
        assert_eq!(movie_files.subtitles().len(), 0);

        let subs_dir = temp_dir_path.join("Subs");
        create_dir(&subs_dir).unwrap();

        let subs_path = subs_dir.join("sub.srt");
        File::create(&subs_path).unwrap();

        let movie_files = MovieFiles::get_movie_files(&temp_dir_path, 1).unwrap();
        assert_eq!(movie_files.movie(), &movie_path);
        assert_eq!(movie_files.subtitles().len(), 1);
        assert_eq!(movie_files.subtitles()[0], subs_path);
    }

    #[test]
    fn test_get_larger_movie() {
        let temp_dir = TempDir::new("test_get_larger_movie").unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();

        let empty_movie = temp_dir_path.join("movie.mp4");
        File::create(&empty_movie).unwrap();

        let larger_movie_path = temp_dir_path.join("larger_movie.mp4");
        let mut larger_movie = File::create(&larger_movie_path).unwrap();

        larger_movie.write_all(&[0; 100]).unwrap();

        let movie_files = MovieFiles::get_movie_files(&temp_dir_path, 1).unwrap();
        assert_eq!(movie_files.movie(), &larger_movie_path);
        assert_eq!(movie_files.subtitles().len(), 0);
    }
}
