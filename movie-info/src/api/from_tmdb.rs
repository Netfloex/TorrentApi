use crate::{models::movie_info::MovieInfo, Error, MovieInfoClient};
impl MovieInfoClient {
    pub async fn from_tmdb(&self, tmdb: u32) -> Result<MovieInfo, Error> {
        let movie: MovieInfo = self.http.get(format!("movie/{}", tmdb)).recv_json().await?;

        Ok(movie)
    }
}
