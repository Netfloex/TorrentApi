use crate::{models::movie_info::MovieInfo, Error, MovieInfoClient};

impl MovieInfoClient {
    pub async fn trending(&self) -> Result<Vec<MovieInfo>, Error> {
        let mut resp = self.http.get("list/tmdb/trending").send().await?;

        Ok(resp.body_json().await?)
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
    async fn test_trending() {
        let trending = CLIENT.trending().await.unwrap();
        assert!(!trending.is_empty());
    }
}
