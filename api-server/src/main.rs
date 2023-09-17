mod graphql;
mod http_error;
mod search_handler;

use graphql::{get_graphql_handler, graphiql, post_graphql_handler, Query, Schema};
use juniper::{EmptyMutation, EmptySubscription, GraphQLInputObject};
use rocket::form::{self, Error};
use rocket::{serde::json::Json, State};
use search_handler::{search_handler, SearchHandlerParams};
use std::vec;
use torrent_search_client::{Category, Order, SortColumn, Torrent, TorrentClient};

use crate::http_error::HttpErrorKind;

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug, GraphQLInputObject)]
pub struct SearchParams {
    query: Option<String>,
    #[field(validate= or(&self.query))]
    imdb: Option<String>,
    #[field(validate= or(&self.query))]
    title: Option<String>,
    #[field(validate = or(&self.imdb))]
    category: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    limit: Option<i32>,
    quality: Option<String>,
    codec: Option<String>,
    source: Option<String>,
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
    client: &State<TorrentClient>,
) -> Result<Json<Vec<Torrent>>, HttpErrorKind> {
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
                    .parse()
                    .expect("Can not give error"),
            ),
            codec: Some(
                search_params
                    .codec
                    .unwrap_or_default()
                    .parse()
                    .expect("Can not give error"),
            ),
            source: Some(
                search_params
                    .source
                    .unwrap_or_default()
                    .parse()
                    .expect("Can not give error"),
            ),
        },
        client,
    )
    .await?;
    Ok(Json(torrents))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TorrentClient::new())
        .manage(Schema::new(
            Query,
            EmptyMutation::<TorrentClient>::new(),
            EmptySubscription::<TorrentClient>::new(),
        ))
        .mount(
            "/",
            routes![search, graphiql, get_graphql_handler, post_graphql_handler],
        )
}
