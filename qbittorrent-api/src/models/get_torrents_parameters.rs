use crate::serialize_hashes::SerializeHashes;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Setters, Default)]
#[setters(strip_option = true)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct GetTorrentsParameters {
    filter: Option<String>,
    category: Option<String>,
    tag: Option<String>,
    sort: Option<String>,
    reverse: Option<bool>,
    limit: Option<u8>,
    offset: Option<i8>,
    #[serde(serialize_with = "Option::serialize_hashes")]
    hashes: Option<Vec<String>>,
}
