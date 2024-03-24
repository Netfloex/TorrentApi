use crate::{
    add_torrent_options::ApiAddTorrentOptions,
    context::ContextPointer,
    filter::Filter,
    http_error::HttpErrorKind,
    search_handler::{search_handler, SearchHandlerParams, SearchHandlerResponse},
    utils::{get_tmdb::get_tmdb, track_movie::track_movie},
};
use async_graphql::{
    http::GraphiQLSource, ComplexObject, EmptyMutation, EmptySubscription, Schema, SimpleObject,
};
use async_graphql::{Context, Object};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use movie_info::{MovieInfo, TmdbId};
use qbittorrent_api::{GetTorrentsParameters, Torrent};
use rocket::{
    response::content::{self},
    State,
};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use tokio::sync::MutexGuard;
use torrent_search_client::{Codec, Provider, Quality, Source};
#[derive(PartialEq, Eq, Hash, SimpleObject)]
struct TorrentMovieInfo {
    title: String,
    year: u16,
    imdb: Option<String>,
    tmdb: TmdbId,
    runtime: u16,

    for_torrents: Vec<String>,
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct ActiveTorrentsResponse {
    torrents: Vec<Torrent>,
}

#[ComplexObject]
impl ActiveTorrentsResponse {
    async fn movie_info<'ctx>(
        &self,
        context: &Context<'ctx>,
    ) -> Result<Vec<TorrentMovieInfo>, HttpErrorKind> {
        let mut torrent_movie_info: HashMap<TmdbId, TorrentMovieInfo> = HashMap::new();

        let tmdb_ids: HashSet<TmdbId> = self
            .torrents
            .iter()
            .filter_map(|torrent| get_tmdb(torrent.get_name()))
            .collect();

        let movie_info = get_context(context)
            .await
            .movie_info_client()
            .bulk(&tmdb_ids)
            .await?;

        movie_info.iter().for_each(|info| {
            let torrents = self.torrents.iter().filter_map(|torrent| {
                get_tmdb(torrent.get_name()).as_ref().and_then(|tmdb| {
                    if info.get_tmdb_id().eq(tmdb) {
                        Some(torrent)
                    } else {
                        None
                    }
                })
            });

            torrents.for_each(|torrent| {
                if let Some(tmdb) = get_tmdb(torrent.get_name()) {
                    if let Some(info) = torrent_movie_info.get_mut(&tmdb) {
                        info.for_torrents.push(torrent.get_hash().clone());
                    } else {
                        torrent_movie_info.insert(
                            tmdb,
                            TorrentMovieInfo {
                                title: info.get_title().to_owned(),
                                year: info.get_year().to_owned(),
                                imdb: info.get_imdb_id().as_ref().map(|s| s.to_owned()),
                                tmdb: info.get_tmdb_id().to_owned(),
                                runtime: info.get_runtime().to_owned(),
                                for_torrents: vec![torrent.get_hash().clone()],
                            },
                        );
                    }
                }
            })
        });

        Ok(torrent_movie_info.into_values().collect())
    }
}

pub type SchemaType = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

async fn get_context<'ctx>(context: &Context<'ctx>) -> MutexGuard<'ctx, crate::Context> {
    context.data::<ContextPointer>().unwrap().lock().await
}

#[Object]
impl Query {
    async fn search_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        params: SearchHandlerParams,
    ) -> Result<SearchHandlerResponse, HttpErrorKind> {
        let ctx = get_context(context).await;
        let torrents =
            search_handler(params, ctx.torrent_client(), ctx.movie_info_client()).await?;
        Ok(torrents)
    }

    async fn active_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        #[graphql(default)] params: GetTorrentsParameters,
    ) -> Result<ActiveTorrentsResponse, HttpErrorKind> {
        let ctx = get_context(context).await;
        let torrents = ctx.qbittorrent_client().torrents(&params).await?;

        Ok(ActiveTorrentsResponse { torrents })
    }

    async fn movie_info<'ctx>(
        &self,
        context: &Context<'ctx>,
        tmdb: TmdbId,
    ) -> Result<Option<MovieInfo>, HttpErrorKind> {
        let movie_info = get_context(context)
            .await
            .movie_info_client()
            .from_tmdb(tmdb)
            .await?;

        Ok(movie_info)
    }

    async fn search_movies<'ctx>(
        &self,
        context: &Context<'ctx>,
        query: String,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = get_context(context).await;

        let movie_info = ctx
            .movie_info_client()
            .search(query, &ctx.config().filters())
            .await?;

        Ok(movie_info)
    }

    async fn tmdb_bulk<'ctx>(
        &self,
        context: &Context<'ctx>,
        tmdb_ids: HashSet<TmdbId>,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let movie_info = get_context(context)
            .await
            .movie_info_client()
            .bulk(&tmdb_ids)
            .await?;

        Ok(movie_info)
    }

    async fn popular_movies<'ctx>(
        &self,
        context: &Context<'ctx>,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = get_context(context).await;
        let movie_info = ctx
            .movie_info_client()
            .popular(ctx.config().filters())
            .await?;

        Ok(movie_info)
    }

    async fn trending_movies<'ctx>(
        &self,
        context: &Context<'ctx>,
    ) -> Result<Vec<MovieInfo>, HttpErrorKind> {
        let ctx = get_context(context).await;
        let movie_info = ctx
            .movie_info_client()
            .trending(ctx.config().filters())
            .await?;

        Ok(movie_info)
    }

    async fn search_filters(&self) -> Vec<Filter> {
        vec![
            Filter::new(
                Quality::iter(),
                "Quality".into(),
                "quality".into(),
                "Quality".into(),
            ),
            Filter::new(
                Codec::iter(),
                "Codec".into(),
                "codec".into(),
                "Codec".into(),
            ),
            Filter::new(
                Source::iter(),
                "Source".into(),
                "source".into(),
                "Source".into(),
            ),
            Filter::new(
                Provider::iter(),
                "Providers".into(),
                "providers".into(),
                "Provider".into(),
            ),
        ]
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_torrent<'ctx>(
        &self,
        context: &Context<'ctx>,
        url: String,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        get_context(context)
            .await
            .qbittorrent_client()
            .add_torrent(url, options.unwrap_or_default().into())
            .await?;
        Ok("Ok".into())
    }

    async fn add_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        urls: Vec<String>,
        options: Option<ApiAddTorrentOptions>,
    ) -> Result<String, HttpErrorKind> {
        get_context(context)
            .await
            .qbittorrent_client()
            .add_torrents(&urls, options.unwrap_or_default().into())
            .await?;
        Ok("Ok".into())
    }

    async fn track_movie<'ctx>(
        &self,
        context: &Context<'ctx>,
        url: String,
        tmdb: TmdbId,
    ) -> Result<String, HttpErrorKind> {
        track_movie(context.data::<ContextPointer>().unwrap(), url, tmdb).await?;
        Ok("Ok".into())
    }

    async fn delete_torrents<'ctx>(
        &self,
        context: &Context<'ctx>,
        hashes: Vec<String>,
        delete_files: bool,
    ) -> Result<String, HttpErrorKind> {
        get_context(context)
            .await
            .qbittorrent_client()
            .delete_torrents(hashes, delete_files)
            .await?;

        Ok("Ok".into())
    }
}

#[rocket::get("/")]
pub fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<SchemaType>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<SchemaType>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}
