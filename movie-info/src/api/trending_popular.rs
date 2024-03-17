use crate::{models::movie_info::MovieInfo, Error, Filters, MovieInfoClient};

impl MovieInfoClient {
    async fn get(&self, path: &str, filters: Filters) -> Result<Vec<MovieInfo>, Error> {
        let mut movies: Vec<MovieInfo> = self
            .http
            .get(format!("list/tmdb/{}", path))
            .recv_json()
            .await?;

        filters.filter(&mut movies);

        Ok(movies)
    }

    pub async fn trending(&self, filters: Filters) -> Result<Vec<MovieInfo>, Error> {
        self.get("trending", filters).await
    }

    pub async fn popular(&self, filters: Filters) -> Result<Vec<MovieInfo>, Error> {
        self.get("popular", filters).await
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
        let trending = CLIENT.trending(Filters::default()).await.unwrap();
        assert!(!trending.is_empty());
    }

    #[tokio::test]
    async fn test_popular() {
        let popular = CLIENT.popular(Filters::default()).await.unwrap();
        assert!(!popular.is_empty());
    }
}
