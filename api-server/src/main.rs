use rocket::serde::json::Json;
use torrent_search_client::{Category, Torrent, TorrentClient};
#[macro_use]
extern crate rocket;

#[get("/search?<query>")]
async fn search(query: String) -> Json<Vec<Torrent>> {
    let client = TorrentClient::new();
    let response = client.search(query, Category::All);
    Json(response.await)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![search])
}
