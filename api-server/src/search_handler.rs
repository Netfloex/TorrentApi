use juniper::{GraphQLInputObject, GraphQLObject};
use movie_info::MovieInfoClient;
use serde::Serialize;
use std::collections::HashMap;
use torrent_search_client::{
    Category, Codec, MovieOptions, Order, Provider, Quality, SearchOptions, SortColumn, Source,
    TorrentClient,
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
    pub codec: Option<Vec<Codec>>,
    pub source: Option<Vec<Source>>,
}

#[derive(GraphQLObject, Serialize)]
pub struct ProviderError {
    provider: Provider,
    error: String,
}

#[derive(GraphQLObject, Serialize)]
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
                        .entry(torrent.info_hash().to_string())
                        .and_modify(|existing| existing.merge(torrent.clone()))
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

    Ok(SearchHandlerResponse { torrents, errors })
}
