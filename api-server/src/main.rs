mod parse_category;
use std::vec;

use parse_category::parse_category;
use rocket::{response::status::BadRequest, serde::json::Json, State};
use torrent_search_client::{Category, Torrent, TorrentClient};

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct SearchParams {
    query: String,
    category: Option<String>,
}

#[get("/search?<search_params..>")]
async fn search(
    search_params: SearchParams,
    client: &State<TorrentClient>,
) -> Result<Json<Vec<Torrent>>, BadRequest<()>> {
    let category = search_params
        .category
        .map_or_else(|| Some(Category::All), |c| parse_category(&c));

    let category = match &category {
        Some(category) => category,
        None => return Err(BadRequest(None)),
    };

    let response = client.search_all(search_params.query, category).await;

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
