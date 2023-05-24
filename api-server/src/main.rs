mod category_form_field;
use category_form_field::CategoryFormField;
use rocket::serde::json::Json;
use torrent_search_client::{Torrent, TorrentClient};

#[macro_use]
extern crate rocket;

#[get("/search?<query>&<category>")]
async fn search(query: String, category: CategoryFormField) -> Json<Vec<Torrent>> {
    let client = TorrentClient::new();

    let response = client.search(query, category.get());
    Json(response.await)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![search])
}
