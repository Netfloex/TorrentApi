use std::collections::HashSet;

use async_graphql::InputObject;
use getset::Getters;
use torrent_search_client::{Category, Codec, Order, Provider, Quality, SortColumn, Source};

#[derive(InputObject, Getters, Debug)]
#[get = "pub"]
pub struct SearchTorrentsParameters {
    query: Option<String>,
    imdb: Option<String>,

    #[graphql(default)]
    category: Category,
    #[graphql(default)]
    sort: SortColumn,
    #[graphql(default)]
    order: Order,
    #[graphql(default)]
    limit: usize,

    #[graphql(default)]
    quality: Vec<Quality>,
    #[graphql(default)]
    codec: Vec<Codec>,
    #[graphql(default)]
    source: Vec<Source>,

    #[graphql(default)]
    providers: HashSet<Provider>,
}
