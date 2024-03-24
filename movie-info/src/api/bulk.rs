use std::collections::HashSet;

use crate::{
    models::{movie_info::MovieInfo, tmdb_id::TmdbId},
    Error, MovieInfoClient,
};
use log::debug;

impl MovieInfoClient {
    pub async fn bulk(&self, tmdb_ids: &HashSet<TmdbId>) -> Result<Vec<MovieInfo>, Error> {
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
    }

    #[tokio::test]
    async fn test_multiple_tmdb_ids() {
        let tmdb_ids = [
            603, // The Matrix
            604, // The Matrix Reloaded
            605, // The Matrix Revolutions
        ];

        let movies = CLIENT
            .bulk(&tmdb_ids.iter().cloned().collect())
            .await
            .unwrap();

        assert_eq!(movies.len(), 3);
        assert_eq!(movies[0].get_title(), "The Matrix");
        assert_eq!(movies[0].get_tmdb_id(), &tmdb_ids[0]);

        assert_eq!(movies[1].get_title(), "The Matrix Reloaded");
        assert_eq!(movies[1].get_tmdb_id(), &tmdb_ids[1]);

        assert_eq!(movies[2].get_title(), "The Matrix Revolutions");
        assert_eq!(movies[2].get_tmdb_id(), &tmdb_ids[2]);
    }

    #[tokio::test]
    async fn test_tmdb_not_found() {
        let movies = CLIENT.bulk(&[0].into_iter().collect()).await.unwrap();

        assert!(movies.is_empty());
    }

    #[tokio::test]
    async fn test_tmdb_ids_with_missing() {
        let tmdb_ids = [
            0,   // Missing
            603, // The Matrix
            604, // The Matrix Reloaded
            605, // The Matrix Revolutions
        ];

        let movies = CLIENT
            .bulk(&tmdb_ids.iter().cloned().collect())
            .await
            .unwrap();

        assert_eq!(movies.len(), 3);

        assert_eq!(movies[0].get_title(), "The Matrix");
        assert_eq!(movies[0].get_tmdb_id(), &tmdb_ids[0]);

        assert_eq!(movies[1].get_title(), "The Matrix Reloaded");
        assert_eq!(movies[1].get_tmdb_id(), &tmdb_ids[1]);

        assert_eq!(movies[2].get_title(), "The Matrix Revolutions");
        assert_eq!(movies[2].get_tmdb_id(), &tmdb_ids[2]);
    }
}
