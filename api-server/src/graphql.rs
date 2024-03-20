use crate::{
    add_torrent_options::ApiAddTorrentOptions,
    config::Config,
    context::ContextPointer,
    filter::Filter,
    http_error::HttpErrorKind,
    search_handler::{search_handler, SearchHandlerParams, SearchHandlerResponse},
    utils::{get_tmdb::get_tmdb, track_movie::track_movie},
};
use juniper::{graphql_object, EmptySubscription, RootNode};
use juniper_rocket::graphiql_source;
use movie_info::{Filters, MovieInfo};
use qbittorrent_api::{GetTorrentsParameters, Torrent};
use rocket::{config, response::content::RawHtml, State};
use std::{collections::HashMap, hash::Hash};
use strum::IntoEnumIterator;
use torrent_search_client::{Codec, Quality, Source};

// impl juniper::Context for Context {}

#[derive(juniper::GraphQLObject, PartialEq, Eq, Hash)]
struct TorrentMovieInfo {
    title: String,
    year: i32,
    imdb: Option<String>,
    tmdb: i32,
    runtime: i32,

    for_torrents: Vec<String>,
}

#[derive(juniper::GraphQLObject)]
struct ActiveTorrentsResponse {
    torrents: Vec<Torrent>,
    movie_info: Vec<TorrentMovieInfo>,
}

pub struct Query;

#[graphql_object(context = ContextPointer)]
impl Query {
    async fn searchTorrents(
        #[graphql(context)] context: &ContextPointer,
        params: SearchHandlerParams,
    ) -> Result<SearchHandlerResponse, HttpErrorKind> {
        let ctx = context.lock().await;
        let torrents =
            search_handler(params, ctx.torrent_client(), ctx.movie_info_client()).await?;
        Ok(torrents)
    }

    async fn activeTorrents(
        #[graphql(context)] context: &ContextPointer,
        params: GetTorrentsParameters,
    ) -> Result<ActiveTorrentsResponse, HttpErrorKind> {
        let torrents = context
            .lock()
            .await
            .qbittorrent_client()
            .torrents(params)
            .await?;

        let mut torrent_movie_info: HashMap<u32, TorrentMovieInfo> = HashMap::new();

        let tmdb_ids: Vec<i32> = torrents
            .iter()
            .filter_map(|torrent| get_tmdb(torrent.name()).as_ref().map(|tmdb| *tmdb as i32))
            .collect();

        let movie_info = context
            .lock()
            .await
            .movie_info_client()
            .bulk(&tmdb_ids)
            .await?;

        movie_info.iter().for_each(|info| {
            let torrents = torrents.iter().filter_map(|torrent| {
                get_tmdb(torrent.name()).as_ref().and_then(|tmdb| {
                    if info.tmdb_id().eq(&(*tmdb as i32)) {
                        Some(torrent)
                    } else {
                        None
                    }
                })
            });

            torrents.for_each(|torrent| {
                if let Some(tmdb) = get_tmdb(torrent.name()) {
                    if let Some(info) = torrent_movie_info.get_mut(&tmdb) {
                        info.for_torrents.push(torrent.hash().clone());
                    } else {
                        torrent_movie_info.insert(
                            tmdb,
                            TorrentMovieInfo {
                                title: info.title().to_owned(),
                                year: info.year().to_owned(),
                                imdb: info.imdb_id().as_ref().map(|s| s.to_owned()),
                                tmdb: info.tmdb_id().to_owned(),
                                runtime: info.runtime().to_owned(),
                                for_torrents: vec![torrent.hash().clone()],
                            },
                        );
                    }
                }
            })
        });

        Ok(ActiveTorrentsResponse {
            torrents,
            movie_info: torrent_movie_info.into_values().collect(),
        })
    }

    async fn movie_info(
        #[graphql(context)] context: &ContextPointer,
        tmdb: i32,
    ) -> Result<Option<MovieInfo>, HttpErrorKind> {
        if tmdb.is_negative() {
            return Err(HttpErrorKind::InvalidParam("tmdb".into()));
        };

        let movie_info = context
            .lock()
            .await
            .movie_info_client()
            .from_tmdb(tmdb as u32)
            .await?;

        Ok(movie_info)
    }

    async fn search_movies(
        #[graphql(context)] context: &ContextPointer,
        query: String,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = context.lock().await;

        let movie_info = ctx
            .movie_info_client()
            .search(query, &ctx.config().filters())
            .await?;

        Ok(movie_info)
    }

    async fn tmdb_bulk(
        #[graphql(context)] context: &ContextPointer,
        tmdb_ids: Vec<i32>,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let movie_info = context
            .lock()
            .await
            .movie_info_client()
            .bulk(&tmdb_ids)
            .await?;

        Ok(movie_info)
    }

    async fn popular_movies(
        #[graphql(context)] context: &ContextPointer,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = context.lock().await;
        let movie_info = ctx
            .movie_info_client()
            .popular(ctx.config().filters())
            .await?;

        Ok(movie_info)
    }

    async fn trending_movies(
        #[graphql(context)] context: &ContextPointer,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = context.lock().await;
        let movie_info = ctx
            .movie_info_client()
            .trending(ctx.config().filters())
            .await?;

        Ok(movie_info)
    }

    fn search_filters() -> Vec<Filter> {
        vec![
            Filter::new(Quality::iter(), "Quality".into(), "quality".into()),
            Filter::new(Codec::iter(), "Codec".into(), "codec".into()),
            Filter::new(Source::iter(), "Source".into(), "source".into()),
        ]
    }
}

pub struct Mutation;
#[graphql_object(context = ContextPointer)]
impl Mutation {
    async fn add_torrent(
        #[graphql(context)] context: &ContextPointer,
        url: String,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        context
            .lock()
            .await
            .qbittorrent_client()
            .add_torrent(url, options.unwrap_or_default().into())
            .await?;
        Ok("Ok".into())
    }

    async fn add_torrents(
        #[graphql(context)] context: &ContextPointer,
        urls: Vec<String>,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        context
            .lock()
            .await
            .qbittorrent_client()
            .add_torrents(&urls, options.unwrap_or_default().into())
            .await?;
        Ok("Ok".into())
    }

    async fn track_movie(
        #[graphql(context)] context: &ContextPointer,
        url: String,
        tmdb: i32,
    ) -> Result<String, HttpErrorKind> {
        if tmdb.is_negative() {
            return Err(HttpErrorKind::InvalidParam("tmdb".into()));
        };

        track_movie(context, url, tmdb as u32).await?;
        Ok("Ok".into())
    }

    async fn delete_torrents(
        #[graphql(context)] context: &ContextPointer,
        hashes: Vec<String>,
        delete_files: bool,
    ) -> Result<String, HttpErrorKind> {
        context
            .lock()
            .await
            .qbittorrent_client()
            .delete_torrents(hashes, delete_files)
            .await?;

        Ok("Ok".into())
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<ContextPointer>>;

#[rocket::get("/")]
pub fn graphiql() -> RawHtml<String> {
    graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    context: &State<ContextPointer>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    context: &State<ContextPointer>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}
