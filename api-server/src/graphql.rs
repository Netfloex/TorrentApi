use crate::{
    add_torrent_options::ApiAddTorrentOptions,
    config::Config,
    http_error::HttpErrorKind,
    search_handler::{search_handler, SearchHandlerParams},
    torrent::ApiTorrent,
};
use derive_getters::Getters;
use juniper::{graphql_object, EmptySubscription, RootNode};
use juniper_rocket::graphiql_source;
use qbittorrent_api::{GetTorrentsParameters, QbittorrentClient, Torrent};
use rocket::{response::content::RawHtml, State};
use std::sync::Arc;
use tokio::sync::Mutex;
use torrent_search_client::TorrentClient;

#[derive(Getters)]
pub struct Context {
    torrent_client: TorrentClient,
    qbittorrent_client: QbittorrentClient,
    config: Config,
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
            config,
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
