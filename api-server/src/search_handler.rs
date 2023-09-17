use juniper::GraphQLInputObject;
use std::collections::HashMap;
use torrent_search_client::{
    Category, MovieOptions, Order, Quality, SearchOptions, SortColumn, Source, TorrentClient,
    VideoCodec,
};

use crate::{http_error::HttpErrorKind, torrent::ApiTorrent};

#[derive(GraphQLInputObject)]
pub struct SearchHandlerParams {
    pub query: Option<String>,
    pub imdb: Option<String>,
    pub title: Option<String>,
    pub category: Option<Category>,
    pub sort: Option<SortColumn>,
    pub order: Option<Order>,
    pub limit: Option<i32>,
    pub quality: Option<Quality>,
    pub codec: Option<VideoCodec>,
    pub source: Option<Source>,
}

pub async fn search_handler(
    search_params: SearchHandlerParams,
    client: &TorrentClient,
) -> Result<Vec<ApiTorrent>, HttpErrorKind> {
    let sort = search_params.sort.unwrap_or_default();
    let category = search_params.category.unwrap_or_default();
    let order = search_params.order.unwrap_or_default();

    let response = if let Some(query) = search_params.query {
        let options = SearchOptions::new(query, category, sort.to_owned(), order.to_owned());
        client.search_all(&options).await
    } else if let Some(imdb) = search_params.imdb {
        let options =
            MovieOptions::new(imdb, search_params.title, sort.to_owned(), order.to_owned());
        client.search_movie_all(&options).await
    } else {
        return Err(HttpErrorKind::missing_query());
    };

    let mut grouped: HashMap<String, ApiTorrent> = HashMap::new();

    for result in response {
        match result {
            Ok(provider_torrents) => {
                for torrent in provider_torrents {
                    grouped
                        .entry(torrent.info_hash.to_string())
                        .and_modify(|existing| existing.merge(torrent.clone().into()))
                        .or_insert(torrent.into());
                }
            }
            Err(err) => eprintln!("Error:\n{:?}", err),
        }
    }

    let mut torrents: Vec<ApiTorrent> = grouped.into_values().collect();

    torrents.retain(|torrent| {
        if let Some(props) = torrent.movie_properties() {
            if let Some(source) = &search_params.source {
                if source == &Source::default() {
                } else if source != props.source() {
                    return false;
                }
            };
            if let Some(codec) = &search_params.codec {
                if codec == &VideoCodec::default() {
                } else if codec != props.codec() {
                    return false;
                }
            };
            if let Some(quality) = &search_params.quality {
                if quality == &Quality::default() {
                } else if quality != props.quality() {
                    return false;
                }
            };

            return true;
        }

        false
    });

    torrents.sort_unstable_by(|a, b| match sort {
        SortColumn::Added => a.added().cmp(b.added()),
        SortColumn::Leechers => a.leechers().cmp(b.leechers()),
        SortColumn::Seeders => a.seeders().cmp(b.seeders()),
        SortColumn::Size => a.size().get().cmp(b.size().get()),
    });

    if order == Order::Descending {
        torrents.reverse();
    }

    if let Some(limit) = search_params.limit {
        torrents.truncate(limit as usize);
    }

    Ok(torrents)
}
