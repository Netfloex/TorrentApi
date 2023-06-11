mod parse_category;
use parse_category::parse_category;
use rocket::{response::status::BadRequest, serde::json::Json};
use torrent_search_client::{Category, Torrent, TorrentClient};

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct SearchParams {
    query: String,
    category: Option<String>,
}

#[get("/search?<search_params..>")]
async fn search(search_params: SearchParams) -> Result<Json<Vec<Torrent>>, BadRequest<()>> {
    let client = TorrentClient::new();

    let category = search_params
        .category
        .map_or_else(|| Some(Category::All), |c| parse_category(&c));

    let category = match &category {
        Some(category) => category,
        None => return Err(BadRequest(None)),
    };

    let response = client.search(search_params.query, category).await;

    Ok(Json(response))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![search])
}
