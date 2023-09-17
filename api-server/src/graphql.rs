use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use juniper_rocket::graphiql_source;
use rocket::{response::content::RawHtml, State};
use torrent_search_client::{Torrent, TorrentClient};

use crate::{
    http_error::HttpErrorKind,
    search_handler::{search_handler, SearchHandlerParams},
};
pub struct Query;
#[graphql_object(context = TorrentClient)]
impl Query {
    async fn search(
        #[graphql(context)] client: &TorrentClient,
        params: SearchHandlerParams,
    ) -> Result<Vec<Torrent>, HttpErrorKind> {
        let torrents = search_handler(params, client).await?;
        Ok(torrents)
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<TorrentClient>, EmptySubscription<TorrentClient>>;

#[rocket::get("/")]
pub fn graphiql() -> RawHtml<String> {
    graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    context: &State<TorrentClient>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    context: &State<TorrentClient>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}
