mod api;
mod background;
mod graphql;
mod models;
mod r#static;
mod utils;

use async_graphql::{EmptySubscription, Schema};
use graphql::{graphiql, graphql_query, graphql_request, Mutation, Query, SchemaType};
use log::error;
use models::config::get_config;
use models::context::{Context, ContextPointer};
use qbittorrent_api::QbittorrentClient;
use simplelog::{
    ColorChoice, ConfigBuilder as LogConfigBuilder, LevelFilter, TermLogger, TerminalMode,
};
use std::sync::Arc;
use std::{process, vec};
use torrent_search_client::TorrentClient;

#[rocket::launch]
async fn rocket() -> _ {
    TermLogger::init(
        LevelFilter::Debug,
        LogConfigBuilder::new()
            .add_filter_ignore_str("isahc")
            .add_filter_ignore_str("tracing")
            .add_filter_ignore_str("hyper")
            .add_filter_ignore_str("selectors")
            .add_filter_ignore_str("html5ever")
            .add_filter_ignore_str("rocket::server")
            .add_filter_ignore_str("handlebars")
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

    let context: ContextPointer = Arc::new(Context::new(
        TorrentClient::new(),
        QbittorrentClient::new(
            config.qbittorrent().username(),
            config.qbittorrent().password(),
            config.qbittorrent().url().as_str(),
        ),
        config,
    ));

    tokio::spawn(background::background(Arc::clone(&context)));

    let schema: SchemaType =
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(context)
            .finish();

    rocket::build().manage(schema).mount(
        "/",
        rocket::routes![graphql_query, graphql_request, graphiql],
    )
}
