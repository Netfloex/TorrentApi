use crate::{
    models::{movie_info::MovieInfo, tmdb_id::TmdbId},
    Error, MovieInfoClient,
};

impl MovieInfoClient {
    pub async fn from_tmdb(&self, tmdb: TmdbId) -> Result<Option<MovieInfo>, Error> {
        let mut resp = self.http.get(format!("movie/{tmdb}")).send().await?;

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

    const TMDB_ID: TmdbId = 603;

    #[tokio::test]
    async fn from_tmdb() {
        let movie = CLIENT.from_tmdb(TMDB_ID).await.unwrap();

        assert!(movie.is_some());
        let movie = movie.unwrap();

        assert_eq!(movie.get_title(), "The Matrix");
        assert_eq!(*movie.get_tmdb_id(), TMDB_ID);
    }

    #[tokio::test]
    async fn from_tmdb_not_found() {
        let movie = CLIENT.from_tmdb(0).await.unwrap();

        assert!(movie.is_none());
    }
}
