use crate::{models::movie_info::MovieInfo, Error, MovieInfoClient};

impl MovieInfoClient {
    pub async fn popular(&self) -> Result<Vec<MovieInfo>, Error> {
        let mut resp = self.http.get("list/tmdb/popular").send().await?;

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
    async fn test_popular() {
        let popular = CLIENT.popular().await.unwrap();
        assert!(!popular.is_empty());
    }
}
