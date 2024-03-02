mod add_torrent_options;
mod background;
mod config;
mod context;
mod graphql;
mod http_error;
mod search_handler;
mod r#static;
mod torrent;
mod utils;

use config::get_config;
use context::{Context, ContextPointer};
use graphql::{get_graphql_handler, graphiql, post_graphql_handler, Mutation, Query, Schema};
use juniper::EmptySubscription;
use qbittorrent_api::QbittorrentClient;
use simplelog::{
    ColorChoice, ConfigBuilder as LogConfigBuilder, LevelFilter, TermLogger, TerminalMode,
};
use std::sync::Arc;
use std::{process, vec};
use tokio::sync::Mutex;
use torrent_search_client::TorrentClient;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    TermLogger::init(
        LevelFilter::Debug,
        LogConfigBuilder::new()
            .add_filter_ignore_str("isahc")
            .add_filter_ignore_str("tracing")
            .add_filter_ignore_str("hyper")
            .add_filter_ignore_str("selectors")
            .add_filter_ignore_str("html5ever")
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let config = get_config().unwrap_or_else(|e| {
        error!("Error missing required config");
        error!("{}", e);
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
            routes![graphiql, get_graphql_handler, post_graphql_handler,],
        )
}
