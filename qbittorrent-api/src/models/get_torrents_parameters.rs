use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

fn serialize_hashes<S>(hashes: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match hashes {
        None => serializer.serialize_none(),
        Some(hashes) => hashes.join("|").serialize(serializer),
    }
}

#[derive(Serialize, Deserialize, Debug, Setters, Default)]
#[setters(strip_option = true)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct GetTorrentsParameters {
    filter: Option<String>,
    category: Option<String>,
    tag: Option<String>,
    sort: Option<String>,
    reverse: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
    #[serde(serialize_with = "serialize_hashes")]
    hashes: Option<Vec<String>>,
}
