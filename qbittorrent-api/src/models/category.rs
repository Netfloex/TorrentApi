use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
pub struct Category {
    name: String,
    #[serde(rename = "savePath")]
    save_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Categories {
    #[serde(flatten)]
    categories: HashMap<String, Category>,
}

impl Categories {
    pub fn categories(self) -> Vec<Category> {
        self.categories.into_values().collect()
    }
}
