use crate::models::filter::Filter;
use async_graphql::Object;
use strum::IntoEnumIterator;
use torrent_search_client::{Codec, Provider, Quality, Source};

#[derive(Default)]
pub struct SearchFiltersQuery;

#[Object]
impl SearchFiltersQuery {
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
