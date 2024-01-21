use crate::{
    add_torrent_options::ApiAddTorrentOptions,
    context::ContextPointer,
    http_error::HttpErrorKind,
    search_handler::{search_handler, SearchHandlerParams},
    torrent::ApiTorrent,
    utils::track_movie::track_movie,
};
use juniper::{graphql_object, EmptySubscription, RootNode};
use juniper_rocket::graphiql_source;
use movie_info::{Filters, MovieInfo};
use qbittorrent_api::{GetTorrentsParameters, Torrent};
use rocket::{response::content::RawHtml, State};

// impl juniper::Context for Context {}
pub struct Query;
#[graphql_object(context = ContextPointer)]
impl Query {
    async fn search(
        #[graphql(context)] context: &ContextPointer,
        params: SearchHandlerParams,
    ) -> Result<Vec<ApiTorrent>, HttpErrorKind> {
        let ctx = context.lock().await;
        let torrents =
            search_handler(params, ctx.torrent_client(), ctx.movie_info_client()).await?;
        Ok(torrents)
    }

    async fn torrents(
        #[graphql(context)] context: &ContextPointer,
        params: GetTorrentsParameters,
    ) -> Result<Vec<Torrent>, HttpErrorKind> {
        let torrents = context
            .lock()
            .await
            .qbittorrent_client()
            .torrents(params)
            .await?;

        Ok(torrents)
    }

    async fn movie_info(
        #[graphql(context)] context: &ContextPointer,
        tmdb: i32,
    ) -> Result<Option<MovieInfo>, HttpErrorKind> {
        if tmdb.is_negative() {
            return Err(HttpErrorKind::InvalidParam("tmdb".into()));
        };

        let movie_info = context
            .lock()
            .await
            .movie_info_client()
            .from_tmdb(tmdb as u32)
            .await?;

        Ok(movie_info)
    }

    async fn search_movies(
        #[graphql(context)] context: &ContextPointer,
        query: String,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = context.lock().await;
        let config = ctx.config();
        let movie_info = ctx
            .movie_info_client()
            .search(
                query,
                Filters::new(
                    *config.hide_movies_no_imdb(),
                    *config.hide_movies_below_runtime(),
                ),
            )
            .await?;

        Ok(movie_info)
    }
}

pub struct Mutation;
#[graphql_object(context = ContextPointer)]
impl Mutation {
    async fn add_torrent(
        #[graphql(context)] context: &ContextPointer,
        url: String,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        context
            .lock()
            .await
            .qbittorrent_client()
            .add_torrent(url, options.unwrap_or_default().into())
            .await?;
        Ok("Ok".into())
    }

    async fn add_torrents(
        #[graphql(context)] context: &ContextPointer,
        urls: Vec<String>,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        context
            .lock()
            .await
            .qbittorrent_client()
            .add_torrents(&urls, options.unwrap_or_default().into())
            .await?;
        Ok("Ok".into())
    }

    async fn track_movie(
        #[graphql(context)] context: &ContextPointer,
        url: String,
        tmdb: i32,
    ) -> Result<String, HttpErrorKind> {
        if tmdb.is_negative() {
            return Err(HttpErrorKind::InvalidParam("tmdb".into()));
        };

        track_movie(context, url, tmdb as u32).await?;
        Ok("Ok".into())
    }

    async fn delete_torrents(
        #[graphql(context)] context: &ContextPointer,
        hashes: Vec<String>,
        delete_files: bool,
    ) -> Result<String, HttpErrorKind> {
        context
            .lock()
            .await
            .qbittorrent_client()
            .delete_torrents(hashes, delete_files)
            .await?;

        Ok("Ok".into())
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<ContextPointer>>;

#[rocket::get("/")]
pub fn graphiql() -> RawHtml<String> {
    graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    context: &State<ContextPointer>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    context: &State<ContextPointer>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}
