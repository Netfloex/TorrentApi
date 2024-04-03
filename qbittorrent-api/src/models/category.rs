use getset::Getters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[get = "pub"]
pub struct Category {
    name: String,
    #[serde(rename = "savePath")]
    save_path: String,
}
