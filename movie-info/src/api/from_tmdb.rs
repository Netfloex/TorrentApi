use crate::{models::movie_info::MovieInfo, Error, MovieInfoClient};

impl MovieInfoClient {
    pub async fn from_tmdb(&self, tmdb: u32) -> Result<Option<MovieInfo>, Error> {
        let mut resp = self.http.get(format!("movie/{}", tmdb)).send().await?;

        if resp.status().is_client_error() {
            return Ok(None);
        }

        let movie: MovieInfo = resp.body_json().await?;

        Ok(Some(movie))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CLIENT: MovieInfoClient = MovieInfoClient::new();
    }

    const TMDB_ID: u32 = 603;

    #[tokio::test]
    async fn from_tmdb() {
        let movie = CLIENT.from_tmdb(TMDB_ID).await.unwrap();

        assert!(movie.is_some());
        let movie = movie.unwrap();

        assert_eq!(movie.title(), "The Matrix");
        assert_eq!(*movie.tmdb_id() as u32, TMDB_ID);
    }

    #[tokio::test]
    async fn from_tmdb_not_found() {
        let movie = CLIENT.from_tmdb(0).await.unwrap();

        assert!(movie.is_none());
    }
}
