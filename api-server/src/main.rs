mod http_error;

use rocket::{serde::json::Json, State};
use std::vec;
use torrent_search_client::{Category, Order, SearchOptions, SortColumn, Torrent, TorrentClient};

use crate::http_error::HttpErrorKind;

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug)]
struct SearchParams {
    query: String,
    category: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    limit: Option<usize>,
}

#[get("/search?<search_params..>")]
async fn search(
    search_params: SearchParams,
    client: &State<TorrentClient>,
) -> Result<Json<Vec<Torrent>>, HttpErrorKind> {
    let category: Category = search_params
        .category
        .map_or_else(|| Ok(Category::default()), |c| c.parse())?;

    let sort: SortColumn = search_params
        .sort
        .map_or_else(|| Ok(SortColumn::default()), |f| f.parse())?;

    let order: Order = search_params
        .order
        .map_or_else(|| Ok(Order::default()), |f| f.parse())?;

    let options = SearchOptions::new(search_params.query, category, sort, order);

    let response = client.search_all(&options).await;

    let mut torrents: Vec<Torrent> = Vec::new();

    for result in response {
        match result {
            Ok(mut torrent) => torrents.append(&mut torrent),
            Err(err) => eprintln!("Error:\n{:?}", err),
        }
    }

    torrents.sort_unstable_by(|a, b| match options.sort() {
        SortColumn::Added => a.added().cmp(b.added()),
        SortColumn::Leechers => a.leechers().cmp(b.leechers()),
        SortColumn::Seeders => a.seeders().cmp(b.seeders()),
        SortColumn::Size => a.size().cmp(b.size()),
    });

    if matches!(options.order(), Order::Descending) {
        torrents.reverse();
    }

    if let Some(limit) = search_params.limit {
        torrents.truncate(limit);
    }

    Ok(Json(torrents))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TorrentClient::new())
        .mount("/", routes![search])
}
