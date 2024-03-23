use crate::serialize_hashes::SerializeHashes;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
#[serde(rename_all = "camelCase")]
pub struct DeleteTorrentsParameters {
    delete_files: bool,
    #[serde(serialize_with = "Vec::serialize_hashes")]
    hashes: Vec<String>,
}

impl DeleteTorrentsParameters {
    pub fn new(hashes: Vec<String>, delete_files: bool) -> Self {
        Self {
            delete_files,
            hashes,
        }
    }
}
