use serde::Serialize;

use crate::{models::movie_info::MovieInfo, Error, MovieInfoClient};

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
    pub async fn search(&self, query: String) -> Result<Vec<MovieInfo>, Error> {
        if query.is_empty() {
            return Ok(Vec::new());
        }
        let movie: Vec<MovieInfo> = self
            .http
            .get("search")
            .query(&Query::new(query))
            .unwrap()
            .recv_json()
            .await?;

        Ok(movie)
    }
}
