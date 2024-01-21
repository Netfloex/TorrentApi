use juniper::GraphQLInputObject;
use movie_info::MovieInfoClient;
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
    pub category: Option<Category>,
    pub sort: Option<SortColumn>,
    pub order: Option<Order>,
    pub limit: Option<i32>,
    pub quality: Option<Vec<Quality>>,
    pub codec: Option<Vec<VideoCodec>>,
    pub source: Option<Vec<Source>>,
}

pub async fn search_handler(
    search_params: SearchHandlerParams,
    client: &TorrentClient,
    movie_info_client: &MovieInfoClient,
) -> Result<Vec<ApiTorrent>, HttpErrorKind> {
    let sort = search_params.sort.unwrap_or_default();
    let category = search_params.category.unwrap_or_default();
    let order = search_params.order.unwrap_or_default();

    let response = if let Some(query) = search_params.query {
        let options = SearchOptions::new(query, category, sort.to_owned(), order.to_owned());
        client.search_all(&options).await
    } else if let Some(imdb) = search_params.imdb {
        let movie_info = movie_info_client.from_imdb(&imdb).await?;

        if let Some(movie_info) = movie_info {
            let options = MovieOptions::new(
                imdb,
                Some(movie_info.format()),
                sort.to_owned(),
                order.to_owned(),
            );

            client.search_movie_all(&options).await
        } else {
            return Err(HttpErrorKind::MovieInfoError("Movie not found".to_owned()));
        }
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
            Err(err) => error!("Error:\n{:?}", err),
        }
    }

    let mut torrents: Vec<ApiTorrent> = grouped.into_values().collect();

    torrents.retain(|torrent| {
        if let Some(props) = torrent.movie_properties() {
            if let Some(source) = &search_params.source {
                if source.is_empty() {
                } else if !source.contains(props.source()) {
                    return false;
                }
            };
            if let Some(codec) = &search_params.codec {
                if codec.is_empty() {
                } else if !codec.contains(props.codec()) {
                    return false;
                }
            };
            if let Some(quality) = &search_params.quality {
                if quality.is_empty() {
                } else if !quality.contains(props.quality()) {
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
        SortColumn::Size => a.size().cmp(b.size()),
    });

    if order == Order::Descending {
        torrents.reverse();
    }

    if let Some(limit) = search_params.limit {
        torrents.truncate(limit as usize);
    }

    Ok(torrents)
}
