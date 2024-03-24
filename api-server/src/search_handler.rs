use async_graphql::{InputObject, SimpleObject};
use movie_info::MovieInfoClient;
use serde::Serialize;
use std::collections::HashMap;
use torrent_search_client::{
    Category, Codec, MovieOptions, Order, Provider, Quality, SearchOptions, SortColumn, Source,
    TorrentClient,
};

use crate::{http_error::HttpErrorKind, torrent::ApiTorrent};

#[derive(InputObject)]
pub struct SearchHandlerParams {
    query: Option<String>,
    imdb: Option<String>,
    category: Option<Category>,
    sort: Option<SortColumn>,
    order: Option<Order>,
    limit: Option<i32>,
    quality: Option<Vec<Quality>>,
    codec: Option<Vec<Codec>>,
    source: Option<Vec<Source>>,
    providers: Option<Vec<Provider>>,
}

#[derive(SimpleObject, Serialize)]
pub struct ProviderError {
    provider: Provider,
    error: String,
}

#[derive(SimpleObject, Serialize)]
pub struct SearchHandlerResponse {
    torrents: Vec<ApiTorrent>,
    errors: Vec<ProviderError>,
}

pub async fn search_handler(
    search_params: SearchHandlerParams,
    client: &TorrentClient,
    movie_info_client: &MovieInfoClient,
) -> Result<SearchHandlerResponse, HttpErrorKind> {
    let sort = search_params.sort.unwrap_or_default();
    let category = search_params.category.unwrap_or_default();
    let order = search_params.order.unwrap_or_default();

    let response = if let Some(query) = search_params.query {
        let options = SearchOptions::new(query, category, sort.to_owned(), order.to_owned());

        client
            .search(
                &options,
                search_params
                    .providers
                    .unwrap_or_default()
                    .into_iter()
                    .collect(),
            )
            .await
    } else if let Some(imdb) = search_params.imdb {
        let movie_info = movie_info_client.from_imdb(&imdb).await?;

        if let Some(movie_info) = movie_info {
            let options = MovieOptions::new(
                imdb,
                Some(movie_info.format()),
                sort.to_owned(),
                order.to_owned(),
            );

            client
                .search_movie(
                    &options,
                    search_params
                        .providers
                        .unwrap_or_default()
                        .into_iter()
                        .collect(),
                )
                .await
        } else {
            return Err(HttpErrorKind::imdb_not_found(imdb));
        }
    } else {
        return Err(HttpErrorKind::missing_query());
    };

    let mut grouped: HashMap<String, ApiTorrent> = HashMap::new();
    let mut errors: Vec<ProviderError> = Vec::new();

    for result in response {
        match result.torrents {
            Ok(provider_torrents) => {
                for torrent in provider_torrents {
                    let torrent: ApiTorrent = torrent.into();
                    grouped
                        .entry(torrent.get_info_hash().to_string())
                        .and_modify(|existing| existing.merge(torrent.to_owned()))
                        .or_insert(torrent);
                }
            }
            Err(err) => {
                error!("Error:\n{:?}", err);
                errors.push(ProviderError {
                    provider: result.provider,
                    error: format!("{:?}: {}", err.kind(), err),
                });
            }
        }
    }

    let mut torrents: Vec<ApiTorrent> = grouped.into_values().collect();

    torrents.retain(|torrent| {
        if let Some(props) = torrent.get_movie_properties() {
            if let Some(source) = &search_params.source {
                if source.is_empty() {
                } else if !source.contains(props.get_source()) {
                    return false;
                }
            };
            if let Some(codec) = &search_params.codec {
                if codec.is_empty() {
                } else if !codec.contains(props.get_codec()) {
                    return false;
                }
            };
            if let Some(quality) = &search_params.quality {
                if quality.is_empty() {
                } else if !quality.contains(props.get_quality()) {
                    return false;
                }
            };

            return true;
        }

        false
    });

    torrents.sort_unstable_by(|a, b| match sort {
        SortColumn::Added => a.get_added().cmp(b.get_added()),
        SortColumn::Leechers => a.get_leechers().cmp(b.get_leechers()),
        SortColumn::Seeders => a.get_seeders().cmp(b.get_seeders()),
        SortColumn::Size => a.get_size().cmp(b.get_size()),
    });

    if order == Order::Descending {
        torrents.reverse();
    }

    if let Some(limit) = search_params.limit {
        torrents.truncate(limit as usize);
    }

    Ok(SearchHandlerResponse { torrents, errors })
}
