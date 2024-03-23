use serde::Serialize;

use crate::{
    models::{filters::Filters, movie_info::MovieInfo},
    utils::parse_imdb_id::parse_imdb_id,
    Error, MovieInfoClient,
};

#[derive(Serialize)]
struct Query {
    q: String,
}

impl Query {
    pub fn new(q: String) -> Self {
        Self { q }
    }
}

impl MovieInfoClient {
    async fn force_search(&self, query: String) -> Result<Vec<MovieInfo>, Error> {
        Ok(self
            .http
            .get("search")
            .query(&Query::new(query))
            .unwrap()
            .recv_json()
            .await?)
    }

    pub async fn search(&self, query: String, filters: &Filters) -> Result<Vec<MovieInfo>, Error> {
        let query = query.trim().to_lowercase();

        if query.is_empty() {
            return Ok(Vec::new());
        }

        let mut movies = match parse_imdb_id(&query) {
            Some(imdb_id) => {
                let movie = self.from_imdb(imdb_id).await?;

                if let Some(movie) = movie {
                    vec![movie]
                } else {
                    Vec::new()
                }
            }
            None => self.force_search(query).await?,
        };

        filters.filter(&mut movies);

        Ok(movies)
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
    async fn test_empty_search() {
        let movies = CLIENT
            .search("".to_string(), &Filters::default())
            .await
            .unwrap();

        assert!(movies.is_empty());
    }

    #[tokio::test]
    async fn test_spaces_search() {
        let movies = CLIENT
            .search("   ".to_string(), &Filters::default())
            .await
            .unwrap();

        assert!(movies.is_empty());
    }

    #[tokio::test]
    async fn test_search() {
        let movies = CLIENT
            .search("the matrix".to_string(), &Filters::default())
            .await
            .unwrap();

        assert!(!movies.is_empty());

        let movie = movies.first().unwrap();

        assert_eq!(movie.get_title(), "The Matrix");
        assert_eq!(movie.get_year(), &1999);
    }

    #[tokio::test]
    async fn test_imdb_filter() {
        let movies = CLIENT
            .search(
                "quantum".to_string(),
                &Filters::new(true, 0, Default::default()),
            )
            .await
            .unwrap();

        assert!(!movies.is_empty());
        assert!(movies.iter().all(|m| m.get_imdb_id().is_some()));
    }

    #[tokio::test]
    async fn test_min_minutes_filter() {
        let movies = CLIENT
            .search(
                "the matrix".to_string(),
                &Filters::new(false, 120, Default::default()),
            )
            .await
            .unwrap();

        assert!(!movies.is_empty());
        assert!(movies.iter().all(|m| m.get_runtime() >= &120));
    }

    #[tokio::test]
    async fn test_imdb_id_search() {
        let movies = CLIENT
            .search("tt0133093".to_string(), &Filters::default())
            .await
            .unwrap();

        assert!(!movies.is_empty());

        let movie = movies.first().unwrap();

        assert_eq!(movie.get_title(), "The Matrix");
        assert_eq!(movie.get_year(), &1999);
    }
}
