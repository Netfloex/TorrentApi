mod add_torrent_options;
mod background;
mod config;
mod context;
mod graphql;
mod http_error;
mod search_handler;
mod search_params;
mod r#static;
mod torrent;
mod utils;

use crate::http_error::HttpErrorKind;
use config::get_config;
use context::{Context, ContextPointer};
use graphql::{get_graphql_handler, graphiql, post_graphql_handler, Mutation, Query, Schema};
use juniper::EmptySubscription;
use qbittorrent_api::QbittorrentClient;
use rocket::{serde::json::Json, State};
use search_handler::{search_handler, SearchHandlerParams};
use search_params::SearchParams;
use std::sync::Arc;
use std::{process, vec};
use tokio::sync::Mutex;
use torrent::ApiTorrent;
use torrent_search_client::{
    Category, Order, Quality, SortColumn, Source, TorrentClient, VideoCodec,
};

#[macro_use]
extern crate rocket;

#[get("/search?<search_params..>")]
async fn search(
    search_params: SearchParams,
    context: &State<ContextPointer>,
) -> Result<Json<Vec<ApiTorrent>>, HttpErrorKind> {
    let category: Category = search_params
        .category()
        .as_ref()
        .map_or_else(|| Ok(Category::default()), |c| c.parse())?;

    let sort: SortColumn = search_params
        .sort()
        .as_ref()
        .map_or_else(|| Ok(SortColumn::default()), |f| f.parse())?;

    let order: Order = search_params
        .order()
        .as_ref()
        .map_or_else(|| Ok(Order::default()), |f| f.parse())?;

    let torrents = search_handler(
        SearchHandlerParams {
            query: search_params.query().clone(),
            imdb: search_params.imdb().clone(),
            title: search_params.title().clone(),
            category: Some(category),
            sort: Some(sort),
            order: Some(order),
            limit: search_params.limit().clone(),
            quality: search_params
                .quality()
                .as_ref()
                .map(|q| q.into_iter().map(|q| Quality::from_str(q)).collect()),
            codec: search_params
                .codec()
                .as_ref()
                .map(|c| c.into_iter().map(|c| VideoCodec::from_str(c)).collect()),
            source: search_params
                .source()
                .as_ref()
                .map(|s| s.into_iter().map(|s| Source::from_str(s)).collect()),
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
        config,
    )));

    tokio::spawn(background::background(context.clone()));

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
