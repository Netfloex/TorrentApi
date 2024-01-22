use crate::{models::movie_info::MovieInfo, Error, MovieInfoClient};
use log::debug;

impl MovieInfoClient {
    pub async fn bulk(&self, tmdb_ids: &Vec<i32>) -> Result<Vec<MovieInfo>, Error> {
        let mut movies: Vec<serde_json::Value> = self
            .http
            .post("movie/bulk")
            .body_json(tmdb_ids)
            .unwrap()
            .recv_json()
            .await?;

        movies.retain(|m| {
            let retain = m["Year"] != 0;
            if !retain {
                debug!("No movie found for tmdb_id: {}", m["TmdbId"])
            };
            retain
        });

        Ok(movies
            .into_iter()
            .map(|m| serde_json::from_value(m).unwrap())
            .collect())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CLIENT: MovieInfoClient = MovieInfoClient::new();
        static ref TMDB_IDS: Vec<i32> = vec![
            603, // The Matrix
            604, // The Matrix Reloaded
            605, // The Matrix Revolutions
        ];
    }

    #[tokio::test]
    async fn test_multiple_tmdb_ids() {
        let movies = CLIENT.bulk(&TMDB_IDS).await.unwrap();

        assert_eq!(movies.len(), 3);
        assert_eq!(movies[0].title(), "The Matrix");
        assert_eq!(movies[0].tmdb_id(), &TMDB_IDS[0]);

        assert_eq!(movies[1].title(), "The Matrix Reloaded");
        assert_eq!(movies[1].tmdb_id(), &TMDB_IDS[1]);

        assert_eq!(movies[2].title(), "The Matrix Revolutions");
        assert_eq!(movies[2].tmdb_id(), &TMDB_IDS[2]);
    }

    #[tokio::test]
    async fn test_tmdb_not_found() {
        let movies = CLIENT.bulk(&vec![0]).await.unwrap();

        assert!(movies.is_empty());
    }

    #[tokio::test]
    async fn test_tmdb_ids_with_missing() {
        let movies = CLIENT.bulk(&vec![0, 603, 0, 604, 0, 605, 0]).await.unwrap();

        assert_eq!(movies.len(), 3);

        assert_eq!(movies[0].title(), "The Matrix");
        assert_eq!(movies[0].tmdb_id(), &TMDB_IDS[0]);

        assert_eq!(movies[1].title(), "The Matrix Reloaded");
        assert_eq!(movies[1].tmdb_id(), &TMDB_IDS[1]);

        assert_eq!(movies[2].title(), "The Matrix Revolutions");
        assert_eq!(movies[2].tmdb_id(), &TMDB_IDS[2]);
    }
}
