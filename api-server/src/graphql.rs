use crate::{
    add_torrent_options::ApiAddTorrentOptions,
    http_error::HttpErrorKind,
    search_handler::{search_handler, SearchHandlerParams},
    torrent::ApiTorrent,
};
use derive_getters::Getters;
use juniper::{graphql_object, EmptySubscription, RootNode};
use juniper_rocket::graphiql_source;
use qbittorrent_api::QbittorrentClient;
use rocket::{response::content::RawHtml, State};
use torrent_search_client::TorrentClient;

#[derive(Getters)]
pub struct Context {
    torrent_client: TorrentClient,
    qbittorrent_client: QbittorrentClient,
}

impl Context {
    pub fn new(torrent_client: TorrentClient, qbittorrent_client: QbittorrentClient) -> Self {
        Self {
            torrent_client,
            qbittorrent_client,
        }
    }
}
// impl juniper::Context for Context {}
pub struct Query;
#[graphql_object(context = Context)]
impl Query {
    async fn search(
        #[graphql(context)] context: &Context,
        params: SearchHandlerParams,
    ) -> Result<Vec<ApiTorrent>, HttpErrorKind> {
        let torrents = search_handler(params, context.torrent_client()).await?;
        Ok(torrents)
    }
}

pub struct Mutation;
#[graphql_object(context = Context)]
impl Mutation {
    async fn add_torrent(
        #[graphql(context)] context: &Context,
        url: String,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        context
            .qbittorrent_client()
            .add_torrent(url, options.unwrap_or_default().into())
            .await?;
        Ok("Ok".into())
    }

    async fn add_torrents(
        #[graphql(context)] context: &Context,
        urls: Vec<String>,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        context
            .qbittorrent_client()
            .add_torrents(&urls, options.unwrap_or_default().into())
            .await?;
        Ok("Ok".into())
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

#[rocket::get("/")]
pub fn graphiql() -> RawHtml<String> {
    graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}
