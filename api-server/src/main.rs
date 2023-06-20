mod parse_category;
use std::vec;

use parse_category::parse_category;
use rocket::{response::status::BadRequest, serde::json::Json, State};
use torrent_search_client::{Category, SearchOptions, Torrent, TorrentClient};

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug)]
struct SearchParams {
    query: String,
    category: Option<String>,
    sort: Option<String>,
    order: Option<String>,
}

#[get("/search?<search_params..>")]
async fn search(
    search_params: SearchParams,
    client: &State<TorrentClient>,
) -> Result<Json<Vec<Torrent>>, BadRequest<()>> {
    let category = search_params
        .category
        .map_or_else(|| Some(Category::All), |c| parse_category(&c));

    let category = match category {
        Some(category) => category,
        None => return Err(BadRequest(None)),
    };

    let options = SearchOptions::new(
        search_params.query,
        category,
        search_params.sort.and_then(|s| s.parse().ok()),
        search_params.order.and_then(|s| s.parse().ok()),
    );

    let response = client.search_all(&options).await;

    let mut torrents: Vec<Torrent> = vec![];

    for result in response {
        match result {
            Ok(mut torrent) => torrents.append(&mut torrent),
            Err(err) => eprintln!("Error:\n{:?}", err),
        }
    }

    Ok(Json(torrents))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TorrentClient::new())
        .mount("/", routes![search])
}
