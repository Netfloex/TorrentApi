use serde::Serialize;

#[derive(Serialize)]
pub struct AddCategoryOptions {
    category: String,
    #[serde(rename = "savePath")]
    save_path: String,
}

impl AddCategoryOptions {
    pub fn new(name: String, save_path: String) -> Self {
        Self {
            category: name,
            save_path,
        }
    }
}
