use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SetCategoryOptions {
    hashes: String,
    category: String,
}

impl SetCategoryOptions {
    pub fn new(hashes: String, category: String) -> Self {
        Self { hashes, category }
    }
}
