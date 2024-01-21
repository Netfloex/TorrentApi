use serde::Serialize;

use crate::{
    models::{filters::Filters, movie_info::MovieInfo},
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
    pub async fn search(&self, query: String, filters: Filters) -> Result<Vec<MovieInfo>, Error> {
        if query.is_empty() {
            return Ok(Vec::new());
        }

        let mut movies: Vec<MovieInfo> = self
            .http
            .get("search")
            .query(&Query::new(query))
            .unwrap()
            .recv_json()
            .await?;

        if *filters.imdb() {
            movies.retain(|m| m.imdb_id().is_some())
        }

        if *filters.min_minutes() > 0 {
            movies.retain(|m| m.runtime() >= &(*filters.min_minutes() as i32))
        }

        Ok(movies)
    }
}
