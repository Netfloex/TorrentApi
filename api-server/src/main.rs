mod http_error;

use rocket::form::{self, Error};
use rocket::{serde::json::Json, State};
use std::collections::HashMap;
use std::vec;
use torrent_search_client::{
    Category, MovieOptions, Order, SearchOptions, SortColumn, Torrent, TorrentClient,
};

use crate::http_error::HttpErrorKind;

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug)]
struct SearchParams {
    query: Option<String>,
    #[field(validate= xor(&self.query))]
    imdb: Option<String>,
    #[field(validate= or(&self.query))]
    title: Option<String>,
    category: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    limit: Option<usize>,
}

fn xor<'v>(first: &Option<String>, second: &Option<String>) -> form::Result<'v, ()> {
    match (first, second) {
        (Some(_), Some(_)) => Err(Error::validation("Not both"))?,
        (None, None) => Err(Error::validation("Both none"))?,
        _ => Ok(()),
    }
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
    println!("{:?}", search_params);
    let category: Category = search_params
        .category
        .map_or_else(|| Ok(Category::default()), |c| c.parse())?;

    let sort: SortColumn = search_params
        .sort
        .map_or_else(|| Ok(SortColumn::default()), |f| f.parse())?;

    let order: Order = search_params
        .order
        .map_or_else(|| Ok(Order::default()), |f| f.parse())?;

    let response = if let Some(query) = search_params.query {
        let options = SearchOptions::new(query, category, sort.clone(), order.clone());
        client.search_all(&options).await
    } else if let Some(imdb) = search_params.imdb {
        let options = MovieOptions::new(imdb, search_params.title, sort.clone(), order.clone());
        client.search_movie_all(&options).await
    } else {
        unreachable!();
    };

    let mut grouped: HashMap<String, Torrent> = HashMap::new();

    for result in response {
        match result {
            Ok(provider_torrents) => {
                for torrent in provider_torrents {
                    grouped
                        .entry(torrent.info_hash.to_string())
                        .and_modify(|existing| existing.merge(torrent.clone()))
                        .or_insert(torrent);
                }
            }
            Err(err) => eprintln!("Error:\n{:?}", err),
        }
    }

    let mut torrents: Vec<Torrent> = grouped.into_iter().map(|(_, v)| v).collect();

    torrents.sort_unstable_by(|a, b| match sort {
        SortColumn::Added => a.added().cmp(b.added()),
        SortColumn::Leechers => a.leechers().cmp(b.leechers()),
        SortColumn::Seeders => a.seeders().cmp(b.seeders()),
        SortColumn::Size => a.size().cmp(b.size()),
    });

    if matches!(order, Order::Descending) {
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
