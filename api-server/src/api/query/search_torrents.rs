use super::super::get_context;
use crate::models::http_error::HttpErrorKind;
use crate::models::provider_error::ProviderError;
use crate::models::search_torrents_parameters::SearchTorrentsParameters;
use async_graphql::SimpleObject;
use async_graphql::{Context, Object};
use log::error;
use serde::Serialize;
use std::collections::HashMap;
use torrent_search_client::{MovieOptions, Order, SearchOptions, SortColumn, Torrent};

#[derive(Default)]
pub struct SearchTorrentsQuery;

#[derive(SimpleObject, Serialize)]
pub struct SearchHandlerResponse {
    torrents: Vec<Torrent>,
    errors: Vec<ProviderError>,
}

#[Object]
impl SearchTorrentsQuery {
    async fn search_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        params: SearchTorrentsParameters,
    ) -> Result<SearchHandlerResponse, HttpErrorKind> {
        let ctx = get_context(context);

        let response = if let Some(query) = params.query() {
            let options = SearchOptions::new(
                query.to_owned(),
                params.category().to_owned(),
                params.sort().to_owned(),
                params.order().to_owned(),
            );

            ctx.torrent_client()
                .search(&options, params.providers())
                .await
        } else if let Some(imdb) = params.imdb().to_owned() {
            let movie_info = ctx.movie_info_client().from_imdb(&imdb).await?;

            if let Some(movie_info) = movie_info {
                let options = MovieOptions::new(
                    imdb,
                    Some(movie_info.format()),
                    params.sort().to_owned(),
                    params.order().to_owned(),
                );

                ctx.torrent_client()
                    .search_movie(&options, params.providers())
                    .await
            } else {
                return Err(HttpErrorKind::imdb_not_found(imdb));
            }
        } else {
            return Err(HttpErrorKind::missing_query());
        };

        let mut grouped: HashMap<String, Torrent> = HashMap::new();
        let mut errors: Vec<ProviderError> = Vec::new();

        for result in response {
            match result.torrents {
                Ok(provider_torrents) => {
                    for torrent in provider_torrents {
                        grouped
                            .entry(torrent.info_hash.to_owned())
                            .and_modify(|existing| existing.merge(torrent.to_owned()))
                            .or_insert(torrent);
                    }
                }
                Err(err) => {
                    error!("Error:\n{err:?}");
                    errors.push(ProviderError::new(
                        result.provider,
                        format!("{:?}: {}", err.kind(), err),
                    ));
                }
            }
        }

        let mut torrents: Vec<Torrent> = grouped.into_values().collect();

        torrents.retain(|torrent| {
            if let Some(props) = &torrent.movie_properties {
                if !params.source().is_empty() && !params.source().contains(props.get_source()) {
                    return false;
                }
                if !params.codec().is_empty() && !params.codec().contains(props.get_codec()) {
                    return false;
                }
                if !params.quality().is_empty() && !params.quality().contains(props.get_quality()) {
                    return false;
                }

                return true;
            }

            false
        });

        torrents.sort_unstable_by(|a, b| match params.sort() {
            SortColumn::Added => a.added.cmp(&b.added),
            SortColumn::Leechers => a.leechers.cmp(&b.leechers),
            SortColumn::Seeders => a.seeders.cmp(&b.seeders),
            SortColumn::Size => a.size.cmp(&b.size),
        });

        if params.order() == &Order::Descending {
            torrents.reverse();
        }

        if params.limit() != &0 {
            torrents.truncate(*params.limit());
        }

        Ok(SearchHandlerResponse { torrents, errors })
    }
}
