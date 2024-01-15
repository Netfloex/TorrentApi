use crate::{
    add_torrent_options::ApiAddTorrentOptions,
    config::Config,
    http_error::HttpErrorKind,
    search_handler::{search_handler, SearchHandlerParams},
    torrent::ApiTorrent,
    utils::track_movie::track_movie,
};
use derive_getters::Getters;
use juniper::{graphql_object, EmptySubscription, RootNode};
use juniper_rocket::graphiql_source;
use movie_info::MovieInfoClient;
use qbittorrent_api::{GetTorrentsParameters, QbittorrentClient, Torrent};
use rocket::{response::content::RawHtml, State};
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use torrent_search_client::TorrentClient;

#[derive(Getters)]
pub struct Context {
    torrent_client: TorrentClient,
    qbittorrent_client: QbittorrentClient,
    movie_info_client: MovieInfoClient,
    config: Config,
    movie_tracking_enabled: Mutex<bool>,
    movie_tracking_ntfy: Arc<Notify>,
}

impl Context {
    pub fn new(
        torrent_client: TorrentClient,
        qbittorrent_client: QbittorrentClient,
        config: Config,
    ) -> Self {
        Self {
            torrent_client,
            qbittorrent_client,
            movie_info_client: MovieInfoClient::new(),
            config,
            movie_tracking_enabled: Mutex::new(true),
            movie_tracking_ntfy: Arc::new(Notify::new()),
        }
    }

    pub async fn enable_movie_tracking(&mut self) {
        if !self.movie_tracking_enabled.lock().await.to_owned() {
            println!("Enabling movie progress tracking");
            *self.movie_tracking_enabled.lock().await = true;
            self.movie_tracking_ntfy.notify_waiters();
        }
    }

    pub async fn disable_movie_tracking(&mut self) {
        if self.movie_tracking_enabled.lock().await.to_owned() {
            println!("Disabling movie progress tracking");
            *self.movie_tracking_enabled.lock().await = false;
            self.movie_tracking_ntfy.notify_waiters();
        }
    }

    pub fn qbittorrent_client_mut(&mut self) -> &mut QbittorrentClient {
        &mut self.qbittorrent_client
    }
}

pub type ContextPointer = Arc<Mutex<Context>>;
// impl juniper::Context for Context {}
pub struct Query;
#[graphql_object(context = ContextPointer)]
impl Query {
    async fn search(
        #[graphql(context)] context: &ContextPointer,
        params: SearchHandlerParams,
    ) -> Result<Vec<ApiTorrent>, HttpErrorKind> {
        let torrents = search_handler(params, context.lock().await.torrent_client()).await?;
        Ok(torrents)
    }

    async fn torrents(
        #[graphql(context)] context: &ContextPointer,
        params: GetTorrentsParameters,
    ) -> Result<Vec<Torrent>, HttpErrorKind> {
        let torrents = context
            .lock()
            .await
            .qbittorrent_client
            .torrents(params)
            .await?;

        Ok(torrents)
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
