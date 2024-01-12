mod add_torrent_options;
mod config;
mod graphql;
mod http_error;
mod search_handler;
mod torrent;

use config::get_config;
use graphql::{
    get_graphql_handler, graphiql, post_graphql_handler, Context, Mutation, Query, Schema,
};
use juniper::{EmptySubscription, GraphQLInputObject};
use qbittorrent_api::QbittorrentClient;
use rocket::form::{self, Error};
use rocket::{serde::json::Json, State};
use search_handler::{search_handler, SearchHandlerParams};
use std::sync::Arc;
use std::{process, vec};
use tokio::sync::Mutex;
use torrent::ApiTorrent;
use torrent_search_client::{Category, Order, Quality, SortColumn, TorrentClient};

use crate::graphql::ContextPointer;
use crate::http_error::HttpErrorKind;

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug, GraphQLInputObject)]
pub struct SearchParams {
    query: Option<String>,
    #[field(validate = or(&self.query))]
    imdb: Option<String>,
    #[field(validate = or(&self.query))]
    title: Option<String>,
    #[field(validate = or(&self.imdb))]
    category: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    limit: Option<i32>,
    quality: Option<Vec<String>>,
    codec: Option<Vec<String>>,
    source: Option<Vec<String>>,
}

fn or<'v>(first: &Option<String>, second: &Option<String>) -> form::Result<'v, ()> {
    match (first, second) {
        (Some(_), Some(_)) => Err(Error::validation("Not both"))?,
        _ => Ok(()),
    }
}

#[get("/search?<search_params..>")]
async fn search(
    search_params: SearchParams,
    context: &State<ContextPointer>,
) -> Result<Json<Vec<ApiTorrent>>, HttpErrorKind> {
    let category: Category = search_params
        .category
        .as_ref()
        .map_or_else(|| Ok(Category::default()), |c| c.parse())?;

    let sort: SortColumn = search_params
        .sort
        .as_ref()
        .map_or_else(|| Ok(SortColumn::default()), |f| f.parse())?;

    let order: Order = search_params
        .order
        .as_ref()
        .map_or_else(|| Ok(Order::default()), |f| f.parse())?;

    let torrents = search_handler(
        SearchHandlerParams {
            query: search_params.query,
            imdb: search_params.imdb,
            title: search_params.title,
            category: Some(category),
            sort: Some(sort),
            order: Some(order),
            limit: search_params.limit,
            quality: Some(
                search_params
                    .quality
                    .unwrap_or_default()
                    .into_iter()
                    .map(|q| q.parse::<Quality>().expect("Can not give error"))
                    .collect(),
            ),
            codec: Some(
                search_params
                    .codec
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| c.parse().expect("Can not give error"))
                    .collect(),
            ),
            source: Some(
                search_params
                    .source
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.parse().expect("Can not give error"))
                    .collect(),
            ),
        },
        context.lock().await.torrent_client(),
    )
    .await?;
    Ok(Json(torrents))
}

#[launch]
async fn rocket() -> _ {
    let config = get_config().unwrap_or_else(|e| {
        println!("Error missing required config");
        println!("{}", e);
        process::exit(1);
    });

    let context: ContextPointer = Arc::new(Mutex::new(Context::new(
        TorrentClient::new(),
        QbittorrentClient::new(
            config.qbittorrent().username(),
            config.qbittorrent().password(),
            config.qbittorrent().url().as_str(),
        ),
    )));

    rocket::build()
        .manage(context)
        .manage(Schema::new(
            Query,
            Mutation,
            EmptySubscription::<ContextPointer>::new(),
        ))
        .mount(
            "/",
            routes![search, graphiql, get_graphql_handler, post_graphql_handler,],
        )
}
