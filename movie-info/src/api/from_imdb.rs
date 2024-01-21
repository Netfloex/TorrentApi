use crate::{models::movie_info::MovieInfo, Error, MovieInfoClient};

impl MovieInfoClient {
    pub async fn from_imdb(&self, imdb: &str) -> Result<Option<MovieInfo>, Error> {
        let movie: Vec<MovieInfo> = self
            .http
            .get(format!("movie/imdb/{}", imdb))
            .recv_json()
            .await?;

        Ok(movie.into_iter().next())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CLIENT: MovieInfoClient = MovieInfoClient::new();
    }

    const IMDB_ID: &str = "tt0133093";

    #[tokio::test]
    async fn from_imdb() {
        let movie = CLIENT.from_imdb(IMDB_ID).await.unwrap();

        assert!(movie.is_some());
        let movie = movie.unwrap();

        assert_eq!(movie.title(), "The Matrix");
        assert_eq!(movie.imdb_id().as_ref().unwrap(), IMDB_ID);
    }

    #[tokio::test]
    async fn from_imdb_not_found() {
        let movie = CLIENT.from_imdb("tt0000000").await.unwrap();

        assert!(movie.is_none());
    }
}
