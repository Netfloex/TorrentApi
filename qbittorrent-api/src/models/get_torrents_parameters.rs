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
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_hashes"
    )]
    hashes: Option<Vec<String>>,
}
